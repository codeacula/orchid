use axum::Router;
use futures_util::{SinkExt, StreamExt};
use orchid::AppConfig;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

fn test_config() -> AppConfig {
    AppConfig {
        backend_port: 0,
        database_url: None,
        redis_url: None,
        owner_username: "admin".to_string(),
        owner_password: "change-me".to_string(),
        owner_system_prompt: "Owner prompt".to_string(),
        user_system_prompt: "User prompt".to_string(),
        ai_provider: "openai".to_string(),
        ai_api_key: String::new(),
        ai_model: "gpt-4o".to_string(),
        ai_base_url: "http://127.0.0.1:9999".to_string(),
    }
}

async fn spawn_app() -> (SocketAddr, reqwest::Client) {
    let app: Router = orchid::create_app(test_config()).await.expect("app builds");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind listener");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server runs");
    });

    let client = reqwest::Client::new();

    (addr, client)
}

fn session_header(response: &reqwest::Response) -> String {
    response
        .headers()
        .get(reqwest::header::SET_COOKIE)
        .expect("session cookie")
        .to_str()
        .expect("cookie str")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_string()
}

#[tokio::test]
async fn login_me_and_logout_flow_works() {
    let (addr, client) = spawn_app().await;
    let base = format!("http://{addr}");

    let login = client
        .post(format!("{base}/api/auth/login"))
        .json(&serde_json::json!({"username":"admin","password":"change-me"}))
        .send()
        .await
        .expect("login request");
    assert!(login.status().is_success());
    let cookie = session_header(&login);

    let me = client
        .get(format!("{base}/api/auth/me"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("me request");
    assert!(me.status().is_success());
    let me_json: Value = me.json().await.expect("me json");
    assert_eq!(me_json["username"], "admin");
    assert_eq!(me_json["role"], "owner");

    let logout = client
        .post(format!("{base}/api/auth/logout"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("logout request");
    assert!(logout.status().is_success());

    let me_after = client
        .get(format!("{base}/api/auth/me"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("me after logout");
    assert_eq!(me_after.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn conversation_crud_flow_works() {
    let (addr, client) = spawn_app().await;
    let base = format!("http://{addr}");

    let login = client
        .post(format!("{base}/api/auth/login"))
        .json(&serde_json::json!({"username":"admin","password":"change-me"}))
        .send()
        .await
        .expect("login request");
    let cookie = session_header(&login);

    let created = client
        .post(format!("{base}/api/conversations"))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&serde_json::json!({"title":"Integration Chat"}))
        .send()
        .await
        .expect("create conversation");
    assert_eq!(created.status(), reqwest::StatusCode::CREATED);
    let created_json: Value = created.json().await.expect("create json");
    let conversation_id = created_json["id"].as_str().expect("conversation id");

    let list = client
        .get(format!("{base}/api/conversations"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("list conversations");
    assert!(list.status().is_success());
    let list_json: Value = list.json().await.expect("list json");
    assert_eq!(list_json.as_array().expect("array").len(), 1);

    let rename = client
        .patch(format!("{base}/api/conversations/{conversation_id}"))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&serde_json::json!({"title":"Renamed Chat"}))
        .send()
        .await
        .expect("rename conversation");
    assert!(rename.status().is_success());

    let fetched = client
        .get(format!("{base}/api/conversations/{conversation_id}"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("get conversation");
    assert!(fetched.status().is_success());
    let fetched_json: Value = fetched.json().await.expect("history json");
    assert_eq!(fetched_json["title"], "Renamed Chat");

    let archived = client
        .delete(format!("{base}/api/conversations/{conversation_id}"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("archive conversation");
    assert!(archived.status().is_success());

    let list_after = client
        .get(format!("{base}/api/conversations"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("list after archive");
    let list_after_json: Value = list_after.json().await.expect("list after json");
    assert!(list_after_json.as_array().expect("array").is_empty());
}

#[tokio::test]
async fn registration_flow_creates_regular_user() {
    let (addr, client) = spawn_app().await;
    let base = format!("http://{addr}");

    let register = client
        .post(format!("{base}/api/auth/register"))
        .json(&serde_json::json!({"username":"friend","password":"secret"}))
        .send()
        .await
        .expect("register request");
    assert_eq!(register.status(), reqwest::StatusCode::CREATED);

    let login = client
        .post(format!("{base}/api/auth/login"))
        .json(&serde_json::json!({"username":"friend","password":"secret"}))
        .send()
        .await
        .expect("login request");
    assert!(login.status().is_success());
    let cookie = session_header(&login);

    let me = client
        .get(format!("{base}/api/auth/me"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("me request");
    let me_json: Value = me.json().await.expect("me json");
    assert_eq!(me_json["username"], "friend");
    assert_eq!(me_json["role"], "user");
}

#[tokio::test]
async fn websocket_chat_flow_streams_tokens() {
    let (addr, client) = spawn_app().await;
    let base = format!("http://{addr}");

    let login = client
        .post(format!("{base}/api/auth/login"))
        .json(&serde_json::json!({"username":"admin","password":"change-me"}))
        .send()
        .await
        .expect("login request");
    let cookie = session_header(&login);

    let created = client
        .post(format!("{base}/api/conversations"))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&serde_json::json!({"title":"WS Chat"}))
        .send()
        .await
        .expect("create conversation");
    let created_json: Value = created.json().await.expect("create json");
    let conversation_id = created_json["id"].as_str().expect("conversation id");

    let mut request = format!("ws://{addr}/ws").into_client_request().expect("ws request");
    request.headers_mut().insert(
        "Cookie",
        cookie.parse().expect("cookie header"),
    );

    let (mut socket, _) = tokio_tungstenite::connect_async(request)
        .await
        .expect("connect websocket");
    let ack = socket.next().await.expect("ack frame").expect("ack message");
    assert!(matches!(ack, Message::Text(_)));

    socket
        .send(Message::Text(
            serde_json::json!({
                "type": "send_message",
                "conversation_id": conversation_id,
                "content": "hello orb",
                "model_id": "gpt-4o"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send ws frame");

    let mut saw_token = false;
    let mut saw_done = false;
    for _ in 0..10 {
        let frame = socket.next().await.expect("ws frame").expect("frame ok");
        let Message::Text(text) = frame else { continue };
        let payload: Value = serde_json::from_str(&text).expect("json frame");
        if payload["type"] == "token" {
            saw_token = true;
        }
        if payload["type"] == "done" {
            saw_done = true;
            break;
        }
    }

    assert!(saw_token);
    assert!(saw_done);
}

#[tokio::test]
async fn websocket_chat_generates_a_better_title() {
    let (addr, client) = spawn_app().await;
    let base = format!("http://{addr}");

    let login = client
        .post(format!("{base}/api/auth/login"))
        .json(&serde_json::json!({"username":"admin","password":"change-me"}))
        .send()
        .await
        .expect("login request");
    let cookie = session_header(&login);

    let created = client
        .post(format!("{base}/api/conversations"))
        .header(reqwest::header::COOKIE, &cookie)
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("create conversation");
    let created_json: Value = created.json().await.expect("create json");
    let conversation_id = created_json["id"].as_str().expect("conversation id");

    let mut request = format!("ws://{addr}/ws").into_client_request().expect("ws request");
    request.headers_mut().insert("Cookie", cookie.parse().expect("cookie header"));
    let (mut socket, _) = tokio_tungstenite::connect_async(request)
        .await
        .expect("connect websocket");
    let _ = socket.next().await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "type": "send_message",
                "conversation_id": conversation_id,
                "content": "plan my herb garden for spring",
                "model_id": "gpt-4o"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send ws frame");

    for _ in 0..10 {
        let frame = socket.next().await.expect("ws frame").expect("frame ok");
        if let Message::Text(text) = frame {
            let payload: Value = serde_json::from_str(&text).expect("json frame");
            if payload["type"] == "done" {
                break;
            }
        }
    }

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let fetched = client
        .get(format!("{base}/api/conversations/{conversation_id}"))
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .await
        .expect("get conversation");
    let fetched_json: Value = fetched.json().await.expect("conversation json");
    let title = fetched_json["title"].as_str().unwrap_or_default();
    assert!(!title.starts_with("Chat "));
    assert!(!title.is_empty());
}
