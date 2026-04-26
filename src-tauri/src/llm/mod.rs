//! `LlmProvider` trait plus per-provider implementations (Anthropic, OpenAI,
//! Google, OpenRouter, Ollama). Every LLM call in StudyLM goes through this
//! trait — HTTP, retries, capability introspection live here and nowhere
//! else.

pub mod types;

pub use types::{Capabilities, ModelInfo, ProviderId, ProviderInfo, ProviderStatus};

use async_trait::async_trait;

use crate::error::AppResult;

/// Common surface for every provider. Phase 1 only needs `ping` (which
/// doubles as `list_models`); chat streaming and tool use land in later
/// phases behind the same trait.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    fn capabilities(&self) -> Capabilities;

    /// Probe the provider with the cheapest authenticated call available
    /// (typically `GET /models`). Returns the available model list on
    /// success.
    async fn ping(&self) -> AppResult<ProviderStatus>;
}
