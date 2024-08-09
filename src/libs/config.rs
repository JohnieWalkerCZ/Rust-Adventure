type DoorDist = Vec<(u8, f32)>;
type EnemyDist = DoorDist;

pub fn get_dist_for_1_door() -> DoorDist {
    return vec![(0, 0.25), (1, 0.75)];
}

pub fn get_dist_for_2_doors() -> DoorDist {
    return vec![(0, 0.15), (1, 0.35), (2, 0.5)];
}

pub fn get_dist_for_3_doors() -> DoorDist {
    return vec![(0, 0.1), (1, 0.3), (2, 0.5), (3, 0.1)];
}

pub fn enemy_level_function(x: u16) -> u16 {
    return (x ^ 2) / 20;
}

pub fn get_dist_for_enemies() -> EnemyDist {
    return vec![(0, 0.3), (1, 0.4), (2, 0.2), (3, 0.1)];
}
