//! All SQLite access. Owns the `sqlx::SqlitePool`, runs migrations on startup,
//! and exposes typed query functions (`create_notebook`, `list_chunks`, …).
//! Commands call these functions; SQL never appears outside this module.

use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::AppResult;

/// Open (or create) the SQLite file at `db_path`, run every pending migration
/// under `src-tauri/migrations/`, and return the connection pool.
pub async fn init_pool(db_path: &Path) -> AppResult<SqlitePool> {
    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!(path = %db_path.display(), "sqlite ready");
    Ok(pool)
}
