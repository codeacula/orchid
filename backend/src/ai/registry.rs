//! Model registry for storing and listing available model configurations.

use serde::{Deserialize, Serialize};

/// Configuration for a single LLM model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_id: String,
    pub provider: String,
    pub api_base: String,
}

impl ModelConfig {
    pub fn new(
        model_id: impl Into<String>,
        provider: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Self {
        Self {
            model_id: model_id.into(),
            provider: provider.into(),
            api_base: api_base.into(),
        }
    }
}

/// Registry of available LLM model configurations.
pub struct ProviderRegistry {
    models: Vec<ModelConfig>,
}

impl ProviderRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self { models: Vec::new() }
    }

    /// Register a model configuration.
    pub fn register(&mut self, config: ModelConfig) {
        self.models.push(config);
    }

    /// List all registered model configurations.
    pub fn list_models(&self) -> Vec<&ModelConfig> {
        self.models.iter().collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
