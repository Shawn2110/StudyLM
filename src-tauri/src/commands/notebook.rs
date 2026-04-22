//! `#[tauri::command]` handlers for notebook CRUD. Thin wrappers around
//! `db::notebooks::*` — the State-managed `SqlitePool` is the only piece of
//! glue that lives at this layer.

use sqlx::SqlitePool;
use tauri::State;

use crate::db::models::{Notebook, PrepMode};
use crate::db::notebooks;
use crate::error::AppResult;

#[tauri::command]
#[specta::specta]
pub async fn create_notebook(
    pool: State<'_, SqlitePool>,
    prep_mode: PrepMode,
) -> AppResult<Notebook> {
    notebooks::create(pool.inner(), &prep_mode).await
}

#[tauri::command]
#[specta::specta]
pub async fn list_notebooks(pool: State<'_, SqlitePool>) -> AppResult<Vec<Notebook>> {
    notebooks::list(pool.inner()).await
}
