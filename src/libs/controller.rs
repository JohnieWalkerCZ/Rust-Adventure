use super::consts::Door;
use super::consts::Position;
use super::game::Game;
use super::room::RoomPosition;
use std::io::Stdout;
use termion::raw::RawTerminal;

pub struct PlayerController {}

impl PlayerController {
    pub fn move_up(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.y == 2
            && game.current_room.doors.contains(&Door::TOP)
            && (4..10).contains(&game.player.position.x)
        {
            let new_position = RoomPosition {
                x: game.current_room.grid_position.x,
                y: game.current_room.grid_position.y + 1,
            };
            game.enter_new_room(
                new_position,
                Door::BOTTOM,
                Position {
                    x: game.player.position.x,
                    y: 6,
                },
                stdout,
            );
        } else if game.player.position.y > 2 {
            game.move_player(0, -1, stdout);
        }
    }

    pub fn move_right(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.x == 11
            && game.current_room.doors.contains(&Door::RIGHT)
            && (3..6).contains(&game.player.position.y)
        {
            let new_position = RoomPosition {
                x: game.current_room.grid_position.x + 1,
                y: game.current_room.grid_position.y,
            };
            game.enter_new_room(
                new_position,
                Door::LEFT,
                Position {
                    x: 2,
                    y: game.player.position.y,
                },
                stdout,
            );
        } else if game.player.position.x < 11 {
            game.move_player(1, 0, stdout);
        }
    }

    pub fn move_down(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.y == 6
            && game.current_room.doors.contains(&Door::BOTTOM)
            && (4..10).contains(&game.player.position.x)
        {
            let new_position = RoomPosition {
                x: game.current_room.grid_position.x,
                y: game.current_room.grid_position.y - 1,
            };
            game.enter_new_room(
                new_position,
                Door::TOP,
                Position {
                    x: game.player.position.x,
                    y: 2,
                },
                stdout,
            );
        } else if game.player.position.y < 6 {
            game.move_player(0, 1, stdout);
        }
    }

    pub fn move_left(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.x == 2
            && game.current_room.doors.contains(&Door::LEFT)
            && (3..6).contains(&game.player.position.y)
        {
            let new_position = RoomPosition {
                x: game.current_room.grid_position.x - 1,
                y: game.current_room.grid_position.y,
            };
            game.enter_new_room(
                new_position,
                Door::RIGHT,
                Position {
                    x: 11,
                    y: game.player.position.y,
                },
                stdout,
            );
        } else if game.player.position.x > 2 {
            game.move_player(-1, 0, stdout);
        }
    }
}
