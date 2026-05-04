//! Anthropic provider — pings `GET /v1/models` and streams chat completions
//! from `POST /v1/messages` (server-sent events).

use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::AppResult;
use crate::llm::types::{
    Capabilities, ChatChunk, ChatRequest, ChatRole, ModelInfo, ProviderId, ProviderStatus,
};
use crate::llm::{ChatStream, LlmProvider};

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

    async fn chat_stream(&self, req: ChatRequest) -> AppResult<ChatStream> {
        let body = AnthropicRequest::from(req);
        let response = self
            .client
            .post(format!("{BASE_URL}/v1/messages"))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Internal(format!("anthropic chat: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let stream = async_stream::stream! {
                yield ChatChunk::Error {
                    message: format!("HTTP {status}: {body}"),
                };
            };
            return Ok(Box::pin(stream));
        }

        let mut bytes = response.bytes_stream();
        let stream = async_stream::stream! {
            let mut buf = String::new();
            while let Some(next) = bytes.next().await {
                let chunk = match next {
                    Ok(c) => c,
                    Err(e) => {
                        yield ChatChunk::Error { message: e.to_string() };
                        return;
                    }
                };
                buf.push_str(&String::from_utf8_lossy(&chunk));

                // SSE events are separated by blank lines (\n\n). Process
                // every fully-received event and keep the trailing partial
                // for the next pass.
                while let Some(boundary) = buf.find("\n\n") {
                    let raw_event: String = buf.drain(..boundary + 2).collect();
                    if let Some(delta) = parse_sse_event(&raw_event) {
                        match delta {
                            ParsedEvent::Delta(text) => yield ChatChunk::Delta { text },
                            ParsedEvent::Stop => {
                                yield ChatChunk::Done;
                                return;
                            }
                            ParsedEvent::Skip => {}
                        }
                    }
                }
            }
            yield ChatChunk::Done;
        };

        Ok(Box::pin(stream))
    }
}

#[derive(Debug)]
enum ParsedEvent {
    Delta(String),
    Stop,
    Skip,
}

fn parse_sse_event(raw: &str) -> Option<ParsedEvent> {
    let mut event_name: Option<&str> = None;
    let mut data_payload: Option<&str> = None;
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("event: ") {
            event_name = Some(rest.trim());
        } else if let Some(rest) = line.strip_prefix("data: ") {
            data_payload = Some(rest.trim());
        }
    }
    let name = event_name?;
    let data = data_payload?;
    match name {
        "content_block_delta" => {
            #[derive(Deserialize)]
            struct Wrapper {
                delta: DeltaInner,
            }
            #[derive(Deserialize)]
            struct DeltaInner {
                #[serde(rename = "type")]
                kind: String,
                #[serde(default)]
                text: String,
            }
            let parsed: Wrapper = serde_json::from_str(data).ok()?;
            if parsed.delta.kind == "text_delta" {
                Some(ParsedEvent::Delta(parsed.delta.text))
            } else {
                Some(ParsedEvent::Skip)
            }
        }
        "message_stop" => Some(ParsedEvent::Stop),
        _ => Some(ParsedEvent::Skip),
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<serde_json::Value>,
}

impl From<ChatRequest> for AnthropicRequest {
    fn from(req: ChatRequest) -> Self {
        // Anthropic expects only user/assistant turns in `messages`; system
        // text lives at the top level. If a stray system message slipped
        // into req.messages, prepend it to req.system rather than dropping.
        let mut system = req.system.clone();
        let messages: Vec<AnthropicMessage> = req
            .messages
            .into_iter()
            .filter_map(|m| match m.role {
                ChatRole::System => {
                    system = Some(match system.take() {
                        Some(prev) => format!("{prev}\n\n{}", m.content),
                        None => m.content,
                    });
                    None
                }
                ChatRole::User => Some(AnthropicMessage {
                    role: "user".into(),
                    content: vec![json!({"type": "text", "text": m.content})],
                }),
                ChatRole::Assistant => Some(AnthropicMessage {
                    role: "assistant".into(),
                    content: vec![json!({"type": "text", "text": m.content})],
                }),
            })
            .collect();

        AnthropicRequest {
            model: req.model,
            max_tokens: req.max_tokens.unwrap_or(2048),
            stream: true,
            system,
            temperature: req.temperature,
            messages,
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
