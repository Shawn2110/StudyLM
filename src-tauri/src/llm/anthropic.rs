//! Anthropic provider — pings `GET /v1/models` and returns the model list.

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::error::AppResult;
use crate::llm::types::{Capabilities, ModelInfo, ProviderId, ProviderStatus};
use crate::llm::LlmProvider;

const BASE_URL: &str = "https://api.anthropic.com";
const API_VERSION: &str = "2023-06-01";

pub struct AnthropicProvider {
    api_key: String,
    client: Client,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Anthropic
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            streaming: true,
            native_tool_use: true,
            max_context: 200_000,
        }
    }

    async fn ping(&self) -> AppResult<ProviderStatus> {
        let url = format!("{BASE_URL}/v1/models?limit=1000");
        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .send()
            .await;

        let response = match response {
            Ok(r) => r,
            Err(e) => return Ok(unreachable_status(e)),
        };

        match response.status() {
            StatusCode::OK => {
                let body: ModelsResponse = match response.json().await {
                    Ok(b) => b,
                    Err(e) => return Ok(error_status(e)),
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
    display_name: Option<String>,
}

impl From<ModelEntry> for ModelInfo {
    fn from(m: ModelEntry) -> Self {
        ModelInfo {
            label: m.display_name.unwrap_or_else(|| m.id.clone()),
            id: m.id,
            context_window: None,
        }
    }
}

fn unreachable_status(e: reqwest::Error) -> ProviderStatus {
    ProviderStatus::Unreachable {
        message: e.to_string(),
    }
}

fn error_status(e: reqwest::Error) -> ProviderStatus {
    ProviderStatus::Error {
        message: e.to_string(),
    }
}
