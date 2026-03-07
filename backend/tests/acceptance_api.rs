//! Acceptance Tests: REST API Endpoints
//!
//! These tests verify the HTTP API for conversation management.
//! They test the full Axum router with mocked/in-memory backends where appropriate.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

#[tokio::test]
async fn create_conversation_returns_201_with_conversation_id() {
    // Verifies: POST /api/conversations with a valid session creates a new conversation
    // and returns 201 Created with the conversation ID in the response body.
    todo!("not yet implemented");
}

#[tokio::test]
async fn list_conversations_returns_conversation_summaries() {
    // Verifies: GET /api/conversations returns a JSON array of conversation summaries
    // (id, title, last_message_preview, updated_at) for the authenticated user.
    todo!("not yet implemented");
}

#[tokio::test]
async fn get_conversation_returns_full_message_history() {
    // Verifies: GET /api/conversations/:id returns the full message history
    // for the specified conversation, including role, content, model, and timestamps.
    todo!("not yet implemented");
}

#[tokio::test]
async fn get_nonexistent_conversation_returns_404() {
    // Verifies: GET /api/conversations/:id with a non-existent ID returns 404 Not Found.
    todo!("not yet implemented");
}

#[tokio::test]
async fn update_conversation_title_returns_200() {
    // Verifies: PATCH /api/conversations/:id with a new title updates the conversation
    // title and returns 200 OK.
    todo!("not yet implemented");
}

#[tokio::test]
async fn archive_conversation_returns_200() {
    // Verifies: DELETE /api/conversations/:id archives the conversation
    // (soft delete via event) and returns 200 OK.
    todo!("not yet implemented");
}

#[tokio::test]
async fn list_available_models_returns_configured_providers() {
    // Verifies: GET /api/models returns a JSON array of available model configurations
    // including provider_id, model_id, and display_name.
    todo!("not yet implemented");
}

#[tokio::test]
async fn send_message_with_model_selection_returns_202() {
    // Verifies: POST /api/conversations/:id/messages with { content, model_id }
    // returns 202 Accepted, indicating the message has been queued for processing.
    // The actual response will stream via WebSocket.
    todo!("not yet implemented");
}

#[tokio::test]
async fn accessing_another_users_conversation_returns_403() {
    // Verifies: GET /api/conversations/:id where the conversation belongs to another
    // user returns 403 Forbidden.
    todo!("not yet implemented");
}

#[tokio::test]
async fn health_check_endpoint_returns_200() {
    // Verifies: GET /api/health returns 200 OK with a JSON body indicating
    // service status (database connectivity, redis connectivity).
    todo!("not yet implemented");
}
