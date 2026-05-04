//! `LlmProvider` trait plus per-provider implementations (Anthropic, OpenAI,
//! Google, OpenRouter, Ollama). Every LLM call in StudyLM goes through this
//! trait — HTTP, retries, capability introspection live here and nowhere
//! else.

pub mod anthropic;
pub mod google;
pub mod ollama;
pub mod openai;
pub mod openrouter;
pub mod types;

pub use types::{
    Capabilities, ChatChunk, ChatMessage, ChatRequest, ChatRole, ModelInfo, ProviderId,
    ProviderInfo, ProviderStatus,
};

use std::pin::Pin;

use async_trait::async_trait;
use futures_util::Stream;

use crate::error::AppResult;

pub type ChatStream = Pin<Box<dyn Stream<Item = ChatChunk> + Send>>;

/// Common surface for every provider.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    fn capabilities(&self) -> Capabilities;

    /// Probe the provider with the cheapest authenticated call available
    /// (typically `GET /models`). Returns the available model list on
    /// success.
    async fn ping(&self) -> AppResult<ProviderStatus>;

    /// Stream an assistant reply for the given chat request. The default
    /// impl returns a single `Error` chunk so providers without streaming
    /// support surface the right UX without panicking.
    async fn chat_stream(&self, _req: ChatRequest) -> AppResult<ChatStream> {
        let provider = self.id().label();
        let stream = async_stream::stream! {
            yield ChatChunk::Error {
                message: format!("{provider} chat is not implemented yet"),
            };
        };
        Ok(Box::pin(stream))
    }
}

/// Construct a provider client from its id and (optional) API key. Ollama
/// ignores the key and connects to its default base URL; cloud providers
/// use the key for auth.
pub fn build(id: ProviderId, api_key: Option<String>) -> Box<dyn LlmProvider> {
    let key = api_key.unwrap_or_default();
    match id {
        ProviderId::Anthropic => Box::new(anthropic::AnthropicProvider::new(key)),
        ProviderId::Openai => Box::new(openai::OpenAIProvider::new(key)),
        ProviderId::Google => Box::new(google::GoogleProvider::new(key)),
        ProviderId::Openrouter => Box::new(openrouter::OpenRouterProvider::new(key)),
        ProviderId::Ollama => Box::new(ollama::OllamaProvider::new(None)),
    }
}

/// All providers the app supports, in the order they should appear in the
/// Settings → Providers picker.
pub const ALL: [ProviderId; 5] = [
    ProviderId::Anthropic,
    ProviderId::Openai,
    ProviderId::Google,
    ProviderId::Openrouter,
    ProviderId::Ollama,
];
