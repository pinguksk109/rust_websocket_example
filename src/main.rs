use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::IntoResponse,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/test", get(handler));
    let addr = "127.0.0.1:3000";
    match create_listener(addr).await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Server error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
        }
    }
}

async fn create_listener(addr: &str) -> Result<tokio::net::TcpListener, std::io::Error> {
    TcpListener::bind(addr).await
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}