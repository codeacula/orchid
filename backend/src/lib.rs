pub mod ai;
pub mod api;
pub mod auth;
pub mod config;
pub mod domain;
pub mod query;
pub mod state;
pub mod ws;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub use config::AppConfig;
pub use state::AppState;

pub async fn create_app(config: AppConfig) -> anyhow::Result<Router> {
    let state = Arc::new(AppState::new(config).await?);

    Ok(Router::new()
        .route("/", get(api::handlers::root))
        .route("/health", get(api::handlers::health))
        .nest("/api", api::router::router())
        .route("/ws", get(ws::handler::ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state))
}
