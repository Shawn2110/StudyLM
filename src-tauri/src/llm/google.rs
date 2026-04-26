//! Google Gemini provider — pings `GET /v1beta/models?key=…`.

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::error::AppResult;
use crate::llm::types::{Capabilities, ModelInfo, ProviderId, ProviderStatus};
use crate::llm::LlmProvider;

const BASE_URL: &str = "https://generativelanguage.googleapis.com";

pub struct GoogleProvider {
    api_key: String,
    client: Client,
}

impl GoogleProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for GoogleProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Google
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            streaming: true,
            native_tool_use: true,
            max_context: 1_000_000,
        }
    }

    async fn ping(&self) -> AppResult<ProviderStatus> {
        let url = format!("{BASE_URL}/v1beta/models");
        let response = self
            .client
            .get(&url)
            .query(&[("key", &self.api_key)])
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
                let models = body
                    .models
                    .into_iter()
                    .filter(|m| {
                        m.supported_generation_methods
                            .as_ref()
                            .is_none_or(|methods| methods.iter().any(|s| s == "generateContent"))
                    })
                    .map(Into::into)
                    .collect();
                Ok(ProviderStatus::Connected { models })
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN | StatusCode::BAD_REQUEST => {
                Ok(ProviderStatus::InvalidKey {
                    message: format!("HTTP {}", response.status()),
                })
            }
            other => Ok(ProviderStatus::Error {
                message: format!("HTTP {other}"),
            }),
        }
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "inputTokenLimit")]
    input_token_limit: Option<i64>,
    #[serde(rename = "supportedGenerationMethods")]
    supported_generation_methods: Option<Vec<String>>,
}

impl From<ModelEntry> for ModelInfo {
    fn from(m: ModelEntry) -> Self {
        // `name` comes through as "models/gemini-2.0-flash" — strip the prefix
        // for the chat-side model id.
        let id = m
            .name
            .strip_prefix("models/")
            .unwrap_or(&m.name)
            .to_string();
        ModelInfo {
            label: m.display_name.unwrap_or_else(|| id.clone()),
            id,
            context_window: m.input_token_limit,
        }
    }
}
