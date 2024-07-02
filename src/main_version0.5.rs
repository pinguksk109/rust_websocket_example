// 参考にしたドキュメント
// https://docs.rs/axum/latest/axum/extract/ws/index.html

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::IntoResponse,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/ws", get(handler));

    // Run it with hyper on localhost:3000
    // いったんローカルで動かす事を考えて、127.0.0.1で指定している
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let addr = "127.0.0.1:3000".parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => return,
        };

        if let Err(_) = socket.send(msg).await {
            return;
        }
    }
}