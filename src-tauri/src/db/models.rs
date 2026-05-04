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

/// Where a document originally came from. MVP ships `pdf`; the other variants
/// are reserved for Phase 1 / P2 follow-ups (URL import, markdown paste, raw
/// text drop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum SourceType {
    Pdf,
    Url,
    Md,
    Text,
}

/// Lifecycle of a document inside the ingestion pipeline. Rendered verbatim
/// in the UI as a parse-state badge (see docs/design.md §6.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum DocumentStatus {
    Pending,
    Parsing,
    Embedding,
    Ready,
    Failed,
}

/// A single source attached to a notebook (PDF, URL, markdown, raw text).
/// `local_path` is always populated — even URL/text sources materialise a
/// file under the app-managed folder so downstream parsing is uniform.
#[derive(Debug, Clone, Serialize, Deserialize, Type, FromRow)]
pub struct Document {
    pub id: String,
    pub notebook_id: String,
    pub filename: String,
    pub source_type: SourceType,
    pub source_url: Option<String>,
    pub local_path: String,
    pub page_count: Option<i64>,
    pub status: DocumentStatus,
    pub error: Option<String>,
    pub created_at: i64,
}

/// Author of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
}

/// A persisted chat session inside a notebook. `model_id` and `provider`
/// snapshot which LLM was used for the first assistant reply — useful for
/// later regeneration and for badge rendering on the message bubble.
#[derive(Debug, Clone, Serialize, Deserialize, Type, FromRow)]
pub struct Chat {
    pub id: String,
    pub notebook_id: String,
    pub title: Option<String>,
    pub model_id: Option<String>,
    pub provider: Option<String>,
    pub created_at: i64,
}

/// A persisted chat message. `citations_json` holds a serialized
/// `Vec<Citation>` (see `Citation` below) on assistant messages; user
/// messages leave it null.
#[derive(Debug, Clone, Serialize, Deserialize, Type, FromRow)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub role: MessageRole,
    pub content: String,
    pub citations_json: Option<String>,
    pub created_at: i64,
}

/// Inline citation pointing back at a chunk + its source document. Rendered
/// as a clickable pill in chat answers.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Citation {
    pub chunk_id: i64,
    pub document_id: String,
    pub document_filename: String,
    pub page: i64,
}
