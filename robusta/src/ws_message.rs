use serde::{Deserialize, Serialize};

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Position { x: f32, y: f32 },
    Message(String),
}

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    GameState(GameState),
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub trains: Vec<Train>,
}
impl GameState {
    pub(crate) const fn new() -> GameState {
        GameState {
            players: Vec::new(),
            trains: Vec::new(),
        }
    }
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub team_id: u32,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub color: String,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Train {
    pub id: u32,
    pub long: f32,
    pub lat: f32,
    pub line_id: String,
    pub direction: String,
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub color: String,
}
