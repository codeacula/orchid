use crate::domain::ConversationHistoryView;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateConversationResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConversationRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub model_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AcceptedResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub database: String,
    pub redis: String,
}

#[derive(Debug, Serialize)]
pub struct ModelResponse {
    pub provider_id: String,
    pub model_id: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    #[serde(flatten)]
    pub history: ConversationHistoryView,
}
