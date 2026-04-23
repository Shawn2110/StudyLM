//! Chunk persistence. `chunk` rows are written via compile-time-checked
//! queries; `chunk_vec` rows are written via dynamic queries because the
//! `vec0` virtual table from the sqlite-vec extension is not visible to
//! sqlx-cli at `cargo sqlx prepare` time (only to our runtime binary where
//! the extension is registered as an auto-extension).

use sqlx::SqlitePool;

use crate::error::AppResult;

/// Insert a chunk into the `chunk` table and return its auto-assigned rowid.
/// The FTS5 mirror is kept in sync automatically by the `chunk_ai` trigger
/// defined in migration `0002_documents.sql`.
pub async fn insert_chunk(
    pool: &SqlitePool,
    document_id: &str,
    page: i64,
    chunk_idx: i64,
    text: &str,
    token_count: i64,
) -> AppResult<i64> {
    let row = sqlx::query!(
        r#"INSERT INTO chunk (document_id, page, chunk_idx, text, token_count)
           VALUES (?, ?, ?, ?, ?)
           RETURNING id as "id!: i64""#,
        document_id,
        page,
        chunk_idx,
        text,
        token_count,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

/// Insert a 768-d float32 embedding into the `chunk_vec` virtual table.
/// `embedding` must be exactly 768 entries long.
pub async fn insert_chunk_vec(
    pool: &SqlitePool,
    chunk_id: i64,
    embedding: &[f32],
) -> AppResult<()> {
    let bytes: &[u8] = bytemuck::cast_slice(embedding);
    sqlx::query("INSERT INTO chunk_vec(chunk_id, embedding) VALUES (?, ?)")
        .bind(chunk_id)
        .bind(bytes)
        .execute(pool)
        .await?;
    Ok(())
}
