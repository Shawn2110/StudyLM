//! End-to-end ingestion for a single PDF. Orchestrates copy → parse → chunk
//! → embed → store. Designed to run inside a Tokio task kicked off from a
//! `#[tauri::command]`; progresses by emitting `document-status` events each
//! time the document's row moves between states.

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter, Manager};
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::db::models::{DocumentStatus, SourceType};
use crate::db::{chunks, documents};
use crate::embeddings::{self, EmbedderSlot, NomicEmbedder};
use crate::error::{AppError, AppResult};
use crate::ingestion::{approximate_token_count, chunk_document, parse_pdf, ChunkOpts};

const EMBED_BATCH: usize = 32;

/// Event name React subscribes to via `listen("document-status", …)`.
pub const EVENT_DOCUMENT_STATUS: &str = "document-status";

#[derive(Clone, Serialize, Deserialize, specta::Type)]
pub struct DocumentStatusPayload {
    pub document_id: String,
    pub status: DocumentStatus,
    pub error: Option<String>,
}

/// Ingest a PDF end-to-end. Returns the newly created document's id as soon
/// as the row is inserted; the heavy work (parse/chunk/embed/store) runs in
/// a spawned task and streams status updates via events.
pub async fn ingest_pdf(
    pool: SqlitePool,
    embedder_slot: EmbedderSlot,
    app_handle: AppHandle,
    notebook_id: String,
    source_path: PathBuf,
) -> AppResult<String> {
    let document_id = Uuid::new_v4().to_string();
    let filename = source_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("document.pdf")
        .to_string();

    // Copy the PDF into the app-managed folder so the original source can
    // move or disappear without breaking reads.
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(format!("app_data_dir: {e}")))?;
    let target_dir = app_data_dir.join("notebooks").join(&notebook_id);
    tokio::fs::create_dir_all(&target_dir).await?;
    let local_path = target_dir.join(format!("{document_id}.pdf"));
    tokio::fs::copy(&source_path, &local_path).await?;

    let local_path_str = local_path.to_string_lossy().to_string();
    documents::create(
        &pool,
        &document_id,
        &notebook_id,
        &filename,
        SourceType::Pdf,
        &local_path_str,
    )
    .await?;
    emit_status(&app_handle, &document_id, DocumentStatus::Pending, None);

    // Spawn the heavy work. The command returns immediately with the id.
    let doc_id_bg = document_id.clone();
    tokio::spawn(async move {
        if let Err(e) =
            run(pool.clone(), embedder_slot, app_handle.clone(), doc_id_bg.clone(), local_path)
                .await
        {
            tracing::error!(document_id = %doc_id_bg, error = %e, "ingestion failed");
            let msg = e.to_string();
            let _ = documents::mark_failed(&pool, &doc_id_bg, &msg).await;
            emit_status(&app_handle, &doc_id_bg, DocumentStatus::Failed, Some(msg));
        }
    });

    Ok(document_id)
}

async fn run(
    pool: SqlitePool,
    embedder_slot: EmbedderSlot,
    app_handle: AppHandle,
    document_id: String,
    local_path: PathBuf,
) -> AppResult<()> {
    // ── parse ───────────────────────────────────────────────────────
    documents::set_status(&pool, &document_id, DocumentStatus::Parsing).await?;
    emit_status(&app_handle, &document_id, DocumentStatus::Parsing, None);

    let parse_path = local_path.clone();
    let pages = spawn_blocking(move || parse_pdf(&parse_path))
        .await
        .map_err(|e| AppError::Internal(format!("parse join: {e}")))??;

    let page_count = pages.len() as i64;
    documents::set_page_count(&pool, &document_id, page_count).await?;

    // ── chunk ───────────────────────────────────────────────────────
    let chunks_vec = chunk_document(&pages, &ChunkOpts::default(), approximate_token_count);
    if chunks_vec.is_empty() {
        return Err(AppError::Internal(
            "parser produced no text to embed".into(),
        ));
    }

    // ── embed + store ───────────────────────────────────────────────
    documents::set_status(&pool, &document_id, DocumentStatus::Embedding).await?;
    emit_status(&app_handle, &document_id, DocumentStatus::Embedding, None);

    let embedder: Arc<NomicEmbedder> = embeddings::get_or_init(&embedder_slot).await?;

    for window in chunks_vec.chunks(EMBED_BATCH) {
        let texts: Vec<String> = window.iter().map(|c| c.text.clone()).collect();
        let embedder_clone = Arc::clone(&embedder);
        let vectors = spawn_blocking(move || {
            let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
            embedder_clone.embed(&refs)
        })
        .await
        .map_err(|e| AppError::Internal(format!("embed join: {e}")))??;

        if vectors.len() != window.len() {
            return Err(AppError::Internal(format!(
                "embedder returned {} vectors for batch of {}",
                vectors.len(),
                window.len()
            )));
        }

        for (chunk, embedding) in window.iter().zip(vectors.iter()) {
            let chunk_id = chunks::insert_chunk(
                &pool,
                &document_id,
                chunk.page as i64,
                chunk.chunk_idx as i64,
                &chunk.text,
                chunk.token_count as i64,
            )
            .await?;
            chunks::insert_chunk_vec(&pool, chunk_id, embedding).await?;
        }
    }

    documents::set_status(&pool, &document_id, DocumentStatus::Ready).await?;
    emit_status(&app_handle, &document_id, DocumentStatus::Ready, None);
    tracing::info!(
        document_id = %document_id,
        chunks = chunks_vec.len(),
        pages = page_count,
        "document ready",
    );
    Ok(())
}

fn emit_status(
    app: &AppHandle,
    document_id: &str,
    status: DocumentStatus,
    error: Option<String>,
) {
    let payload = DocumentStatusPayload {
        document_id: document_id.to_string(),
        status,
        error,
    };
    if let Err(e) = app.emit(EVENT_DOCUMENT_STATUS, payload) {
        tracing::warn!(%e, "failed to emit document-status event");
    }
}
