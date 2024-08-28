use super::consts::Position;

#[derive(PartialEq, Clone, Copy)]
pub struct Player {
    pub position: Position,
    pub level: u16,
    pub fighting: bool,
    health: u16,
}

impl Player {
    pub fn new(position: Position) -> Player {
        Player {
            position,
            level: 1,
            fighting: false,
            health: 100,
        }
    }

    pub fn set_fighting(&mut self, value: bool) {
        self.fighting = value;
    }

    pub fn decrease_health(&mut self, value: u16) {
        self.health -= value;
    }
}
