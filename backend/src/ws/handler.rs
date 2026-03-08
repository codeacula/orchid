use crate::auth::handlers::current_user;
use crate::state::AppState;
use crate::ws::protocol::{ClientFrame, ServerFrame};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let Some(user) = current_user(&state, &headers).await else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    ws.on_upgrade(move |socket| handle_socket(state, user, socket))
}

async fn handle_socket(state: Arc<AppState>, user: crate::state::PublicUser, mut socket: WebSocket) {
    if send_frame(&mut socket, &ServerFrame::Ack).await.is_err() {
        return;
    }

    while let Some(Ok(message)) = socket.next().await {
        match message {
            Message::Text(text) => {
                let Ok(frame) = serde_json::from_str::<ClientFrame>(&text) else {
                    continue;
                };

                match frame {
                    ClientFrame::SendMessage {
                        conversation_id,
                        content,
                        model_id,
                    } => {
                        let model_id = match state
                            .add_user_message(&user, &conversation_id, content.clone(), model_id)
                            .await
                        {
                            Ok(model_id) => model_id,
                            Err(error) => {
                                let _ = send_frame(
                                    &mut socket,
                                    &ServerFrame::Error {
                                        conversation_id,
                                        message: format!("{error:?}"),
                                    },
                                )
                                .await;
                                continue;
                            }
                        };

                        let reply = state
                            .generate_assistant_reply(&user, &conversation_id, &content, &model_id)
                            .await;

                        let mut assembled = String::new();
                        for token in tokenize(&reply) {
                            assembled.push_str(&token);
                            if send_frame(
                                &mut socket,
                                &ServerFrame::Token {
                                    conversation_id: conversation_id.clone(),
                                    content: token,
                                },
                            )
                            .await
                            .is_err()
                            {
                                return;
                            }
                        }

                        let _ = state
                            .add_assistant_message(&user, &conversation_id, assembled, model_id)
                            .await;

                        let state_for_title = state.clone();
                        let user_for_title = user.clone();
                        let conversation_for_title = conversation_id.clone();
                        tokio::spawn(async move {
                            let _ = state_for_title
                                .maybe_generate_title(&user_for_title, &conversation_for_title)
                                .await;
                        });

                        if send_frame(
                            &mut socket,
                            &ServerFrame::Done {
                                conversation_id,
                            },
                        )
                        .await
                        .is_err()
                        {
                            return;
                        }
                    }
                }
            }
            Message::Close(_) => {
                let _ = socket.close().await;
                return;
            }
            _ => {}
        }
    }
}

async fn send_frame(socket: &mut WebSocket, frame: &ServerFrame) -> Result<(), axum::Error> {
    socket
        .send(Message::Text(serde_json::to_string(frame).expect("serializable").into()))
        .await
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|token| format!("{token} "))
        .collect()
}
