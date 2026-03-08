//! Acceptance Tests: Multi-Model AI Routing
//!
//! These tests verify the LlmBackend trait, ProviderRouter, and model selection logic.
//! They use wiremock to simulate OpenAI-compatible API servers.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

use futures::StreamExt;
use orchid::ai::{ChatMessage, LlmError, OpenAiCompatBackend, ProviderRouter, ProviderRegistry, ModelConfig, LlmBackend};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn provider_router_routes_to_correct_backend_by_model_id() {
    // Verifies: Given a ProviderRouter with two backends registered
    // ("ollama/llama3" and "openai/gpt-4"), requesting a completion with
    // model_id "ollama/llama3" routes to the Ollama backend, and
    // "openai/gpt-4" routes to the OpenAI backend.

    // Set up two mock servers
    let ollama_server = MockServer::start().await;
    let openai_server = MockServer::start().await;

    // Mount completion response on ollama server
    let ollama_response = ResponseTemplate::new(200)
        .set_body_json(serde_json::json!({
            "id": "chatcmpl-ollama",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "ollama/llama3",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Response from Ollama"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }));
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ollama_response)
        .mount(&ollama_server)
        .await;

    // Mount completion response on openai server
    let openai_response = ResponseTemplate::new(200)
        .set_body_json(serde_json::json!({
            "id": "chatcmpl-openai",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "openai/gpt-4",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Response from OpenAI"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }));
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(openai_response)
        .mount(&openai_server)
        .await;

    // Create backends pointing to respective servers
    let ollama_backend = Box::new(OpenAiCompatBackend::new(
        ollama_server.uri(),
        "test-key".to_string(),
        10,
    ));
    let openai_backend = Box::new(OpenAiCompatBackend::new(
        openai_server.uri(),
        "test-key".to_string(),
        10,
    ));

    // Create router and register backends
    let mut router = ProviderRouter::new();
    router.register("ollama/llama3", ollama_backend);
    router.register("openai/gpt-4", openai_backend);

    // Test messages
    let messages = vec![ChatMessage::new("user", "Hello")];

    // Request completion for ollama model
    let ollama_result = router.complete("ollama/llama3", messages.clone()).await;
    assert!(ollama_result.is_ok());
    assert!(ollama_result.unwrap().contains("Ollama"));

    // Request completion for openai model
    let openai_result = router.complete("openai/gpt-4", messages.clone()).await;
    assert!(openai_result.is_ok());
    assert!(openai_result.unwrap().contains("OpenAI"));
}

#[tokio::test]
async fn provider_router_returns_error_for_unknown_model() {
    // Verifies: Requesting a completion with an unregistered model_id
    // returns a clear error (not a panic).

    let router = ProviderRouter::new();
    let messages = vec![ChatMessage::new("user", "Hello")];

    let result = router.complete("unknown/model", messages).await;
    assert!(result.is_err());

    if let Err(LlmError::UnknownModel(model_id)) = result {
        assert_eq!(model_id, "unknown/model");
    } else {
        panic!("Expected UnknownModel error");
    }
}

#[tokio::test]
async fn llm_backend_streams_completion_via_openai_compatible_api() {
    // Verifies: An OpenAiCompatBackend configured with a wiremock server
    // as its base_url successfully streams a chat completion response.
    // The stream yields multiple chunks and terminates correctly.

    let server = MockServer::start().await;

    // Create a streaming response with multiple chunks
    let streaming_response = "data: {\"id\":\"chatcmpl-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"test\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"finish_reason\":null}]}\n\ndata: {\"id\":\"chatcmpl-1\",\"object\":\"chat.completion.chunk\",\"created\":1234567890,\"model\":\"test\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" world\"},\"finish_reason\":null}]}\n\ndata: [DONE]\n\n";

    let response = ResponseTemplate::new(200)
        .set_header("content-type", "text/event-stream")
        .set_body_string(streaming_response);

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(response)
        .mount(&server)
        .await;

    let backend = OpenAiCompatBackend::new(server.uri(), "test-key".to_string(), 10);
    let messages = vec![ChatMessage::new("user", "Hello")];

    let result = backend.complete_stream("test-model", messages).await;
    assert!(result.is_ok());

    let mut stream = result.unwrap();
    let mut chunks = Vec::new();

    while let Some(chunk_result) = stream.next().await {
        assert!(chunk_result.is_ok());
        chunks.push(chunk_result.unwrap());
    }

    assert!(!chunks.is_empty());
    assert!(chunks.iter().any(|c| c.contains("Hello")));
    assert!(chunks.iter().any(|c| c.contains("world")));
}

#[tokio::test]
async fn llm_backend_sends_full_message_history_in_request() {
    // Verifies: When requesting a completion, the backend sends the full
    // conversation message history (system prompt + all user/assistant messages)
    // to the API, not just the latest message.

    let server = MockServer::start().await;

    // Track if request contains all messages using a custom matcher
    let response = ResponseTemplate::new(200)
        .set_body_json(serde_json::json!({
            "id": "chatcmpl-test",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "test-model",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Test response"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }));

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(response)
        .mount(&server)
        .await;

    let backend = OpenAiCompatBackend::new(server.uri(), "test-key".to_string(), 10);

    // Create full message history
    let messages = vec![
        ChatMessage::new("system", "You are a helpful assistant."),
        ChatMessage::new("user", "What is 2+2?"),
        ChatMessage::new("assistant", "4"),
        ChatMessage::new("user", "And 3+3?"),
    ];

    let result = backend.complete("test-model", messages.clone()).await;
    assert!(result.is_ok());

    // Verify that the request was made (proving all messages were sent)
    let requests = server.received_requests().await;
    assert!(!requests.is_empty());

    // Check the request body contains all messages
    let body_str = String::from_utf8_lossy(&requests[0].body);
    assert!(body_str.contains("You are a helpful assistant"));
    assert!(body_str.contains("What is 2+2?"));
    assert!(body_str.contains("3+3?"));
}

#[tokio::test]
async fn llm_backend_handles_api_timeout_gracefully() {
    // Verifies: When the AI API server doesn't respond within the timeout,
    // the backend returns a timeout error rather than hanging indefinitely.

    let server = MockServer::start().await;

    // Use a very long delay to trigger timeout
    let response = ResponseTemplate::new(200)
        .set_delay(std::time::Duration::from_secs(5))
        .set_body_json(serde_json::json!({
            "id": "chatcmpl-test",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "test-model",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Test response"
                    },
                    "finish_reason": "stop"
                }
            ]
        }));

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(response)
        .mount(&server)
        .await;

    // Create backend with very short timeout (1 second)
    let backend = OpenAiCompatBackend::new(server.uri(), "test-key".to_string(), 1);
    let messages = vec![ChatMessage::new("user", "Hello")];

    let result = backend.complete("test-model", messages).await;
    assert!(result.is_err());

    if let Err(LlmError::Timeout) = result {
        // Expected behavior
    } else {
        panic!("Expected Timeout error, got: {:?}", result);
    }
}

#[tokio::test]
async fn model_list_reflects_provider_registry_configuration() {
    // Verifies: The ProviderRegistry correctly loads model configurations
    // and exposes them for the API to list available models.

    let mut registry = ProviderRegistry::new();

    // Register model configurations
    let config1 = ModelConfig::new("ollama/llama3", "ollama", "http://localhost:11434");
    let config2 = ModelConfig::new("openai/gpt-4", "openai", "https://api.openai.com/v1");

    registry.register(config1);
    registry.register(config2);

    // List models and verify
    let models = registry.list_models();
    assert_eq!(models.len(), 2);

    // Verify model data
    assert_eq!(models[0].model_id, "ollama/llama3");
    assert_eq!(models[0].provider, "ollama");
    assert_eq!(models[0].api_base, "http://localhost:11434");

    assert_eq!(models[1].model_id, "openai/gpt-4");
    assert_eq!(models[1].provider, "openai");
    assert_eq!(models[1].api_base, "https://api.openai.com/v1");
}
