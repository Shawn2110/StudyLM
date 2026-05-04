//! Chat session persistence. A chat belongs to one notebook and groups
//! the back-and-forth user/assistant messages persisted in `db::messages`.

use sqlx::SqlitePool;

use crate::db::models::Chat;
use crate::error::AppResult;

/// Insert a fresh chat row. `title`, `model_id`, and `provider` are nullable
/// — they're populated as soon as the first assistant reply lands.
pub async fn create(
    pool: &SqlitePool,
    id: &str,
    notebook_id: &str,
) -> AppResult<Chat> {
    let now = now_unix();
    sqlx::query!(
        r#"INSERT INTO chat (id, notebook_id, created_at) VALUES (?, ?, ?)"#,
        id,
        notebook_id,
        now,
    )
    .execute(pool)
    .await?;
    get_by_id(pool, id).await
}

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<Chat> {
    let row = sqlx::query_as!(
        Chat,
        r#"SELECT id           as "id!: String",
                  notebook_id  as "notebook_id!: String",
                  title,
                  model_id,
                  provider,
                  created_at   as "created_at!: i64"
           FROM chat WHERE id = ?"#,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn list_by_notebook(
    pool: &SqlitePool,
    notebook_id: &str,
) -> AppResult<Vec<Chat>> {
    let rows = sqlx::query_as!(
        Chat,
        r#"SELECT id           as "id!: String",
                  notebook_id  as "notebook_id!: String",
                  title,
                  model_id,
                  provider,
                  created_at   as "created_at!: i64"
           FROM chat
           WHERE notebook_id = ?
           ORDER BY created_at DESC"#,
        notebook_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn set_meta(
    pool: &SqlitePool,
    id: &str,
    title: Option<&str>,
    model_id: &str,
    provider: &str,
) -> AppResult<()> {
    sqlx::query!(
        r#"UPDATE chat
           SET title = COALESCE(?, title),
               model_id = ?,
               provider = ?
           WHERE id = ?"#,
        title,
        model_id,
        provider,
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
