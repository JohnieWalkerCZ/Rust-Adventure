use super::consts::Position;

#[derive(PartialEq, Clone, Copy)]
pub struct Player {
    pub position: Position,
    pub level: u16,
}

impl Player {
    pub fn new(position: Position) -> Player {
        Player { position, level: 1 }
    }
}
