//! Document persistence. All SQL touching the `document` table lives here.
//! Chunk-level writes (chunk / chunk_fts / chunk_vec) are owned by the
//! ingestion pipeline under `src-tauri/src/ingestion/`.

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
