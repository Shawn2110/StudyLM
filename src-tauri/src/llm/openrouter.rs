//! OpenRouter provider — pings `GET /api/v1/models`.

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::error::AppResult;
use crate::llm::types::{Capabilities, ModelInfo, ProviderId, ProviderStatus};
use crate::llm::LlmProvider;

const BASE_URL: &str = "https://openrouter.ai";

pub struct OpenRouterProvider {
    api_key: String,
    client: Client,
}

impl OpenRouterProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenRouterProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Openrouter
    }

    fn capabilities(&self) -> Capabilities {
        // Capabilities depend on the underlying model selected; return
        // conservative defaults and let the chat layer override per-request.
        Capabilities {
            streaming: true,
            native_tool_use: true,
            max_context: 128_000,
        }
    }

    async fn ping(&self) -> AppResult<ProviderStatus> {
        let url = format!("{BASE_URL}/api/v1/models");
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        let response = match response {
            Ok(r) => r,
            Err(e) => {
                return Ok(ProviderStatus::Unreachable {
                    message: e.to_string(),
                });
            }
        };

        match response.status() {
            StatusCode::OK => {
                let body: ModelsResponse = match response.json().await {
                    Ok(b) => b,
                    Err(e) => {
                        return Ok(ProviderStatus::Error {
                            message: e.to_string(),
                        });
                    }
                };
                Ok(ProviderStatus::Connected {
                    models: body.data.into_iter().map(Into::into).collect(),
                })
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Ok(ProviderStatus::InvalidKey {
                message: format!("HTTP {}", response.status()),
            }),
            other => Ok(ProviderStatus::Error {
                message: format!("HTTP {other}"),
            }),
        }
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
    name: Option<String>,
    context_length: Option<i64>,
}

impl From<ModelEntry> for ModelInfo {
    fn from(m: ModelEntry) -> Self {
        ModelInfo {
            label: m.name.unwrap_or_else(|| m.id.clone()),
            id: m.id,
            context_window: m.context_length,
        }
    }
}
