//! All SQLite access. Owns the `sqlx::SqlitePool`, runs migrations on startup,
//! and exposes typed query functions (`create_notebook`, `list_chunks`, …).
//! Commands call these functions; SQL never appears outside this module.

pub mod chunks;
pub mod documents;
pub mod models;
pub mod notebooks;

use std::path::Path;
use std::sync::Once;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::AppResult;

static REGISTER_VEC: Once = Once::new();

/// Register the sqlite-vec extension as a SQLite auto-extension. Once
/// registered, every `sqlite3_open` call (including those sqlx makes when
/// building the pool) gets `vec0` virtual tables for free. Called once per
/// process.
fn register_sqlite_vec_auto_extension() {
    REGISTER_VEC.call_once(|| {
        // SAFETY: sqlite3_vec_init is the official entrypoint shipped by the
        // sqlite-vec crate; sqlite3_auto_extension consumes it per the
        // SQLite C API. libsqlite3-sys's bindgen output types the
        // entrypoint with the three-arg extension signature, so the cast
        // happens through a matching function-pointer type.
        type EntryPoint = unsafe extern "C" fn(
            *mut libsqlite3_sys::sqlite3,
            *mut *mut std::os::raw::c_char,
            *const libsqlite3_sys::sqlite3_api_routines,
        ) -> std::os::raw::c_int;

        unsafe {
            let entry: EntryPoint =
                std::mem::transmute(sqlite_vec::sqlite3_vec_init as *const ());
            libsqlite3_sys::sqlite3_auto_extension(Some(entry));
        }
    });
}

/// Open (or create) the SQLite file at `db_path`, run every pending migration
/// under `src-tauri/migrations/`, and return the connection pool. Registers
/// the sqlite-vec extension on first call so vec0 tables are available to
/// every connection.
pub async fn init_pool(db_path: &Path) -> AppResult<SqlitePool> {
    register_sqlite_vec_auto_extension();

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

    // chunk_vec lives outside the migration files because the sqlite-vec
    // extension is only loaded by *our* binary (registered above). sqlx-cli
    // can't see vec0, so it ships separately and is created idempotently.
    sqlx::query(
        "CREATE VIRTUAL TABLE IF NOT EXISTS chunk_vec USING vec0(\
            chunk_id INTEGER PRIMARY KEY, \
            embedding float[768]\
        )",
    )
    .execute(&pool)
    .await?;

    tracing::info!(path = %db_path.display(), "sqlite ready");
    Ok(pool)
}
