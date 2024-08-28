use super::consts::Position;
use rand::distributions::{Distribution, WeightedIndex};

pub fn select_random_weighted<Value>(items: &[(Value, f32)]) -> &Value {
    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
    let result = &items[dist.sample(&mut rng)].0;
    result
}

pub fn generate_enemy_position() -> Position {
    let x = (rand::random::<u8>() % 8) + 3;
    let y = (rand::random::<u8>() % 3) + 3;
    let position = Position { x, y };
    position
}
