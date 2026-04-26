//! Shared types for the LLM provider layer. Kept in their own module so
//! `commands/`, `db/`, and `llm/<provider>.rs` can all reference them
//! without pulling in the trait surface.

use serde::{Deserialize, Serialize};
use specta::Type;

/// The five providers StudyLM can target. The serialized identifier is
/// snake_case, matching the rest of the IPC surface and the keychain keys.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum ProviderId {
    Anthropic,
    Openai,
    Google,
    Openrouter,
    Ollama,
}

impl ProviderId {
    /// Stable string used as both the keychain entry name and the value
    /// stored in the `settings` table.
    pub fn as_str(self) -> &'static str {
        match self {
            ProviderId::Anthropic => "anthropic",
            ProviderId::Openai => "openai",
            ProviderId::Google => "google",
            ProviderId::Openrouter => "openrouter",
            ProviderId::Ollama => "ollama",
        }
    }

    /// Pretty label for UI ("Anthropic", "OpenAI", …).
    pub fn label(self) -> &'static str {
        match self {
            ProviderId::Anthropic => "Anthropic",
            ProviderId::Openai => "OpenAI",
            ProviderId::Google => "Google",
            ProviderId::Openrouter => "OpenRouter",
            ProviderId::Ollama => "Ollama",
        }
    }

    /// `true` when the provider authenticates via an API key. `false` for
    /// providers reached without auth (Ollama on localhost).
    pub fn needs_api_key(self) -> bool {
        !matches!(self, ProviderId::Ollama)
    }
}

/// What `ping()` reports back to the UI. Status colors and copy in the
/// Settings → Providers screen are driven off this.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "kind", content = "detail")]
pub enum ProviderStatus {
    /// No key configured (or for Ollama, the daemon was not probed yet).
    NotConfigured,
    /// Provider responded successfully to a low-cost ping.
    Connected { models: Vec<ModelInfo> },
    /// Network reached the provider but it rejected our credentials.
    InvalidKey { message: String },
    /// We could not reach the provider at all (network down, Ollama not
    /// running, DNS failure, etc.).
    Unreachable { message: String },
    /// Anything else that doesn't fit the above (5xx, malformed payload).
    Error { message: String },
}

/// A single model exposed by a provider. `id` is the provider-native
/// identifier passed back into chat requests; `label` is the friendly name
/// for UI; `context_window` is in tokens when known.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ModelInfo {
    pub id: String,
    pub label: String,
    pub context_window: Option<i64>,
}

/// Provider capability flags consumed by the generation layer (Phase 4+)
/// to decide between native tool-use and JSON-prompting fallback. Stubbed
/// in Phase 1 with conservative defaults.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub struct Capabilities {
    pub streaming: bool,
    pub native_tool_use: bool,
    pub max_context: i64,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            streaming: true,
            native_tool_use: false,
            max_context: 8_000,
        }
    }
}

/// Provider summary for the Settings → Providers list. Exists so the
/// frontend can render the picker without instantiating provider clients.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProviderInfo {
    pub id: ProviderId,
    pub label: String,
    pub needs_api_key: bool,
}

impl From<ProviderId> for ProviderInfo {
    fn from(id: ProviderId) -> Self {
        Self {
            id,
            label: id.label().to_string(),
            needs_api_key: id.needs_api_key(),
        }
    }
}
