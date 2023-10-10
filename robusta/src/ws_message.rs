#[derive(specta::Type)]
pub enum ClientMessage {
    Position { x: f32, y: f32 },
    Message(String),
}
