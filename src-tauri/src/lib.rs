pub mod commands;
pub mod db;
pub mod embeddings;
pub mod error;
pub mod events;
pub mod generation;
pub mod ingestion;
pub mod keychain;
pub mod llm;
pub mod prompts;
pub mod retrieval;

use tauri::Manager;
use tracing_subscriber::EnvFilter;

/// Single source of truth for the command surface. Both the runtime
/// (`run()`) and the bindings generator (`bin/generate-bindings.rs`) call
/// this so the TS types and the actual handlers can never drift.
pub fn commands_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
        commands::notebook::create_notebook,
        commands::notebook::list_notebooks,
        commands::document::list_documents,
    ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,studylm_lib=debug".into()),
        )
        .init();

    let builder = commands_builder();

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            let app_data_dir = app.path().app_data_dir()?;
            let db_path = app_data_dir.join("studylm.db");
            let pool = tauri::async_runtime::block_on(db::init_pool(&db_path))?;
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
