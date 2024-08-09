use super::consts::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Enemy {
    pub level: u16,
    pub position: Position,
}
