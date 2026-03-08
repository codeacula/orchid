use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub backend_port: u16,
    pub database_url: Option<String>,
    pub redis_url: Option<String>,
    pub owner_username: String,
    pub owner_password: String,
    pub owner_system_prompt: String,
    pub user_system_prompt: String,
    pub ai_provider: String,
    pub ai_api_key: String,
    pub ai_model: String,
    pub ai_base_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            backend_port: env::var("BACKEND_PORT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL").ok(),
            redis_url: env::var("REDIS_URL").ok(),
            owner_username: env::var("OWNER_USERNAME").unwrap_or_else(|_| "admin".to_string()),
            owner_password: env::var("OWNER_PASSWORD").unwrap_or_else(|_| "change-me".to_string()),
            owner_system_prompt: env::var("OWNER_SYSTEM_PROMPT").unwrap_or_else(|_| {
                "You are a helpful assistant for the system owner.".to_string()
            }),
            user_system_prompt: env::var("USER_SYSTEM_PROMPT")
                .unwrap_or_else(|_| "You are a helpful assistant.".to_string()),
            ai_provider: env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string()),
            ai_api_key: env::var("AI_API_KEY").unwrap_or_default(),
            ai_model: env::var("AI_MODEL")
                .unwrap_or_else(|_| "Hermes-3-Llama-3.1-8B-Q6_K_L".to_string()),
            ai_base_url: env::var("AI_BASE_URL")
                .unwrap_or_else(|_| "http://192.168.1.50:8080".to_string()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
