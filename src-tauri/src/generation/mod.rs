//! Feature-level generation layered on retrieval + LLM provider: chat
//! (Phase 3, streaming + cited), study guide (Phase 4, map-reduce over
//! sections), flashcards (Phase 4, prep-mode-specific schema), podcast
//! script (Phase 5). Each entry point takes a `PrepMode` snapshot and
//! routes to the matching template under `prompts/`.

pub mod chat;
pub mod citations;

pub use chat::{
    send_message as send_chat_message, ChatStreamEvent, SendChatRequest, EVENT_CHAT_STREAM,
};
