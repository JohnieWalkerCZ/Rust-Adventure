#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Door {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}
