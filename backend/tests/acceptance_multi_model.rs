//! Acceptance Tests: Multi-Model AI Routing
//!
//! These tests verify the LlmBackend trait, ProviderRouter, and model selection logic.
//! They use wiremock to simulate OpenAI-compatible API servers.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

#[tokio::test]
async fn provider_router_routes_to_correct_backend_by_model_id() {
    // Verifies: Given a ProviderRouter with two backends registered
    // ("ollama/llama3" and "openai/gpt-4"), requesting a completion with
    // model_id "ollama/llama3" routes to the Ollama backend, and
    // "openai/gpt-4" routes to the OpenAI backend.
    todo!("not yet implemented");
}

#[tokio::test]
async fn provider_router_returns_error_for_unknown_model() {
    // Verifies: Requesting a completion with an unregistered model_id
    // returns a clear error (not a panic).
    todo!("not yet implemented");
}

#[tokio::test]
async fn llm_backend_streams_completion_via_openai_compatible_api() {
    // Verifies: An OpenAiCompatBackend configured with a wiremock server
    // as its base_url successfully streams a chat completion response.
    // The stream yields multiple chunks and terminates correctly.
    todo!("not yet implemented");
}

#[tokio::test]
async fn llm_backend_sends_full_message_history_in_request() {
    // Verifies: When requesting a completion, the backend sends the full
    // conversation message history (system prompt + all user/assistant messages)
    // to the API, not just the latest message.
    todo!("not yet implemented");
}

#[tokio::test]
async fn llm_backend_handles_api_timeout_gracefully() {
    // Verifies: When the AI API server doesn't respond within the timeout,
    // the backend returns a timeout error rather than hanging indefinitely.
    todo!("not yet implemented");
}

#[tokio::test]
async fn model_list_reflects_provider_registry_configuration() {
    // Verifies: The ProviderRegistry correctly loads model configurations
    // and exposes them for the API to list available models.
    todo!("not yet implemented");
}
