use crate::api::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<std::sync::Arc<crate::state::AppState>> {
    Router::new()
        .route("/health", get(handlers::health_with_state))
        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))
        .route("/auth/me", get(handlers::me))
        .route("/auth/logout", post(handlers::logout))
        .route("/models", get(handlers::models))
        .route(
            "/conversations",
            get(handlers::list_conversations).post(handlers::create_conversation),
        )
        .route(
            "/conversations/{conversation_id}",
            get(handlers::get_conversation)
                .patch(handlers::rename_conversation)
                .delete(handlers::archive_conversation),
        )
        .route(
            "/conversations/{conversation_id}/messages",
            post(handlers::send_message),
        )
}
