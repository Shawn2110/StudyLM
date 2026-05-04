//! Chat message persistence. Both the user's question and the assistant's
//! reply land here; citations on assistant messages are stored as a JSON
//! array of `Citation` values (see `db::models`).

use sqlx::SqlitePool;

use crate::db::models::{Message, MessageRole};
use crate::error::AppResult;

pub async fn create(
    pool: &SqlitePool,
    id: &str,
    chat_id: &str,
    role: MessageRole,
    content: &str,
    citations_json: Option<&str>,
) -> AppResult<Message> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    sqlx::query!(
        r#"INSERT INTO message (id, chat_id, role, content, citations_json, created_at)
           VALUES (?, ?, ?, ?, ?, ?)"#,
        id,
        chat_id,
        role,
        content,
        citations_json,
        now,
    )
    .execute(pool)
    .await?;
    get_by_id(pool, id).await
}

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<Message> {
    let row = sqlx::query_as!(
        Message,
        r#"SELECT id              as "id!: String",
                  chat_id         as "chat_id!: String",
                  role            as "role!: MessageRole",
                  content         as "content!: String",
                  citations_json,
                  created_at      as "created_at!: i64"
           FROM message WHERE id = ?"#,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn list_by_chat(pool: &SqlitePool, chat_id: &str) -> AppResult<Vec<Message>> {
    let rows = sqlx::query_as!(
        Message,
        r#"SELECT id              as "id!: String",
                  chat_id         as "chat_id!: String",
                  role            as "role!: MessageRole",
                  content         as "content!: String",
                  citations_json,
                  created_at      as "created_at!: i64"
           FROM message
           WHERE chat_id = ?
           ORDER BY created_at ASC"#,
        chat_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
