use super::config::win_probability_function;
use super::consts::{Door, Position};
use super::enemy::{self, Enemy};
use super::player::Player;
use super::room::{Room, RoomPosition};
use rand::seq::index;
use rand::Rng;
use termion::cursor::SteadyBlock;
use std::collections::HashMap;
use std::io::Stdout;
use std::io::Write;
use termion::raw::RawTerminal;

pub struct Game {
    pub rooms: HashMap<RoomPosition, Room>,
    pub player: Player,
    pub current_room: Room,
    pub fighting_enemy: Option<Enemy>,
}

impl Game {
    pub fn start() -> Game {
        let current_room = Room::new(
            RoomPosition { x: 0, y: 0 },
            vec![Door::TOP, Door::RIGHT, Door::BOTTOM, Door::LEFT],
        );
        let mut rooms = HashMap::new();
        rooms.insert(RoomPosition { x: 0, y: 0 }, current_room.clone());
        Game {
            rooms,
            player: Player::new(Position { x: 5, y: 3 }),
            current_room,
            fighting_enemy: None,
        }
    }

    pub fn enter_new_room(
        &mut self,
        new_position: RoomPosition,
        entry_door: Door,
        player_new_pos: Position,
        stdout: &mut RawTerminal<Stdout>,
    ) {
        self.save_current_room();
        if let Some(room) = self.rooms.get(&new_position) {
            self.current_room = room.clone();
        } else {
            let new_room = Room::create_next_room(new_position, entry_door, &mut self.rooms);
            self.rooms.insert(new_position, new_room.clone());
            self.current_room = new_room;
        }
        self.player.position.x = player_new_pos.x as u8;
        self.player.position.y = player_new_pos.y as u8;
        self.current_room.render_room(self.player.position, stdout);
    }

    pub fn move_player(&mut self, dx: i16, dy: i16, stdout: &mut RawTerminal<Stdout>) {
        self.clear_character(stdout);
        self.player.position.x = (self.player.position.x as i16 + dx) as u8;
        self.player.position.y = (self.player.position.y as i16 + dy) as u8;
        self.draw_player(stdout);
    }

    pub fn move_player_direct(&mut self, position: Position, stdout: &mut RawTerminal<Stdout>) {
        self.clear_character(stdout);
        self.player.position = position;
        self.draw_player(stdout);
    }

    pub fn save_current_room(&mut self) {
        self.rooms
            .insert(self.current_room.grid_position, self.current_room.clone());
    }

    fn clear_character(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{} ",
            termion::cursor::Goto(self.player.position.x as u16, self.player.position.y as u16)
        )
        .expect("Failed to clear character");
    }

    fn draw_player(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}&",
            termion::cursor::Goto(self.player.position.x as u16, self.player.position.y as u16)
        )
        .expect("Failed to draw player");
    }

    pub fn fight(&mut self, stdout: &mut RawTerminal<Stdout>) {
        match self.fighting_enemy {
            None => return,
            Some(enemy) => {
                let probability = win_probability_function(self.player.level, enemy.level);

                let mut rng = rand::thread_rng();
                let win = rng.gen_bool(probability as f64);
                if win {
                    self.win(enemy, stdout);
                    self.player.fighting = false;
                    return;
                }
                self.lose(enemy.level);
            }
        }
    }

    fn win(&mut self, enemy: Enemy, stdout: &mut RawTerminal<Stdout>) {
        self.player.level += enemy.level;
        write!(
            stdout,
            "{} ",
            termion::cursor::Goto(enemy.position.x as u16, enemy.position.y as u16)
        );
        self.move_player_direct(enemy.position, stdout);

        let index = self
            .current_room
            .enemies
            .iter()
            .position(|x| *x == enemy)
            .unwrap();
        self.current_room.enemies.remove(index);
    }

    fn lose(&mut self, enemy_level: u16) {
        let difference: i16 = (self.player.level as i16 - enemy_level as i16 + 1).abs();
        self.player.decrease_health(difference as u16);
    }
}
