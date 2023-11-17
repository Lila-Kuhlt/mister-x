use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(specta::Type, Clone, Deserialize, Debug)]
pub enum ClientMessage {
    Position { long: f32, lat: f32 },
    SetTeamPosition { long: f32, lat: f32, team_id: u32 },
    JoinTeam { team_id: u32 },
    EmbarkTrain { train_id: String },
    DisembarkTrain(u8),
    Message(String),
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub teams: Vec<Team>,
    pub trains: Vec<Train>,
}

#[derive(Default, Clone, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub team_id: u32,
}

#[derive(specta::Type, Clone, Deserialize, Debug)]
pub struct CreateTeam {
    pub name: String,
    pub color: String,
    pub kind: TeamKind,
}

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum TeamKind {
    MrX,
    #[default]
    Detective,
    Observer,
}

#[derive(Clone, Debug)]
pub enum CreateTeamError {
    InvalidName,
    NameAlreadyExists,
}
impl IntoResponse for CreateTeamError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::InvalidName => "invalid name",
            Self::NameAlreadyExists => "name already exists",
        };

        (StatusCode::UNPROCESSABLE_ENTITY, body).into_response()
    }
}

#[derive(specta::Type, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: u32,
    pub long: f32,
    pub lat: f32,
    pub on_train: Option<String>,
    pub name: String,
    pub color: String,
    pub kind: TeamKind,
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

#[derive(Default, Clone, Debug)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub color: String,
}
