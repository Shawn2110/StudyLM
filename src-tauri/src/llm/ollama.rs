//! Ollama provider — pings `GET /api/tags` against the local daemon.
//! No auth in the typical setup; the optional `base_url` lets a future
//! Settings field point at a remote Ollama on the same LAN.

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::error::AppResult;
use crate::llm::types::{Capabilities, ModelInfo, ProviderId, ProviderStatus};
use crate::llm::LlmProvider;

pub const DEFAULT_BASE_URL: &str = "http://localhost:11434";

pub struct OllamaProvider {
    base_url: String,
    client: Client,
}

impl OllamaProvider {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Ollama
    }

    fn capabilities(&self) -> Capabilities {
        // Tool-use varies by underlying model and is unreliable on small
        // local models — JSON-prompting fallback is the conservative default.
        Capabilities {
            streaming: true,
            native_tool_use: false,
            max_context: 8_000,
        }
    }

    async fn ping(&self) -> AppResult<ProviderStatus> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send().await;

        let response = match response {
            Ok(r) => r,
            Err(e) => {
                return Ok(ProviderStatus::Unreachable {
                    message: format!("ollama not reachable at {}: {e}", self.base_url),
                });
            }
        };

        match response.status() {
            StatusCode::OK => {
                let body: TagsResponse = match response.json().await {
                    Ok(b) => b,
                    Err(e) => {
                        return Ok(ProviderStatus::Error {
                            message: e.to_string(),
                        });
                    }
                };
                Ok(ProviderStatus::Connected {
                    models: body.models.into_iter().map(Into::into).collect(),
                })
            }
            other => Ok(ProviderStatus::Error {
                message: format!("HTTP {other}"),
            }),
        }
    }
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    name: String,
    details: Option<ModelDetails>,
}

#[derive(Deserialize)]
struct ModelDetails {
    parameter_size: Option<String>,
}

impl From<ModelEntry> for ModelInfo {
    fn from(m: ModelEntry) -> Self {
        let label = match &m.details {
            Some(d) => match &d.parameter_size {
                Some(p) => format!("{} ({})", m.name, p),
                None => m.name.clone(),
            },
            None => m.name.clone(),
        };
        ModelInfo {
            label,
            id: m.name,
            context_window: None,
        }
    }
}
