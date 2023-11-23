use std::io::Write;
use std::{collections::HashMap, fs, ops::ControlFlow};

use kvv::LineDepartures;
use tracing::{info, log::warn};
use ws_message::{GameState, Team};

use crate::ws_message::ClientMessage;

pub mod kvv;
pub mod point;
pub mod unique_id;
pub mod ws_message;
const LOG_FILE: &str = "log.csv";

#[derive(Debug)]
pub enum InputMessage {
    Client(String, u32),
    Server(ServerMessage),
}

#[derive(Debug)]
pub enum ServerMessage {
    Departures(LineDepartures),
    ClientDisconnected(u32),
}

#[derive(Debug)]
pub enum ServerResponse {
    Broadcast(String),
    P2P(String, u32),
}

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub team_id: u32,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub connections: Vec<Connection>,
    pub teams: Vec<Team>,
}

impl AppState {
    pub fn client_mut(&mut self, id: u32) -> Option<&mut Connection> {
        self.connections.iter_mut().find(|x| x.id == id)
    }

    pub fn client(&self, id: u32) -> Option<&Connection> {
        self.connections.iter().find(|x| x.id == id)
    }

    pub fn team_mut_by_client_id(&mut self, id: u32) -> Option<&mut Team> {
        if let Some(team_id) = self.client(id).map(|x| x.team_id) {
            self.teams.iter_mut().find(|t| t.id == team_id)
        } else {
            None
        }
    }
}

pub fn process_message(
    msg: InputMessage,
    state: &mut AppState,
    departures: &mut HashMap<String, kvv::Journey>,
) -> ControlFlow<()> {
    match msg {
        InputMessage::Client(msg, id) => {
            let msg = serde_json::from_str(&msg).unwrap();
            info!("Got message from client {}: {:?}", id, msg);
            match msg {
                ClientMessage::Position { long, lat } => {
                    if let Some(team) = state.team_mut_by_client_id(id) {
                        team.long = (long + team.long) / 2.;
                        team.lat = (lat + team.lat) / 2.;
                    }
                }
                ClientMessage::SetTeamPosition { long, lat, team_id } => {
                    if let Some(team) = state.teams.iter_mut().find(|t| t.id == team_id) {
                        team.long = long;
                        team.lat = lat;
                    }
                }
                ClientMessage::Message(msg) => {
                    info!("Got message: {}", msg);
                }
                ClientMessage::JoinTeam { team_id } => {
                    let Some(client) = state.client_mut(id) else {
                        warn!("Client {} not found", id);
                        return ControlFlow::Break(());
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
            info!("Client {} disconnected", id);
            state.connections.retain(|x| x.id != id);
        }
    }
    ControlFlow::Continue(())
}

pub fn generate_respone(
    departures: &HashMap<String, kvv::Journey>,
    state: &mut AppState,
) -> ServerResponse {
    let time = chrono::Utc::now();
    let mut trains = kvv::train_positions(departures, time);
    trains.retain(|x| !x.line_id.contains("bus"));
    let mut log_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_FILE)
        .unwrap();

    // update positions for players on trains
    for team in state.teams.iter_mut() {
        if let Some(train_id) = &team.on_train {
            if let Some(train) = trains.iter().find(|x| &x.line_id == train_id) {
                team.long = train.long;
                team.lat = train.lat;
            }
        }
    }

    let game_state = GameState {
        teams: state.teams.clone(),
        trains,
    };
    writeln!(
        log_file,
        "{}, {}",
        time.with_timezone(&chrono_tz::Europe::Berlin).to_rfc3339(),
        serde_json::to_string(&game_state).unwrap()
    )
    .unwrap();
    ServerResponse::Broadcast(serde_json::to_string(&game_state).unwrap())
}
