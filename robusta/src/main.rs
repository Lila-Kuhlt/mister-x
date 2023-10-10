use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

mod ws_message;

mod kvv;

async fn handler(ws: WebSocketUpgrade) -> Response {
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

        let update = ws_message::ServerMessage::GameState(ws_message::GameState::default());
        let msg = serde_json::to_string(&update).unwrap();

        if socket.send(msg.into()).await.is_err() {
            // client disconnected
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    specta::export::ts("../liberica/src/lib/bindings.ts").unwrap();

    // build our application with a single route
    let app = Router::new().route("/ws", get(handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
