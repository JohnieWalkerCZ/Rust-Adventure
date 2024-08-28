pub const DOOR_DIST_1: [(u8, f32); 2] = [(0, 0.25), (1, 0.75)];

pub const DOOR_DIST_2: [(u8, f32); 3] = [(0, 0.15), (1, 0.35), (2, 0.5)];

pub const DOOR_DIST_3: [(u8, f32); 4] = [(0, 0.1), (1, 0.3), (2, 0.5), (3, 0.1)];

pub const ENEMY_DIST: [(u8, f32); 4] = [(0, 0.3), (1, 0.4), (2, 0.2), (3, 0.1)];

pub const DIALOG_LINE: u16 = 9;
pub const DIALOG_COLUMN: u16 = 1;

pub fn enemy_level_function(x: u16) -> f32 {
    return x.pow(2) as f32 / 5.0;
}

pub fn win_probability_function(player_level: u16, enemy_level: u16) -> f32 {
    let difference = player_level as i32 - enemy_level as i32;
    let prob = -(0.5_f32.powi(difference) / 2.0) + 1.0;
    prob.max(0.0)
}
