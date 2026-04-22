//! Rust structs that mirror rows in the SQLite schema. Each model derives
//! `serde` for IPC, `specta::Type` for TypeScript generation, and
//! `sqlx::Type` / `sqlx::FromRow` for database round-tripping.

use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::FromRow;

/// Which exam the notebook is preparing for. Drives prompt selection across
/// every generation feature (chat, study guide, flashcards, podcast).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum ExamType {
    Internal,
    Midsem,
    Endsem,
    Viva,
    Practical,
    Assignment,
    Competitive,
    Custom,
}

/// Question/answer shape the student will face in the exam.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum Format {
    Mcq,
    Short,
    Long,
    Oral,
    Numerical,
    Mixed,
}

/// Emphasis for flashcards, guide tone, and worked-example selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum DifficultyFocus {
    Conceptual,
    ProblemSolving,
    Memorization,
    Mixed,
}

/// All six prep-mode fields supplied when a notebook is created. Snapshotted
/// onto every generated artifact so regenerations stay consistent with the
/// student's original intent.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct PrepMode {
    /// Which exam the student is preparing for.
    pub exam_type: ExamType,
    /// Question format of that exam.
    pub format: Format,
    /// Free-text subject label (e.g. "Thermodynamics", "Constitutional Law").
    pub subject: Option<String>,
    /// Minutes the exam itself will take.
    pub duration_minutes: Option<i64>,
    /// Unix epoch seconds of when the exam starts; null means unspecified.
    pub exam_at: Option<i64>,
    /// Tone and emphasis of generated output.
    pub difficulty_focus: Option<DifficultyFocus>,
}

/// A persisted notebook row. `id` is a uuid v4 and `created_at` is unix
/// epoch seconds.
#[derive(Debug, Clone, Serialize, Deserialize, Type, FromRow)]
pub struct Notebook {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub exam_type: ExamType,
    pub format: Format,
    pub subject: Option<String>,
    pub duration_minutes: Option<i64>,
    pub exam_at: Option<i64>,
    pub difficulty_focus: Option<DifficultyFocus>,
}
