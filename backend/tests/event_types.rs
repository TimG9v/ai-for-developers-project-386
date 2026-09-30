//! HTTP-шов: GET /event-types отдаёт список типов встреч в форме контракта.
//! Хранение in-memory, состояние подставляется через app_with_state.

use backend::api::api_types::EventType;
use backend::infra::InMemoryEventTypes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const EVENT_TYPES_REQUEST: &str =
    "GET /event-types HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";

async fn send(router: axum::Router, request: &str) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind to random port");
    let addr = listener
        .local_addr()
        .expect("read actual port from listener");

    tokio::spawn(async move {
        axum::serve(listener, router).await.expect("serve");
    });

    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect to server");
    stream
        .write_all(request.as_bytes())
        .await
        .expect("write request");

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .expect("read response");
    String::from_utf8_lossy(&response).into_owned()
}

#[tokio::test]
async fn event_types_list_returns_seeded_types_as_contract_json() {
    let repo = InMemoryEventTypes::new();
    repo.add(EventType {
        id: "et1".to_string(),
        title: "Знакомство".to_string(),
        description: Some("Первичный созвон".to_string()),
        duration_minutes: 30,
    });
    repo.add(EventType {
        id: "et2".to_string(),
        title: "Консультация".to_string(),
        description: None,
        duration_minutes: 60,
    });
    let app = backend::app_with_state(backend::AppState {
        event_types: std::sync::Arc::new(repo),
    });

    let raw = send(app, EVENT_TYPES_REQUEST).await;

    assert!(raw.contains("HTTP/1.1 200"), "got: {raw}");
    let body = raw.split("\r\n\r\n").nth(1).expect("response body");
    let json: serde_json::Value = serde_json::from_str(body).expect("JSON-тело");
    let items = json.as_array().expect("EventType[]");

    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["id"], "et1");
    assert_eq!(items[0]["title"], "Знакомство");
    assert_eq!(items[0]["description"], "Первичный созвон");
    assert_eq!(items[0]["durationMinutes"], 30);
    assert_eq!(items[1]["title"], "Консультация");
    assert_eq!(items[1]["durationMinutes"], 60);
    assert!(
        items[1].get("description").is_none(),
        "description без значения не сериализуется: {}",
        items[1]
    );
}

#[tokio::test]
async fn event_types_list_on_empty_storage_returns_empty_array() {
    let raw = send(backend::app(), EVENT_TYPES_REQUEST).await;

    assert!(raw.contains("HTTP/1.1 200"), "got: {raw}");
    let body = raw.split("\r\n\r\n").nth(1).expect("response body");
    assert_eq!(body.trim(), "[]", "пустое хранилище — валидный список");
}
