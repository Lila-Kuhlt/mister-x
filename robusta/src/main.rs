use axum::{
    body::{boxed, Body, BoxBody},
    debug_handler,
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::{Request, Uri},
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router,
};
use chrono::Local;
use futures_util::SinkExt;
use kvv::LineDepartures;
use reqwest::StatusCode;
use tokio::sync::mpsc::Sender;
use tower::util::ServiceExt;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::{error, info, Level};
use ws_message::{ClientMessage, GameState, Team};

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
    ClientDisconnected(u32),
}

#[derive(Debug)]
struct Client {
    recv: tokio::sync::mpsc::Receiver<GameState>,
    send: tokio::sync::mpsc::Sender<InputMessage>,
    id: u32,
}

#[derive(Debug, Clone)]
struct ClientConnection {
    id: u32,
    team_id: u32,
    send: tokio::sync::mpsc::Sender<GameState>,
}

#[derive(Debug, Clone)]
struct AppState {
    pub teams: Vec<ws_message::Team>,
    pub game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>,
    pub connections: Vec<ClientConnection>,
    pub client_id_counter: u32,
    pub team_id_counter: u32,
}

impl AppState {
    const fn new(game_logic_sender: tokio::sync::mpsc::Sender<InputMessage>) -> Self {
        Self {
            teams: Vec::new(),
            game_logic_sender,
            connections: Vec::new(),
            client_id_counter: 0,
            team_id_counter: 0,
        }
    }

    fn client(&self, id: u32) -> Option<&ClientConnection> {
        self.connections.iter().find(|x| x.id == id)
    }
    fn client_mut(&mut self, id: u32) -> Option<&mut ClientConnection> {
        self.connections.iter_mut().find(|x| x.id == id)
    }
    fn team_mut_by_client_id(&mut self, id: u32) -> Option<&mut Team> {
        self.client(id)
            .map(|x| x.team_id)
            .and_then(|team_id| self.teams.iter_mut().find(|team| team.id == team_id))
    }
}

type SharedState = std::sync::Arc<tokio::sync::Mutex<AppState>>;

#[debug_handler]
async fn handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    let (send, rec) = tokio::sync::mpsc::channel(100);
    let client = {
        let mut state = state.lock().await;
        state.client_id_counter += 1;
        let client_connection = ClientConnection {
            id: state.client_id_counter,
            team_id: 0,
            send,
        };
        state.connections.push(client_connection);
        Client {
            recv: rec,
            send: state.game_logic_sender.clone(),
            id: state.client_id_counter,
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
            .clone()
            .send(InputMessage::Server(ServerMessage::ClientDisconnected(
                client_id,
            )))
            .await
            .expect("game logic queue disconnected");
    };

    // Propagate ws update to the game logic queue
    tokio::task::spawn(async move {
        while let Some(msg) = recv.next().await {
            let msg = if let Ok(Ok(msg)) = msg.map(|msg| msg.to_text().map(|msg| msg.to_owned())) {
                if let Ok(msg) = serde_json::from_str::<ws_message::ClientMessage>(&msg) {
                    msg
                } else {
                    // invalid message
                    tracing::warn!("Recieved invalid message: {}", msg);
                    continue;
                }
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
        let msg = serde_json::to_string(&update).unwrap();

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
) -> impl IntoResponse {
    let team = {
        let mut state = state.lock().await;
        state.team_id_counter += 1;
        let team = Team {
            id: state.team_id_counter,
            color: team.color,
            name: team.name,
            ..Default::default()
        };
        state.teams.push(team.clone());
        team
    };
    Json(team)
}

async fn list_teams(State(state): State<SharedState>) -> Json<Vec<Team>> {
    let state = state.lock().await;
    Json(state.teams.clone())
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

    let teams = std::fs::read_to_string("teams.json")
        .map(|x| serde_json::from_str::<Vec<Team>>(&x).unwrap())
        .unwrap_or_default();

    let mut state = AppState::new(send.clone());
    state.teams = teams;

    let state = std::sync::Arc::new(tokio::sync::Mutex::new(state));

    // fetch departures every 5 seconds and send them to the game logic queue
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
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
        .nest_service(
            "/",
            get_service(
                ServeDir::new("../liberica/dist")
                    .fallback(ServeFile::new("../liberica/dist/index.html")),
            ),
        )
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    tracing::info!("Starting web server");

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn run_game_loop(mut recv: tokio::sync::mpsc::Receiver<InputMessage>, state: SharedState) {
    let mut tick = 0;
    let mut game_state = GameState::new();
    let departures = &mut kvv::fetch_departures_for_region().await;
    loop {
        tick += 1;
        tracing::trace!("tick {}", tick);

        std::fs::write(
            "teams.json",
            serde_json::to_string_pretty(&game_state.teams).unwrap(),
        )
        .unwrap();
        let mut state = state.lock().await;
        while let Ok(msg) = recv.try_recv() {
            match msg {
                InputMessage::Client(msg, id) => {
                    info!("Got message from client {}: {:?}", id, msg);
                    match msg {
                        ClientMessage::Position { long, lat } => {
                            if let Some(team) = state.team_mut_by_client_id(id) {
                                if team.name != "Mr. X" {
                                    team.long = (long + team.long) / 2.;
                                    team.lat = (lat + team.lat) / 2.;
                                }
                            }
                        }
                        ClientMessage::Message(msg) => {
                            info!("Got message: {}", msg);
                        }
                        ClientMessage::JoinTeam { team_id } => {
                            let Some(client) = state.client_mut(id) else {
                                tracing::warn!("Client {} not found", id);
                                continue;
                            };
                            client.team_id = team_id;
                        }
                        ClientMessage::EmbarkTrain { train_id } => {
                            if let Some(team) = state.team_mut_by_client_id(id) {
                                team.on_train = Some(train_id);
                            }
                        }
                        ClientMessage::DisembarkTrain(_) => {
                            if let Some(team) = state.team_mut_by_client_id(id) {
                                team.on_train = None;
                            }
                        }
                    }
                }
                InputMessage::Server(ServerMessage::Departures(deps)) => {
                    *departures = deps;
                }
                InputMessage::Server(ServerMessage::ClientDisconnected(id)) => {
                    tracing::info!("Client {} disconnected", id);
                    state.connections.retain(|x| x.id != id);
                }
            }
        }

        tracing::trace!("updating train positions");
        let time = Local::now().with_timezone(&chrono_tz::Europe::Berlin);
        let mut trains = kvv::train_positions(departures, time.naive_local()).await;
        trains.retain(|x| !x.line_id.contains("bus"));

        // update positions for players on trains
        for team in state.teams.iter_mut() {
            if let Some(train_id) = &team.on_train {
                if let Some(train) = trains.iter().find(|x| &x.line_id == train_id) {
                    team.long = train.long;
                    team.lat = train.lat;
                }
            }
        }

        game_state.trains = trains;
        game_state.teams = state.teams.clone();

        for connection in state.connections.iter_mut() {
            if connection.send.send(game_state.clone()).await.is_err() {
                continue;
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
