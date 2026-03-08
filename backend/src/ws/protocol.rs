use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientFrame {
    SendMessage {
        conversation_id: String,
        content: String,
        model_id: Option<String>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerFrame {
    Ack,
    Token {
        conversation_id: String,
        content: String,
    },
    Done {
        conversation_id: String,
    },
    Error {
        conversation_id: String,
        message: String,
    },
}
