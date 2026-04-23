//! `#[tauri::command]` handlers for document CRUD and ingestion. Thin
//! wrappers around `db::documents::*` and the ingestion pipeline under
//! `src-tauri/src/ingestion/`. The State-managed `SqlitePool` and the
//! embedder slot are the only glue at this layer.

use std::path::PathBuf;

use sqlx::SqlitePool;
use tauri::{AppHandle, State};

use crate::db::documents;
use crate::db::models::Document;
use crate::embeddings::EmbedderSlot;
use crate::error::AppResult;
use crate::ingestion;

#[tauri::command]
#[specta::specta]
pub async fn list_documents(
    pool: State<'_, SqlitePool>,
    notebook_id: String,
) -> AppResult<Vec<Document>> {
    documents::list_by_notebook(pool.inner(), &notebook_id).await
}

/// Kick off ingestion for `path` into `notebook_id`. Returns the new document
/// id immediately; the frontend listens on `document-status` events for
/// progress and terminal state.
#[tauri::command]
#[specta::specta]
pub async fn ingest_document(
    pool: State<'_, SqlitePool>,
    embedder: State<'_, EmbedderSlot>,
    app_handle: AppHandle,
    notebook_id: String,
    path: String,
) -> AppResult<String> {
    ingestion::ingest_pdf(
        pool.inner().clone(),
        embedder.inner().clone(),
        app_handle,
        notebook_id,
        PathBuf::from(path),
    )
    .await
}
