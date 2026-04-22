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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,studylm_lib=debug".into()),
        )
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let db_path = app_data_dir.join("studylm.db");
            let pool = tauri::async_runtime::block_on(db::init_pool(&db_path))?;
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
