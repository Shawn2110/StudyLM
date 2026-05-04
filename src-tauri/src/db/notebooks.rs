//! Notebook persistence. All SQL touching the `notebook` table lives here.

use sqlx::SqlitePool;

use crate::db::models::{DifficultyFocus, ExamType, Format, Notebook, PrepMode};
use crate::error::AppResult;

/// Insert a new notebook from a `PrepMode` snapshot, returning the persisted
/// row (with generated `id`, `title`, `created_at`).
pub async fn create(pool: &SqlitePool, prep_mode: &PrepMode) -> AppResult<Notebook> {
    let id = uuid::Uuid::new_v4().to_string();
    let title = auto_title(prep_mode);
    let created_at = unix_now();

    sqlx::query!(
        r#"INSERT INTO notebook
           (id, title, created_at, exam_type, format, subject,
            duration_minutes, exam_at, difficulty_focus)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        id,
        title,
        created_at,
        prep_mode.exam_type,
        prep_mode.format,
        prep_mode.subject,
        prep_mode.duration_minutes,
        prep_mode.exam_at,
        prep_mode.difficulty_focus,
    )
    .execute(pool)
    .await?;

    Ok(Notebook {
        id,
        title,
        created_at,
        exam_type: prep_mode.exam_type,
        format: prep_mode.format,
        subject: prep_mode.subject.clone(),
        duration_minutes: prep_mode.duration_minutes,
        exam_at: prep_mode.exam_at,
        difficulty_focus: prep_mode.difficulty_focus,
    })
}

/// Look up a single notebook by id.
pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<Notebook> {
    let row = sqlx::query_as!(
        Notebook,
        r#"SELECT id              as "id!: String",
                  title           as "title!: String",
                  created_at      as "created_at!: i64",
                  exam_type       as "exam_type!: ExamType",
                  format          as "format!: Format",
                  subject,
                  duration_minutes,
                  exam_at,
                  difficulty_focus as "difficulty_focus: DifficultyFocus"
           FROM notebook WHERE id = ?"#,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// Return all notebooks, newest first.
pub async fn list(pool: &SqlitePool) -> AppResult<Vec<Notebook>> {
    let rows = sqlx::query_as!(
        Notebook,
        r#"SELECT id              as "id!: String",
                  title           as "title!: String",
                  created_at      as "created_at!: i64",
                  exam_type       as "exam_type!: ExamType",
                  format          as "format!: Format",
                  subject,
                  duration_minutes,
                  exam_at,
                  difficulty_focus as "difficulty_focus: DifficultyFocus"
           FROM notebook
           ORDER BY created_at DESC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

fn auto_title(prep_mode: &PrepMode) -> String {
    let subject = prep_mode
        .subject
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let Some(s) = subject {
        return s.to_string();
    }
    format!("{:?} · {:?}", prep_mode.exam_type, prep_mode.format)
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
