//! AI model routing and LLM backend integration.

pub mod backend;
pub mod registry;
pub mod router;

pub use backend::{ChatMessage, LlmBackend, LlmError, LlmStream, OpenAiCompatBackend};
pub use registry::{ModelConfig, ProviderRegistry};
pub use router::ProviderRouter;
