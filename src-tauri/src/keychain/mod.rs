//! Thin async wrapper over `keyring-rs` that stores API keys in the native
//! OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret
//! Service). Key values are never logged and never cross the Rust ↔ React
//! boundary.

use keyring::Entry;
use tokio::task::spawn_blocking;

use crate::error::{AppError, AppResult};

const TARGET: &str = "com.studylm.app";

/// Store `key` in the native keychain under the logical `service`
/// (e.g. "anthropic", "openai", "ollama_base_url"). Overwrites any
/// existing value.
pub async fn store_key(service: &str, key: &str) -> AppResult<()> {
    let service = service.to_owned();
    let key = key.to_owned();
    spawn_blocking(move || -> AppResult<()> {
        let entry = Entry::new(TARGET, &service)?;
        entry.set_password(&key)?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("keychain join: {e}")))?
}

/// Read the key for `service`, or `None` if no entry exists.
pub async fn get_key(service: &str) -> AppResult<Option<String>> {
    let service = service.to_owned();
    spawn_blocking(move || -> AppResult<Option<String>> {
        let entry = Entry::new(TARGET, &service)?;
        match entry.get_password() {
            Ok(pw) => Ok(Some(pw)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AppError::from(e)),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("keychain join: {e}")))?
}

/// Idempotent delete — missing entries are not an error.
pub async fn delete_key(service: &str) -> AppResult<()> {
    let service = service.to_owned();
    spawn_blocking(move || -> AppResult<()> {
        let entry = Entry::new(TARGET, &service)?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AppError::from(e)),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("keychain join: {e}")))?
}
