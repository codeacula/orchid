use crate::api::models::{
    AcceptedResponse, ConversationResponse, CreateConversationRequest,
    CreateConversationResponse, HealthResponse, ModelResponse, SendMessageRequest,
    UpdateConversationRequest,
};
use crate::auth::handlers as auth_handlers;
use crate::state::{ApiStateError, AppState};
use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Orchid backend is running")
}

pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        service: "orchid-backend",
        database: "configured".to_string(),
        redis: "configured".to_string(),
    })
}

pub async fn health_with_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (database, redis) = state.health_report().await;
    Json(HealthResponse {
        status: "ok",
        service: "orchid-backend",
        database,
        redis,
    })
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    payload: Json<crate::auth::models::LoginRequest>,
) -> impl IntoResponse {
    auth_handlers::login(State(state), payload).await
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    payload: Json<crate::auth::models::RegisterRequest>,
) -> impl IntoResponse {
    auth_handlers::register(State(state), payload).await
}

pub async fn me(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    auth_handlers::me(State(state), headers).await
}

pub async fn logout(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    auth_handlers::logout(State(state), headers).await
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    let id = state.create_conversation(&user, payload.title).await;
    (
        StatusCode::CREATED,
        Json(CreateConversationResponse { id }),
    )
        .into_response()
}

pub async fn list_conversations(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    (StatusCode::OK, Json(state.list_conversations(&user).await)).into_response()
}

pub async fn get_conversation(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match state.get_history(&user, &conversation_id).await {
        Ok(history) => (StatusCode::OK, Json(ConversationResponse { history })).into_response(),
        Err(err) => state_error_response(err),
    }
}

pub async fn rename_conversation(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(conversation_id): Path<String>,
    Json(payload): Json<UpdateConversationRequest>,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match state.rename_conversation(&user, &conversation_id, payload.title).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(err) => state_error_response(err),
    }
}

pub async fn archive_conversation(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match state.archive_conversation(&user, &conversation_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(err) => state_error_response(err),
    }
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let user = match require_user(&state, &headers).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match state
        .add_user_message(&user, &conversation_id, payload.content, payload.model_id)
        .await
    {
        Ok(_) => (
            StatusCode::ACCEPTED,
            Json(AcceptedResponse { status: "accepted" }),
        )
            .into_response(),
        Err(err) => state_error_response(err),
    }
}

pub async fn models(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let models = state
        .registry
        .list_models()
        .into_iter()
        .map(|model| ModelResponse {
            provider_id: model.provider.clone(),
            model_id: model.model_id.clone(),
            display_name: model.model_id.clone(),
        })
        .collect::<Vec<_>>();

    (StatusCode::OK, Json(models)).into_response()
}

async fn require_user(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<crate::state::PublicUser, axum::response::Response> {
    auth_handlers::current_user(state, headers)
        .await
        .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())
}

fn state_error_response(error: ApiStateError) -> axum::response::Response {
    match error {
        ApiStateError::NotFound => StatusCode::NOT_FOUND.into_response(),
        ApiStateError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        ApiStateError::Conflict(message) => {
            (StatusCode::CONFLICT, Json(json!({ "error": message }))).into_response()
        }
        ApiStateError::BadRequest(message) => {
            (StatusCode::BAD_REQUEST, Json(json!({ "error": message }))).into_response()
        }
    }
}
