use crate::auth::models::{AuthenticatedUser, LoginRequest, RegisterRequest};
use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use std::sync::Arc;

pub const SESSION_COOKIE_NAME: &str = "orchid_session";

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.authenticate(&payload.username, &payload.password).await {
        Some(user) => {
            let token = state.create_session(&user).await;
            let mut headers = HeaderMap::new();
            headers.insert(header::SET_COOKIE, session_cookie(&token));
            (StatusCode::OK, headers, Json(AuthenticatedUser::from(user))).into_response()
        }
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn me(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    match current_user(&state, &headers).await {
        Some(user) => (StatusCode::OK, Json(AuthenticatedUser::from(user))).into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match state
        .seed_user(&payload.username, &payload.password, crate::state::UserRole::User)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::CONFLICT.into_response(),
    }
}

pub async fn logout(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(token) = session_token(&headers) {
        state.destroy_session(&token).await;
    }

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_static("orchid_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"),
    );

    (StatusCode::OK, response_headers).into_response()
}

pub async fn current_user(state: &Arc<AppState>, headers: &HeaderMap) -> Option<crate::state::PublicUser> {
    let token = session_token(headers)?;
    state.get_user_by_session(&token).await
}

pub fn session_token(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie_header
        .split(';')
        .map(|pair| pair.trim())
        .find_map(|pair| {
            let (name, value) = pair.split_once('=')?;
            if name == SESSION_COOKIE_NAME {
                Some(value.to_string())
            } else {
                None
            }
        })
}

fn session_cookie(token: &str) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "{SESSION_COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax"
    ))
    .expect("valid session cookie")
}
