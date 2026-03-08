//! LLM Backend trait and implementations for AI model routing.

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::time::Duration;
use thiserror::Error;

/// Represents a single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }
}

/// Error types returned by LLM operations.
#[derive(Debug, Error)]
pub enum LlmError {
    #[error("Model '{0}' is not registered with any provider")]
    UnknownModel(String),

    #[error("Request timed out")]
    Timeout,

    #[error("API error: {0}")]
    ApiError(String),
}

/// Stream type for LLM completions.
pub type LlmStream = Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>;

/// Trait for language model backends.
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// Request a completion from the model.
    /// Sends the full message history to the backend.
    async fn complete(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, LlmError>;

    /// Request a streaming completion from the model.
    /// Returns a stream that yields text chunks as they arrive.
    async fn complete_stream(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<LlmStream, LlmError>;
}

/// Request body for OpenAI-compatible API
#[derive(Debug, Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

/// Response for non-streaming completion
#[derive(Debug, Deserialize)]
struct CompletionResponse {
    choices: Vec<CompletionChoice>,
}

#[derive(Debug, Deserialize)]
struct CompletionChoice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

/// Streaming response chunk
#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: DeltaContent,
}

#[derive(Debug, Deserialize)]
struct DeltaContent {
    #[serde(default)]
    content: Option<String>,
}

/// OpenAI-compatible backend implementation.
/// Works with any API that implements the OpenAI chat completion interface.
pub struct OpenAiCompatBackend {
    api_base: String,
    api_key: String,
    timeout_secs: u64,
}

impl OpenAiCompatBackend {
    pub fn new(api_base: String, api_key: String, timeout_secs: u64) -> Self {
        Self {
            api_base,
            api_key,
            timeout_secs,
        }
    }
}

#[async_trait]
impl LlmBackend for OpenAiCompatBackend {
    async fn complete(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, LlmError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()
            .map_err(|e| LlmError::ApiError(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!("{}/v1/chat/completions", self.api_base);

        let request = CompletionRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await;

        match response {
            Err(e) if e.is_timeout() => Err(LlmError::Timeout),
            Err(e) => Err(LlmError::ApiError(format!("HTTP request failed: {}", e))),
            Ok(resp) => {
                let completion: CompletionResponse = resp
                    .json()
                    .await
                    .map_err(|e| LlmError::ApiError(format!("Failed to parse response: {}", e)))?;

                let content = completion
                    .choices
                    .first()
                    .ok_or_else(|| LlmError::ApiError("No choices in response".to_string()))?
                    .message
                    .content
                    .clone();

                Ok(content)
            }
        }
    }

    async fn complete_stream(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<LlmStream, LlmError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()
            .map_err(|e| LlmError::ApiError(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!("{}/v1/chat/completions", self.api_base);

        let request = CompletionRequest {
            model: model.to_string(),
            messages,
            stream: true,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await;

        match response {
            Err(e) if e.is_timeout() => Err(LlmError::Timeout),
            Err(e) => Err(LlmError::ApiError(format!("HTTP request failed: {}", e))),
            Ok(resp) => {
                let body = resp
                    .text()
                    .await
                    .map_err(|e| LlmError::ApiError(format!("Failed to read response: {}", e)))?;

                // Parse SSE format
                let chunks: Vec<Result<String, LlmError>> = body
                    .lines()
                    .filter(|line| !line.is_empty() && line.starts_with("data: "))
                    .filter_map(|line| {
                        let data = &line[6..]; // Remove "data: " prefix
                        if data == "[DONE]" {
                            return None;
                        }

                        match serde_json::from_str::<StreamChunk>(data) {
                            Ok(chunk) => {
                                if let Some(choice) = chunk.choices.first() {
                                    if let Some(content) = &choice.delta.content {
                                        if !content.is_empty() {
                                            return Some(Ok(content.clone()));
                                        }
                                    }
                                }
                                None
                            }
                            Err(e) => Some(Err(LlmError::ApiError(format!(
                                "Failed to parse stream chunk: {}",
                                e
                            )))),
                        }
                    })
                    .collect();

                let stream = futures::stream::iter(chunks);
                Ok(Box::pin(stream))
            }
        }
    }
}
