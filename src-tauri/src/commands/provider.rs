//! `#[tauri::command]` handlers for the LLM provider config flow:
//! list, validate (live ping with a candidate key), check stored status,
//! store/delete the keychain entry, and remember which provider is active.

use std::str::FromStr;

use sqlx::SqlitePool;
use tauri::State;

use crate::db::settings;
use crate::error::{AppError, AppResult};
use crate::keychain;
use crate::llm::{self, ProviderId, ProviderInfo, ProviderStatus};

#[tauri::command]
#[specta::specta]
pub async fn list_providers() -> AppResult<Vec<ProviderInfo>> {
    Ok(llm::ALL.iter().copied().map(ProviderInfo::from).collect())
}

/// Validate a candidate key against the live provider before persisting it.
/// Returns the same status enum used everywhere else so the UI can render
/// uniform feedback (Connected, InvalidKey, Unreachable, Error).
#[tauri::command]
#[specta::specta]
pub async fn validate_provider_key(
    provider: ProviderId,
    api_key: String,
) -> AppResult<ProviderStatus> {
    let client = llm::build(provider, Some(api_key));
    client.ping().await
}

/// Probe the provider with the key currently stored in the keychain.
/// Returns `NotConfigured` when no key is stored (and the provider needs
/// one); for Ollama it always pings since it has no key requirement.
#[tauri::command]
#[specta::specta]
pub async fn get_provider_status(provider: ProviderId) -> AppResult<ProviderStatus> {
    let key = keychain::get_key(provider.as_str()).await?;
    if provider.needs_api_key() && key.is_none() {
        return Ok(ProviderStatus::NotConfigured);
    }
    let client = llm::build(provider, key);
    client.ping().await
}

#[tauri::command]
#[specta::specta]
pub async fn store_provider_key(provider: ProviderId, api_key: String) -> AppResult<()> {
    keychain::store_key(provider.as_str(), &api_key).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_provider_key(provider: ProviderId) -> AppResult<()> {
    keychain::delete_key(provider.as_str()).await
}

#[tauri::command]
#[specta::specta]
pub async fn set_active_provider(
    pool: State<'_, SqlitePool>,
    provider: Option<ProviderId>,
) -> AppResult<()> {
    match provider {
        Some(p) => settings::set(pool.inner(), settings::ACTIVE_PROVIDER, p.as_str()).await,
        None => settings::delete(pool.inner(), settings::ACTIVE_PROVIDER).await,
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_active_provider(pool: State<'_, SqlitePool>) -> AppResult<Option<ProviderId>> {
    let raw = settings::get(pool.inner(), settings::ACTIVE_PROVIDER).await?;
    match raw {
        None => Ok(None),
        Some(s) => ProviderId::from_str(&s)
            .map(Some)
            .map_err(|_| AppError::InvalidInput(format!("unknown provider id: {s}"))),
    }
}

