//! End-to-end chat-message orchestration.
//!
//! Take a user's question on an existing chat → resolve the active
//! provider + key → run hybrid retrieval against the notebook → build a
//! prep-mode-aware system prompt → stream the assistant reply, emitting
//! `chat-stream` events for each delta → persist the final assistant
//! message with parsed citations.

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::db::models::{
    Chat, Citation, MessageRole, Notebook, PrepMode,
};
use crate::db::{chats, messages, notebooks, settings};
use crate::embeddings::{self, EmbedderSlot};
use crate::error::{AppError, AppResult};
use crate::generation::citations;
use crate::keychain;
use crate::llm::{self, ChatChunk, ChatMessage, ChatRequest, ChatRole, ProviderId};
use crate::prompts;
use crate::retrieval::{self, RetrievalOpts};

pub const EVENT_CHAT_STREAM: &str = "chat-stream";

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChatStreamEvent {
    /// A streamed text fragment from the active LLM.
    Delta { chat_id: String, text: String },
    /// Stream finished. Carries the persisted assistant message id and
    /// parsed citation pills.
    Done {
        chat_id: String,
        message_id: String,
        citations: Vec<Citation>,
    },
    /// Stream aborted. The chat record is intact; no assistant message is
    /// persisted on error.
    Error { chat_id: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SendChatRequest {
    pub chat_id: String,
    pub user_text: String,
    pub model_id: String,
}

/// Spawn a chat turn. Returns the assistant-side message id immediately
/// after we know which row will receive the streamed text; the actual
/// streaming happens on a Tokio task.
pub async fn send_message(
    pool: SqlitePool,
    embedder_slot: EmbedderSlot,
    app_handle: AppHandle,
    req: SendChatRequest,
) -> AppResult<String> {
    let chat = chats::get_by_id(&pool, &req.chat_id).await?;
    let notebook = notebooks::get_by_id(&pool, &chat.notebook_id).await?;

    // Persist user message.
    let user_msg_id = Uuid::new_v4().to_string();
    messages::create(
        &pool,
        &user_msg_id,
        &chat.id,
        MessageRole::User,
        &req.user_text,
        None,
    )
    .await?;

    let assistant_msg_id = Uuid::new_v4().to_string();

    let pool_bg = pool.clone();
    let chat_id_bg = chat.id.clone();
    let assistant_id_bg = assistant_msg_id.clone();
    tokio::spawn(async move {
        if let Err(e) = run(
            pool_bg.clone(),
            embedder_slot,
            app_handle.clone(),
            chat,
            notebook,
            req,
            assistant_id_bg.clone(),
        )
        .await
        {
            tracing::error!(chat_id = %chat_id_bg, error = %e, "chat turn failed");
            emit(
                &app_handle,
                ChatStreamEvent::Error {
                    chat_id: chat_id_bg,
                    message: e.to_string(),
                },
            );
        }
    });

    Ok(assistant_msg_id)
}

async fn run(
    pool: SqlitePool,
    embedder_slot: EmbedderSlot,
    app_handle: AppHandle,
    chat: Chat,
    notebook: Notebook,
    req: SendChatRequest,
    assistant_msg_id: String,
) -> AppResult<()> {
    // 1. Active provider + key.
    let provider = active_provider(&pool).await?;
    let key = keychain::get_key(provider.as_str()).await?;
    if provider.needs_api_key() && key.is_none() {
        return Err(AppError::InvalidInput(format!(
            "no api key configured for {}",
            provider.label()
        )));
    }

    // 2. Hybrid retrieval.
    let embedder = embeddings::get_or_init(&embedder_slot).await?;
    let sources = retrieval::retrieve(
        &pool,
        embedder,
        &notebook.id,
        &req.user_text,
        RetrievalOpts::default(),
    )
    .await?;
    let sources_block = retrieval::format_sources(&sources);

    // 3. System prompt + message history.
    let prep = PrepMode {
        exam_type: notebook.exam_type,
        format: notebook.format,
        subject: notebook.subject.clone(),
        duration_minutes: notebook.duration_minutes,
        exam_at: notebook.exam_at,
        difficulty_focus: notebook.difficulty_focus,
    };
    let system = prompts::build_chat_system_prompt(&prep, &sources_block)?;

    let history = messages::list_by_chat(&pool, &chat.id).await?;
    let llm_messages: Vec<ChatMessage> = history
        .iter()
        .filter(|m| matches!(m.role, MessageRole::User | MessageRole::Assistant))
        .map(|m| ChatMessage {
            role: match m.role {
                MessageRole::User => ChatRole::User,
                MessageRole::Assistant => ChatRole::Assistant,
            },
            content: m.content.clone(),
        })
        .collect();

    // 4. Stream.
    let client = llm::build(provider, key);
    let chat_request = ChatRequest {
        model: req.model_id.clone(),
        system: Some(system),
        messages: llm_messages,
        max_tokens: Some(2048),
        temperature: None,
    };
    let mut stream = client.chat_stream(chat_request).await?;
    let mut accumulated = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            ChatChunk::Delta { text } => {
                accumulated.push_str(&text);
                emit(
                    &app_handle,
                    ChatStreamEvent::Delta {
                        chat_id: chat.id.clone(),
                        text,
                    },
                );
            }
            ChatChunk::Done => break,
            ChatChunk::Error { message } => {
                return Err(AppError::Internal(format!("llm stream: {message}")));
            }
        }
    }

    // 5. Persist assistant message + citations.
    let cites = citations::collect(&accumulated, &sources);
    let cites_json = if cites.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&cites).map_err(|e| AppError::Internal(e.to_string()))?)
    };
    messages::create(
        &pool,
        &assistant_msg_id,
        &chat.id,
        MessageRole::Assistant,
        &accumulated,
        cites_json.as_deref(),
    )
    .await?;

    // First reply also seeds chat metadata (model + provider + title).
    let title = chat.title.as_deref().or_else(|| Some(short_title(&accumulated)));
    chats::set_meta(&pool, &chat.id, title, &req.model_id, provider.as_str()).await?;

    emit(
        &app_handle,
        ChatStreamEvent::Done {
            chat_id: chat.id,
            message_id: assistant_msg_id,
            citations: cites,
        },
    );

    Ok(())
}

async fn active_provider(pool: &SqlitePool) -> AppResult<ProviderId> {
    let raw = settings::get(pool, settings::ACTIVE_PROVIDER).await?;
    let id = raw.ok_or_else(|| {
        AppError::InvalidInput("no active provider — configure one in Settings".into())
    })?;
    use std::str::FromStr;
    ProviderId::from_str(&id)
        .map_err(|_| AppError::InvalidInput(format!("unknown active provider: {id}")))
}

fn emit(app: &AppHandle, ev: ChatStreamEvent) {
    if let Err(e) = app.emit(EVENT_CHAT_STREAM, ev) {
        tracing::warn!(%e, "failed to emit chat-stream event");
    }
}

/// Cheap chat-title heuristic until summarization is wired up: take the
/// first line of the assistant's reply, capped at ~60 chars.
fn short_title(text: &str) -> &str {
    let line = text.lines().next().unwrap_or(text).trim();
    if line.len() <= 60 {
        line
    } else {
        &line[..60]
    }
}
