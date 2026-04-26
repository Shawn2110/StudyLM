//! Settings table — opaque key-value store. The keychain holds API keys;
//! the `settings` table holds the rest of the user's preferences (active
//! provider, theme, opt-in analytics, …).

use sqlx::SqlitePool;

use crate::error::AppResult;

pub const ACTIVE_PROVIDER: &str = "active_provider";

/// Read the value stored under `key`, or `None` if no row exists.
pub async fn get(pool: &SqlitePool, key: &str) -> AppResult<Option<String>> {
    let row = sqlx::query!(
        r#"SELECT value as "value!: String" FROM settings WHERE key = ?"#,
        key,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.value))
}

/// Insert or update the value for `key`.
pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> AppResult<()> {
    sqlx::query!(
        r#"INSERT INTO settings (key, value) VALUES (?, ?)
           ON CONFLICT(key) DO UPDATE SET value = excluded.value"#,
        key,
        value,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove `key` if present. Idempotent.
pub async fn delete(pool: &SqlitePool, key: &str) -> AppResult<()> {
    sqlx::query!("DELETE FROM settings WHERE key = ?", key)
        .execute(pool)
        .await?;
    Ok(())
}
