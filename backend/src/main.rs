#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("bind 127.0.0.1:8081");
    let app = backend::app();
    axum::serve(listener, app).await.expect("serve");
}
