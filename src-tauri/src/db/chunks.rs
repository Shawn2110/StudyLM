//! Chunk persistence + retrieval queries. `chunk` rows are written via
//! compile-time-checked queries; `chunk_vec` queries (insert and
//! similarity search) are dynamic because the `vec0` virtual table from
//! the sqlite-vec extension is not visible to sqlx-cli at
//! `cargo sqlx prepare` time — only to our runtime binary where the
//! extension is registered as an auto-extension.

use sqlx::{FromRow, SqlitePool};

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

/// Dense KNN search over the `chunk_vec` virtual table for chunks belonging
/// to `notebook_id`. Returns chunk ids in ascending distance order.
pub async fn dense_search(
    pool: &SqlitePool,
    notebook_id: &str,
    query_embedding: &[f32],
    k: i64,
) -> AppResult<Vec<i64>> {
    let bytes: &[u8] = bytemuck::cast_slice(query_embedding);
    let rows: Vec<(i64,)> = sqlx::query_as(
        r#"SELECT v.chunk_id
           FROM chunk_vec v
           JOIN chunk c ON c.id = v.chunk_id
           JOIN document d ON d.id = c.document_id
           WHERE v.embedding MATCH ? AND v.k = ? AND d.notebook_id = ?
           ORDER BY v.distance"#,
    )
    .bind(bytes)
    .bind(k)
    .bind(notebook_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// BM25 search over the FTS5 mirror for chunks belonging to `notebook_id`.
/// Returns chunk ids in best-match-first order.
pub async fn fts_search(
    pool: &SqlitePool,
    notebook_id: &str,
    query: &str,
    k: i64,
) -> AppResult<Vec<i64>> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        r#"SELECT c.id
           FROM chunk_fts f
           JOIN chunk c ON c.id = f.rowid
           JOIN document d ON d.id = c.document_id
           WHERE chunk_fts MATCH ? AND d.notebook_id = ?
           ORDER BY bm25(chunk_fts)
           LIMIT ?"#,
    )
    .bind(query)
    .bind(notebook_id)
    .bind(k)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Joined chunk + source-document metadata, returned by `fetch_chunks`.
#[derive(Debug, Clone, FromRow)]
pub struct ChunkWithDocument {
    pub id: i64,
    pub text: String,
    pub page: i64,
    pub document_id: String,
    pub document_filename: String,
}

/// Hydrate chunk rows by id, joining each with its parent document so
/// citations carry filename + page out of the box. Order of the returned
/// vec follows `chunk_ids` so callers can preserve their ranking.
pub async fn fetch_chunks(
    pool: &SqlitePool,
    chunk_ids: &[i64],
) -> AppResult<Vec<ChunkWithDocument>> {
    if chunk_ids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; chunk_ids.len()].join(",");
    let sql = format!(
        r#"SELECT c.id            AS id,
                  c.text          AS text,
                  c.page          AS page,
                  c.document_id   AS document_id,
                  d.filename      AS document_filename
           FROM chunk c
           JOIN document d ON d.id = c.document_id
           WHERE c.id IN ({placeholders})"#
    );
    let mut q = sqlx::query_as::<_, ChunkWithDocument>(&sql);
    for id in chunk_ids {
        q = q.bind(id);
    }
    let mut rows = q.fetch_all(pool).await?;

    // Preserve caller-provided ordering.
    let order: std::collections::HashMap<i64, usize> = chunk_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();
    rows.sort_by_key(|r| *order.get(&r.id).unwrap_or(&usize::MAX));
    Ok(rows)
}
