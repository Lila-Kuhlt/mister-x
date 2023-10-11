use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tracing::{event, info, span, Level};
use ws_message::ClientMessage;

mod ws_message;

mod kvv;

#[derive(Clone, Default)]
struct AppState {
    pub teams: Vec<ws_message::Team>,
}

type SharedState = std::sync::Arc<tokio::sync::Mutex<AppState>>;

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(Ok(msg)) = msg.map(|msg| msg.to_text().map(|msg| msg.to_owned())) {
            if let Ok(msg) = serde_json::from_str::<ws_message::ClientMessage>(&msg) {
                msg
            } else {
                // invalid message
                continue;
            }
        } else {
            // client disconnected
            return;
        };
        match msg {
            ClientMessage::Position { x, y } => {
                info!("Got position: {}, {}", x, y);
            }
            ClientMessage::Message(msg) => {
                info!("Got message: {}", msg);
            }
        }

        let update = ws_message::ServerMessage::GameState(ws_message::GameState::default());
        let msg = serde_json::to_string(&update).unwrap();

        if socket.send(msg.into()).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn create_team(
    State(state): State<SharedState>,
    Json(team): Json<ws_message::Team>,
) -> impl IntoResponse {
    let mut state = state.lock().await;
    state.teams.push(team.clone());
    Response::new(format!("Created, {}!", team.name))
}

async fn list_teams(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.lock().await;
    Response::new(serde_json::to_string(&state.teams).unwrap())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    specta::export::ts("../liberica/src/lib/bindings.ts").unwrap();

    let trains = kvv::train_positions();
    dbg!(trains.await);

    let state = AppState::default();

    let api = Router::new()
        .route("/create-team", post(create_team))
        .route("/teams", get(list_teams))
        .with_state(std::sync::Arc::new(tokio::sync::Mutex::new(state)));

    // build our application with a single route
    let app = Router::new().route("/ws", get(handler)).nest("/api", api);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
