use super::consts::Door::{BOTTOM, LEFT, RIGHT, TOP};
use super::consts::{Door, Position};
use super::enemy::{self, Enemy};
use super::helper::{generate_enemy_position, select_random_weighted};
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RoomPosition {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Room {
    pub grid_position: RoomPosition,
    pub doors: Vec<Door>,
    pub enemies: Vec<Enemy>,
}

impl Room {
    pub fn new(grid_position: RoomPosition, doors: Vec<Door>) -> Room {
        Room {
            grid_position,
            doors,
            enemies: Vec::new(),
        }
    }

    pub fn add_enemies(&mut self, enemies: &mut Vec<Enemy>) {
        self.enemies.append(enemies);
    }

    pub fn render_room(&self, player_position: Position, stdout: &mut RawTerminal<Stdout>) {
        self.draw_background(stdout);
        self.draw_doors(stdout);
        self.draw_player(stdout, player_position);
        self.draw_enemies(stdout);
        stdout.flush().unwrap();
    }

    fn draw_background(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}████████████{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Goto(1, 2)
        )
        .expect("Failed to draw top");
        for r in 0..5 {
            write!(stdout, "█          █{}", termion::cursor::Goto(1, 3 + r))
                .expect("Failed to draw wall");
        }
        write!(stdout, "████████████{}", termion::cursor::Goto(1, 9))
            .expect("Failed to draw bottom");
    }

    fn draw_doors(&self, stdout: &mut RawTerminal<Stdout>) {
        for door in self.doors.clone().into_iter() {
            match door {
                Door::TOP => Result::expect(
                    write!(stdout, "{}███      ███", termion::cursor::Goto(1, 1)),
                    "Failed drawing top door",
                ),
                Door::RIGHT => Result::expect(
                    write!(
                        stdout,
                        "{} {} {} ",
                        termion::cursor::Goto(12, 3),
                        termion::cursor::Goto(12, 4),
                        termion::cursor::Goto(12, 5)
                    ),
                    "Failed drawing top door",
                ),
                Door::BOTTOM => Result::expect(
                    write!(stdout, "{}███      ███", termion::cursor::Goto(1, 7)),
                    "Failed drawing top door",
                ),
                Door::LEFT => Result::expect(
                    write!(
                        stdout,
                        "{} {} {} ",
                        termion::cursor::Goto(1, 3),
                        termion::cursor::Goto(1, 4),
                        termion::cursor::Goto(1, 5)
                    ),
                    "Failed drawing top door",
                ),
            }
        }
    }

    fn draw_player(&self, stdout: &mut RawTerminal<Stdout>, player_position: Position) {
        write!(
            stdout,
            "{}&",
            termion::cursor::Goto(player_position.x as u16, player_position.y as u16)
        )
        .expect("Failed to draw player");
    }

    fn draw_enemies(&self, stdout: &mut RawTerminal<Stdout>) {
        if self.enemies.len() == 0 {
            return;
        }

        for enemy in &self.enemies {
            let position = enemy.position;
            write!(
                stdout,
                "{}§",
                termion::cursor::Goto(position.x as u16, position.y as u16)
            )
            .expect("Failed drawing enemy");
        }
    }

    pub fn create_next_room(
        grid_position: RoomPosition,
        direction: Door,
        rooms: &mut HashMap<RoomPosition, Room>,
    ) -> Room {
        // Doors - 1 from the coming direction, next random
        let mut rng = rand::thread_rng();
        let all_directions = vec![TOP, BOTTOM, LEFT, RIGHT];

        let mut neighbour_rooms = vec![];
        let mut banned_directions = vec![];
        // Checking room on the right
        if let Some(room) = rooms.get(&RoomPosition {
            x: grid_position.x + 1,
            y: grid_position.y,
        }) {
            if room.doors.contains(&LEFT) {
                neighbour_rooms.push(RIGHT);
            } else {
                banned_directions.push(RIGHT);
            }
        }
        // Checking room on the left
        if let Some(room) = rooms.get(&RoomPosition {
            x: grid_position.x - 1,
            y: grid_position.y,
        }) {
            if room.doors.contains(&RIGHT) {
                neighbour_rooms.push(LEFT);
            } else {
                banned_directions.push(LEFT);
            }
        }
        // Checking room on the top
        if let Some(room) = rooms.get(&RoomPosition {
            x: grid_position.x,
            y: grid_position.y + 1,
        }) {
            if room.doors.contains(&BOTTOM) {
                neighbour_rooms.push(TOP);
            } else {
                banned_directions.push(TOP);
            }
        }
        // Checking room on the bottom
        if let Some(room) = rooms.get(&RoomPosition {
            x: grid_position.x,
            y: grid_position.y - 1,
        }) {
            if room.doors.contains(&TOP) {
                neighbour_rooms.push(BOTTOM);
            } else {
                banned_directions.push(BOTTOM);
            }
        }

        neighbour_rooms.push(direction);
        let neighbour_rooms_set: HashSet<_> = neighbour_rooms.clone().into_iter().collect();

        let all_directions_set: HashSet<_> = all_directions.into_iter().collect();

        let diff: Vec<_> = all_directions_set
            .difference(&neighbour_rooms_set)
            .cloned()
            .collect();

        let banned_set: HashSet<_> = banned_directions.into_iter().collect();

        let diff_set: HashSet<_> = diff.into_iter().collect();
        let result: Vec<_> = diff_set.difference(&banned_set).cloned().collect();

        let mut num_doors: u8 = 0;
        if result.len() == 1 {
            let items = vec![(0, 0.25), (1, 0.75)];
            num_doors = *select_random_weighted::<u8>(&items);
            // let items = [(0, 0.25), (1, 0.75)];
            // let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            // num_doors = items[dist.sample(&mut rng)].0;
        } else if result.len() == 2 {
            let items = vec![(0, 0.15), (1, 0.35), (2, 0.5)];
            num_doors = *select_random_weighted::<u8>(&items);
            // let items = [(0, 0.15), (1, 0.35), (2, 0.5)];
            // let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            // num_doors = items[dist.sample(&mut rng)].0;
        } else if result.len() == 3 {
            let items = vec![(0, 0.1), (1, 0.3), (2, 0.5), (3, 0.1)];
            num_doors = *select_random_weighted::<u8>(&items);
            // let items = [(0, 0.1), (1, 0.3), (2, 0.5), (3, 0.1)];
            // let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            // num_doors = items[dist.sample(&mut rng)].0;
        }

        let mut new_doors: Vec<_> = result
            .choose_multiple(&mut rng, num_doors as usize)
            .cloned()
            .collect();
        new_doors.append(&mut neighbour_rooms);

        let new_room = Room::new(grid_position, new_doors);

        // Generate enemies
        let items = vec![(0, 0.4), (1, 0.3), (2, 0.2), (3, 0.1)];
        let num_enemies = *select_random_weighted::<u8>(&items);
        if num_enemies == 0 {
            return new_room;
        }
        let enemy_positions: Vec<Position> = Vec::new();
        for i in 0..num_enemies {
            let mut position = generate_enemy_position();
            while enemy_positions.contains(&position) {
                position = generate_enemy_position();
            }

            let manhattant_distance_from_center = grid_position.x.abs() + grid_position.y.abs();
        }

        return new_room;
    }
}
