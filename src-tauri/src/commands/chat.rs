//! `#[tauri::command]` handlers for chat (Phase 3).
//! Read-side: list chats per notebook, list messages per chat.
//! Write-side: create a chat, send a message (kicks off streaming).

use sqlx::SqlitePool;
use tauri::{AppHandle, State};
use uuid::Uuid;

use crate::db::models::{Chat, Message};
use crate::db::{chats, messages};
use crate::embeddings::EmbedderSlot;
use crate::error::AppResult;
use crate::generation;

#[tauri::command]
#[specta::specta]
pub async fn list_chats(
    pool: State<'_, SqlitePool>,
    notebook_id: String,
) -> AppResult<Vec<Chat>> {
    chats::list_by_notebook(pool.inner(), &notebook_id).await
}

#[tauri::command]
#[specta::specta]
pub async fn create_chat(
    pool: State<'_, SqlitePool>,
    notebook_id: String,
) -> AppResult<Chat> {
    let id = Uuid::new_v4().to_string();
    chats::create(pool.inner(), &id, &notebook_id).await
}

#[tauri::command]
#[specta::specta]
pub async fn list_messages(
    pool: State<'_, SqlitePool>,
    chat_id: String,
) -> AppResult<Vec<Message>> {
    messages::list_by_chat(pool.inner(), &chat_id).await
}

/// Send a user message and kick off the assistant reply. The command
/// returns the assistant message id immediately; deltas + the final
/// message arrive on the `chat-stream` event.
#[tauri::command]
#[specta::specta]
pub async fn send_chat_message(
    pool: State<'_, SqlitePool>,
    embedder: State<'_, EmbedderSlot>,
    app_handle: AppHandle,
    chat_id: String,
    user_text: String,
    model_id: String,
) -> AppResult<String> {
    generation::send_chat_message(
        pool.inner().clone(),
        embedder.inner().clone(),
        app_handle,
        generation::SendChatRequest {
            chat_id,
            user_text,
            model_id,
        },
    )
    .await
}
