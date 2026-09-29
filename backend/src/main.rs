#[tokio::main]
async fn main() {
    let bind_addr = "127.0.0.1:8081";
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap_or_else(|err| panic!("bind {bind_addr}: {err}"));
    let app = backend::app();
    axum::serve(listener, app).await.expect("serve");
}
