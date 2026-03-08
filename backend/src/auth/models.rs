use crate::state::PublicUser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub role: String,
}

impl From<PublicUser> for AuthenticatedUser {
    fn from(value: PublicUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
            role: match value.role {
                crate::state::UserRole::Owner => "owner".to_string(),
                crate::state::UserRole::User => "user".to_string(),
            },
        }
    }
}
