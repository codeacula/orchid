//! Acceptance Tests: Authentication & Authorization
//!
//! These tests verify the login flow, session management, and route protection
//! using axum-login with tower-sessions.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

#[tokio::test]
async fn login_with_valid_credentials_returns_success_and_session_cookie() {
    // Verifies: POST /api/auth/login with valid username/password returns 200 OK
    // and a Set-Cookie header containing a session cookie.
    todo!("not yet implemented");
}

#[tokio::test]
async fn login_with_invalid_credentials_returns_unauthorized() {
    // Verifies: POST /api/auth/login with wrong password returns 401 Unauthorized.
    // No session cookie is set.
    todo!("not yet implemented");
}

#[tokio::test]
async fn protected_route_rejects_unauthenticated_request() {
    // Verifies: GET /api/conversations without a session cookie returns 401 Unauthorized.
    todo!("not yet implemented");
}

#[tokio::test]
async fn protected_route_allows_authenticated_request() {
    // Verifies: After logging in and obtaining a session cookie,
    // GET /api/conversations with the session cookie returns 200 OK.
    todo!("not yet implemented");
}

#[tokio::test]
async fn auth_me_returns_current_user_info() {
    // Verifies: GET /api/auth/me with a valid session cookie returns the current
    // user's info (id, username, role). Without a session cookie, returns 401.
    todo!("not yet implemented");
}

#[tokio::test]
async fn logout_invalidates_session() {
    // Verifies: POST /api/auth/logout with a valid session cookie invalidates
    // the session. Subsequent requests with the same cookie return 401.
    todo!("not yet implemented");
}

#[tokio::test]
async fn user_can_only_see_own_conversations() {
    // Verifies: Given two users each with conversations, when user A lists
    // conversations, they only see their own — not user B's.
    todo!("not yet implemented");
}

#[tokio::test]
async fn owner_role_gets_personalized_system_prompt() {
    // Verifies: When the owner sends a message, the system prompt used for
    // the AI request is the OWNER_SYSTEM_PROMPT (not the default user prompt).
    // This can be verified by inspecting the command/event metadata.
    todo!("not yet implemented");
}
