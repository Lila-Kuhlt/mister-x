use std::cell::Cell;

use axum::{
    debug_handler,
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use futures_util::SinkExt;
use kvv::LineDepartures;
use tracing::{error, event, info, span, Level};
use ws_message::{ClientMessage, GameState};

mod ws_message;

mod kvv;

#[derive(Debug)]
enum InputMessage {
    Client(ClientMessage, u32),
    Server(ServerMessage),
}

#[derive(Debug)]
enum ServerMessage {
    Departures(LineDepartures),
}

#[derive(Debug)]
struct Client {
    recv: tokio::sync::mpsc::Receiver<GameState>,
    send: tokio::sync::mpsc::Sender<InputMessage>,
}

#[derive(Debug, Clone)]
struct ClientConnection {
    id: u32,
    team_id: u32,
    name: String,
    send: tokio::sync::mpsc::Sender<GameState>,
}

#[derive(Debug, Clone)]
struct AppState {
    pub teams: Vec<ws_message::Team>,
    pub game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>,
    pub connections: Vec<ClientConnection>,
    pub id_counter: u32,
}

impl AppState {
    const fn new(game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>) -> Self {
        Self {
            teams: Vec::new(),
            game_logic_sender,
            connections: Vec::new(),
            id_counter: 0,
        }
    }
}

type SharedState = std::sync::Arc<tokio::sync::Mutex<AppState>>;

#[debug_handler]
async fn handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    let (send, rec) = tokio::sync::mpsc::channel(100);
    let client = {
        let mut state = state.lock().await;
        state.id_counter += 1;
        let client_connection = ClientConnection {
            id: state.id_counter,
            team_id: 0,
            name: String::new(),
            send,
        };
        state.connections.push(client_connection);
        Client {
            recv: rec,
            send: state.game_logic_sender.clone(),
        }
    };
    ws.on_upgrade(|socket| handle_socket(socket, client))
}

async fn handle_socket(socket: WebSocket, mut client: Client) {
    use futures_util::stream::{Stream, StreamExt};

    let (mut send, mut recv) = socket.split();

    // Propagate ws update to the game logic queue
    tokio::spawn(async move {
        while let Some(msg) = recv.next().await {
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

            client
                .send
                .send(InputMessage::Client(msg.clone(), 0))
                .await
                .unwrap_or_else(|_| return);

            match msg {
                ClientMessage::Position { x, y } => {
                    info!("Got position: {}, {}", x, y);
                }
                ClientMessage::Message(msg) => {
                    info!("Got message: {}", msg);
                }
            }
        }
    });

    // Push game updates to the ws stream
    while let Some(update) = client.recv.recv().await {
        let msg = serde_json::to_string(&update).unwrap();

        if send.send(msg.into()).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn create_team(
    State(state): State<SharedState>,
    Json(team): Json<ws_message::Team>,
) -> impl IntoResponse {
    {
        let mut state = state.lock().await;
        state.teams.push(team.clone());
    }
    Response::new(format!("Created, {}!", team.name))
}

async fn list_teams(State(state): State<SharedState>) -> impl IntoResponse {
    let teams = {
        let state = state.lock().await;
        Response::new(serde_json::to_string(&state.teams).unwrap())
    };
    Response::new(teams)
}

async fn list_stops() -> impl IntoResponse {
    let stops = kvv::KVV_STOPS.get().unwrap();
    Response::new(serde_json::to_string(&stops).unwrap())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    specta::export::ts("../liberica/src/lib/bindings.ts").unwrap();

    info!("Starting server");
    kvv::init().await;

    let (send, recv) = tokio::sync::mpsc::channel(100);

    let state = AppState::new(send.clone());

    let state = std::sync::Arc::new(tokio::sync::Mutex::new(state));

    // fetch departures every 5 seconds and send them to the game logic queue
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let departures = kvv::fetch_departures_for_region().await;
            if let Err(err) = send
                .send(InputMessage::Server(ServerMessage::Departures(departures)))
                .await
            {
                error!("Error while fetching data: {err}")
            }
        }
    });

    tracing::info!("Starting game loop");
    tokio::spawn(run_game_loop(recv, state.clone()));

    let api = Router::new()
        .route("/create-team", post(create_team))
        .route("/teams", get(list_teams))
        .route("/stops", get(list_stops))
        .with_state(state.clone());

    // build our application with a single route
    let app = Router::new()
        .route("/ws", get(handler))
        .nest("/api", api)
        .with_state(state.clone());

    tracing::info!("Starting server");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn run_game_loop(mut recv: tokio::sync::mpsc::Receiver<InputMessage>, state: SharedState) {
    tracing::info!("Starting game loop");
    let mut tick = 0;
    let mut game_state = GameState::new();
    let departures = &mut kvv::fetch_departures_for_region().await;
    let mut request_time = Utc::now();
    loop {
        tick += 1;
        tracing::trace!("tick {}", tick);

        let mut state = state.lock().await;
        while let Ok(msg) = recv.try_recv() {
            match msg {
                InputMessage::Client(msg, id) => {
                    info!("Got message from client {}: {:?}", id, msg);
                    if let Some(team) = state.teams.iter().find(|team| team.id == id) {
                        match msg {
                            ClientMessage::Position { x, y } => {
                                let t = game_state.teams.entry(id).or_insert_with(|| team.clone());
                                t.x = x;
                                t.y = y;
                            }
                            ClientMessage::Message(msg) => {
                                info!("Got message: {}", msg);
                            }
                        }
                    }
                }
                InputMessage::Server(ServerMessage::Departures(deps)) => {
                    *departures = deps;
                    request_time = Utc::now();
                }
            }
        }

        tracing::trace!("updating train positions");
        tracing::trace!(
            "offset: {:?}",
            (Utc::now() - request_time).num_milliseconds()
        );
        let mut trains = kvv::train_positions(departures, Utc::now() - request_time).await;
        trains.retain(|x| !x.line_id.contains("bus"));
        //dbg!(&trains);
        game_state.trains = trains;

        for connection in state.connections.iter_mut() {
            if connection.send.send(game_state.clone()).await.is_err() {
                return;
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}
