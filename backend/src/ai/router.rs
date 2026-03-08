//! Provider router for routing requests to the correct LLM backend.

use super::backend::{ChatMessage, LlmBackend, LlmError};
use std::collections::HashMap;

/// Routes completion requests to the appropriate backend based on model_id.
pub struct ProviderRouter {
    backends: HashMap<String, Box<dyn LlmBackend + Send + Sync>>,
}

impl ProviderRouter {
    /// Create a new empty router.
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    /// Register a backend for a specific model_id.
    pub fn register(
        &mut self,
        model_id: impl Into<String>,
        backend: Box<dyn LlmBackend + Send + Sync>,
    ) {
        self.backends.insert(model_id.into(), backend);
    }

    /// Request a completion from the appropriate backend.
    /// Returns an error if the model_id is not registered.
    pub async fn complete(
        &self,
        model_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<String, LlmError> {
        let backend = self
            .backends
            .get(model_id)
            .ok_or_else(|| LlmError::UnknownModel(model_id.to_string()))?;

        backend.complete(model_id, messages).await
    }

    /// Request a streaming completion from the appropriate backend.
    /// Returns an error if the model_id is not registered.
    pub async fn complete_stream(
        &self,
        model_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<super::backend::LlmStream, LlmError> {
        let backend = self
            .backends
            .get(model_id)
            .ok_or_else(|| LlmError::UnknownModel(model_id.to_string()))?;

        backend.complete_stream(model_id, messages).await
    }
}

impl Default for ProviderRouter {
    fn default() -> Self {
        Self::new()
    }
}
