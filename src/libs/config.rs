pub const DOOR_DIST_1: [(u8, f32); 2] = [(0, 0.25), (1, 0.75)];

pub const DOOR_DIST_2: [(u8, f32); 3] = [(0, 0.15), (1, 0.35), (2, 0.5)];

pub const DOOR_DIST_3: [(u8, f32); 4] = [(0, 0.1), (1, 0.3), (2, 0.5), (3, 0.1)];

pub const ENEMY_DIST: [(u8, f32); 4] = [(0, 0.3), (1, 0.4), (2, 0.2), (3, 0.1)];

pub fn enemy_level_function(x: u16) -> u16 {
    return (x ^ 2) / 20;
}
