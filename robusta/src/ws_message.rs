
#[derive(specta::Type)]
pub enum ClientMessage {
    Position { x: f32, y: f32 },
    Message(String),
}

#[derive(specta::Type)]
pub enum ServerMessage {
    GameState(GameState),
}


#[derive(specta::Type)]
pub struct GameState {
    pub players: Vec<Player>,
    pub trains: Vec<Train>,
}


#[derive(specta::Type)]
pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub color: String,
}

#[derive(specta::Type)]
pub struct Train {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub line_id: Line,
    pub direction: String,
}

#[derive(specta::Type)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub color: String,
}

