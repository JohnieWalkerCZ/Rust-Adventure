use rand::distributions::{Distribution, WeightedIndex};

use super::consts::Position;

pub fn select_random_weighted<Value>(items: &Vec<(Value, f32)>) -> &Value {
    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
    let result = &items[dist.sample(&mut rng)].0;
    result
}

pub fn generate_enemy_position() -> Position {
    let x = (rand::random::<u8>() % 10) + 1;
    let y = (rand::random::<u8>() % 3) + 1;
    let position = Position { x, y };
    position
}
