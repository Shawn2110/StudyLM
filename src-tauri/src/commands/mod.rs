//! `#[tauri::command]` handlers invoked from React. Each submodule owns one
//! feature's command surface (notebooks, documents, chat, generation, provider
//! config). Handlers delegate to `db`, `ingestion`, `generation`, etc.—they
//! never contain business logic themselves.

pub mod notebook;
