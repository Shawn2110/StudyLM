//! `#[tauri::command]` handlers for document CRUD and ingestion. Thin
//! wrappers around `db::documents::*` and (once Phase 2.4 lands) the
//! ingestion pipeline. The State-managed `SqlitePool` is the only glue at
//! this layer.

use sqlx::SqlitePool;
use tauri::State;

use crate::db::documents;
use crate::db::models::Document;
use crate::error::AppResult;

#[tauri::command]
#[specta::specta]
pub async fn list_documents(
    pool: State<'_, SqlitePool>,
    notebook_id: String,
) -> AppResult<Vec<Document>> {
    documents::list_by_notebook(pool.inner(), &notebook_id).await
}
