use std::collections::HashMap;
use std::io::Write;
use termion::raw::RawTerminal;

use super::consts::{Door, Position};
use super::player::Player;
use super::room::{Room, RoomPosition};
use std::io::Stdout;

pub struct Game {
    pub rooms: HashMap<RoomPosition, Room>,
    pub player: Player,
    pub current_room: Room,
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
        }
    }

    pub fn enter_new_room(
        &mut self,
        new_position: RoomPosition,
        entry_door: Door,
        player_new_pos: Position,
        stdout: &mut RawTerminal<Stdout>,
    ) {
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
}
