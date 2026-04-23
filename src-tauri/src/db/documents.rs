//! Document persistence. All SQL touching the `document` table lives here.
//! Chunk-level writes (chunk / chunk_fts / chunk_vec) are owned by
//! `src-tauri/src/db/chunks.rs`.

use sqlx::SqlitePool;

use crate::db::models::{Document, DocumentStatus, SourceType};
use crate::error::AppResult;

/// List every document attached to a notebook, newest first. Returns an empty
/// vec when the notebook has none.
pub async fn list_by_notebook(
    pool: &SqlitePool,
    notebook_id: &str,
) -> AppResult<Vec<Document>> {
    let rows = sqlx::query_as!(
        Document,
        r#"SELECT id                as "id!: String",
                  notebook_id       as "notebook_id!: String",
                  filename          as "filename!: String",
                  source_type       as "source_type!: SourceType",
                  source_url,
                  local_path        as "local_path!: String",
                  page_count,
                  status            as "status!: DocumentStatus",
                  error,
                  created_at        as "created_at!: i64"
           FROM document
           WHERE notebook_id = ?
           ORDER BY created_at DESC"#,
        notebook_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Insert a new document row in the `pending` state. Callers update the
/// status as the ingestion pipeline progresses.
pub async fn create(
    pool: &SqlitePool,
    id: &str,
    notebook_id: &str,
    filename: &str,
    source_type: SourceType,
    local_path: &str,
) -> AppResult<()> {
    let pending = DocumentStatus::Pending;
    let created_at = now_unix();
    sqlx::query!(
        r#"INSERT INTO document
           (id, notebook_id, filename, source_type, local_path, status, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        id,
        notebook_id,
        filename,
        source_type,
        local_path,
        pending,
        created_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_status(
    pool: &SqlitePool,
    id: &str,
    status: DocumentStatus,
) -> AppResult<()> {
    sqlx::query!(
        "UPDATE document SET status = ? WHERE id = ?",
        status,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_page_count(pool: &SqlitePool, id: &str, page_count: i64) -> AppResult<()> {
    sqlx::query!(
        "UPDATE document SET page_count = ? WHERE id = ?",
        page_count,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_failed(pool: &SqlitePool, id: &str, error: &str) -> AppResult<()> {
    let failed = DocumentStatus::Failed;
    sqlx::query!(
        "UPDATE document SET status = ?, error = ? WHERE id = ?",
        failed,
        error,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
