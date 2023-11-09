use serde::{Deserialize, Serialize};

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Position { long: f32, lat: f32 },
    SetTeamPosition { long: f32, lat: f32, team_id: u32 },
    JoinTeam { team_id: u32 },
    EmbarkTrain { train_id: String },
    DisembarkTrain(u8),
    Message(String),
}

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    GameState(GameState),
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub teams: Vec<Team>,
    pub trains: Vec<Train>,
}
impl GameState {
    pub(crate) const fn new() -> GameState {
        GameState {
            teams: Vec::new(),
            trains: Vec::new(),
        }
    }
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub team_id: u32,
}

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub struct CreateTeam {
    pub name: String,
    pub color: String,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: u32,
    pub long: f32,
    pub lat: f32,
    pub on_train: Option<String>,
    pub name: String,
    pub color: String,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Train {
    pub id: u32,
    pub long: f32,
    pub lat: f32,
    pub line_id: String,
    pub line_name: String,
    pub direction: String,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub color: String,
}
