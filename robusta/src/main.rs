use std::fs;
use std::time::Duration;
use std::{collections::HashMap, ops::ControlFlow};

use axum::{
    body::{boxed, Body, BoxBody},
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::{Request, Uri},
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router,
};
use chai::unique_id::UniqueIdGen;
use chai::ws_message::Team;
use chai::{kvv, ServerResponse};
use chai::{ws_message, InputMessage, ServerMessage};
use futures_util::SinkExt;
use reqwest::StatusCode;
use tokio::sync::mpsc::Sender;
use tower::util::ServiceExt;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::{error, info, warn, Level};

const TEAMS_FILE: &str = "teams.json";

/// The name used for the Mr. X team.
const MRX: &str = "Mr. X";

#[derive(Debug)]
struct Client {
    recv: tokio::sync::mpsc::Receiver<String>,
    send: tokio::sync::mpsc::Sender<InputMessage>,
    id: u32,
}

#[derive(Debug)]
struct ClientConnection {
    id: u32,
    send: tokio::sync::mpsc::Sender<String>,
}

#[derive(Debug)]
struct AppState {
    pub teams_state: chai::AppState,
    pub game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>,
    pub connections: Vec<ClientConnection>,
    pub client_id_gen: UniqueIdGen,
    pub team_id_gen: UniqueIdGen,
}

impl AppState {
    fn new(game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>) -> Self {
        Self {
            teams_state: Default::default(),
            game_logic_sender,
            connections: Vec::new(),
            client_id_gen: UniqueIdGen::new(),
            team_id_gen: UniqueIdGen::new(),
        }
    }
}

type SharedState = std::sync::Arc<tokio::sync::Mutex<AppState>>;

async fn handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    let (send, rec) = tokio::sync::mpsc::channel(100);
    let client = {
        let mut state = state.lock().await;
        let id = state.client_id_gen.next();
        let client_connection = ClientConnection { id, send };
        state.connections.push(client_connection);
        Client {
            recv: rec,
            send: state.game_logic_sender.clone(),
            id,
        }
    };
    ws.on_upgrade(|socket| handle_socket(socket, client))
}

async fn handle_socket(socket: WebSocket, mut client: Client) {
    use futures_util::stream::StreamExt;

    let (mut send, mut recv) = socket.split();
    let client_send = client.send.clone();
    let client_id = client.id;

    let disconnect = |client_send: Sender<InputMessage>, client_id| async move {
        client_send
            .send(InputMessage::Server(ServerMessage::ClientDisconnected(
                client_id,
            )))
            .await
            .expect("game logic queue disconnected");
    };

    // Propagate ws update to the game logic queue
    tokio::task::spawn(async move {
        while let Some(msg) = recv.next().await {
            let msg = if let Some(msg) = msg.ok().and_then(|msg| {
                if matches!(msg, axum::extract::ws::Message::Close(_)) {
                    None
                } else {
                    msg.into_text().ok()
                }
            }) {
                msg
            } else {
                // client disconnected
                disconnect(client.send, client.id).await;
                return;
            };

            client
                .send
                .send(InputMessage::Client(msg, client.id))
                .await
                .expect("game logic queue disconnected");
        }
    });

    // Push game updates to the ws stream
    while let Some(update) = client.recv.recv().await {
        let msg = update;

        if send.send(msg.into()).await.is_err() {
            disconnect(client_send, client_id).await;
            return;
        }
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match tower_http::services::ServeDir::new("../liberica/dist")
        .oneshot(req)
        .await
    {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    dbg!(&uri);
    let res = get_static_file(uri.clone()).await?;
    dbg!(&res);

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn create_team(
    State(state): State<SharedState>,
    Json(team): Json<ws_message::CreateTeam>,
) -> Result<Json<Team>, ws_message::CreateTeamError> {
    let mut state = state.lock().await;
    let team_name = team.name.trim();

    // validation
    if team_name.is_empty() {
        return Err(ws_message::CreateTeamError::InvalidName);
    } else if state.teams_state.teams.iter().any(|t| t.name == team_name) {
        return Err(ws_message::CreateTeamError::NameAlreadyExists);
    }

    let team = Team {
        id: state.team_id_gen.next(),
        color: team.color,
        name: team_name.to_owned(),
        ..Default::default()
    };
    state.teams_state.teams.push(team.clone());
    Ok(Json(team))
}

async fn list_teams(State(state): State<SharedState>) -> Json<Vec<Team>> {
    let state = state.lock().await;
    Json(state.teams_state.teams.clone())
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

    const BINDINGS: &str = "../liberica/src/lib/bindings.ts";
    const TEMP_BINDINGS: &str = "../target/bindings.ts.tmp";
    specta::export::ts(TEMP_BINDINGS).unwrap();
    let old = fs::read_to_string(BINDINGS).unwrap_or_default();
    let new = fs::read_to_string(TEMP_BINDINGS).unwrap();
    // Only update bindings if they changed to avoid triggering a recompile of the frontend
    if old != new {
        info!("Updating bindings");
        fs::write(BINDINGS, new).unwrap();
    }

    info!("Starting server");
    kvv::init().await;

    let (send, recv) = tokio::sync::mpsc::channel(100);

    let mut teams = fs::read_to_string(TEAMS_FILE)
        .ok()
        .and_then(|x| serde_json::from_str::<Vec<Team>>(&x).ok())
        .unwrap_or_default();

    let mut state = AppState::new(send.clone());
    let max_id = teams.iter().map(|x| x.id).max().unwrap_or(0);
    state.team_id_gen.set_min(max_id + 1);
    if !teams.iter().any(|team| team.mr_x) {
        // no Mr. X present
        teams.push(Team {
            id: state.team_id_gen.next(),
            long: 0.0,
            lat: 0.0,
            on_train: None,
            name: MRX.to_owned(),
            color: "#FFFFFF".to_owned(),
            mr_x: true,
        });
    }
    state.teams_state.teams = teams;

    let state = std::sync::Arc::new(tokio::sync::Mutex::new(state));

    // fetch departures every 60 seconds and send them to the game logic queue
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let departures = kvv::fetch_departures_for_region().await;
            if departures.is_empty() {
                warn!("Fetched no departures");
            }
            if let Err(err) = send
                .send(InputMessage::Server(ServerMessage::Departures(departures)))
                .await
            {
                error!("Error while fetching data: {err}")
            }
        }
    });

    info!("Starting game loop");
    tokio::spawn(run_game_loop(recv, state.clone()));

    let api = Router::new()
        .route("/create-team", post(create_team))
        .route("/teams", get(list_teams))
        .route("/stops", get(list_stops))
        .route("/ping", get(|_: ()| async { "pong" }))
        .with_state(state.clone());

    // build our application with a single route
    let app = Router::new()
        .route("/ws", get(handler))
        .nest("/api", api)
        .nest_service(
            "/",
            get_service(
                ServeDir::new("../liberica/dist")
                    .fallback(ServeFile::new("../liberica/dist/index.html")),
            ),
        )
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    info!("Starting web server");

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn run_game_loop(mut recv: tokio::sync::mpsc::Receiver<InputMessage>, state: SharedState) {
    let mut departures = HashMap::new();
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;

        let mut state = state.lock().await;
        while let Ok(msg) = recv.try_recv() {
            // TODO: this does not do anything yet
            if let ControlFlow::Break(_) =
                chai::process_message(msg, &mut state.teams_state, &mut departures)
            {
                continue;
            }
        }

        let msg = chai::generate_response(&departures, &mut state.teams_state);

        fs::write(
            TEAMS_FILE,
            serde_json::to_string_pretty(&state.teams_state.teams).unwrap(),
        )
        .unwrap();

        match msg {
            ServerResponse::Broadcast(msg) => {
                for connection in state.connections.iter_mut() {
                    if connection.send.send(msg.clone()).await.is_err() {
                        continue;
                    }
                }
            }
            ServerResponse::P2P(msg, id) => {
                if let Some(connection) = state.connections.iter_mut().find(|x| x.id == id) {
                    if connection.send.send(msg).await.is_err() {
                        continue;
                    }
                }
            }
        }
    }
}
