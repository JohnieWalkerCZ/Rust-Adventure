use super::consts::Door;
use super::consts::Position;
use super::enemy;
use super::enemy::Enemy;
use super::game::Game;
use super::room::RoomPosition;
use crate::libs::ui::dialog::Dialog;
use std::io::Stdout;
use termion::raw::RawTerminal;

pub struct PlayerController {}

impl PlayerController {
    pub fn move_up(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.y == 2
            && game.current_room.doors.contains(&Door::TOP)
            && (4..10).contains(&game.player.position.x)
        {
            PlayerController::handle_no_enemy_collistion(game, stdout);
            // Entering a new room
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
            // Moving in the same room
            let new_position = Position {
                x: game.player.position.x,
                y: game.player.position.y - 1,
            };
            if let Some(enemy) = game.current_room.get_enemy_at_position(new_position) {
                PlayerController::handle_enemy_collistion(game, enemy, stdout);
                return;
            }

            PlayerController::handle_no_enemy_collistion(game, stdout);
            game.move_player(0, -1, stdout);
        }
    }

    pub fn move_right(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.x == 11
            && game.current_room.doors.contains(&Door::RIGHT)
            && (3..6).contains(&game.player.position.y)
        {
            PlayerController::handle_no_enemy_collistion(game, stdout);
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
            let new_position = Position {
                x: game.player.position.x + 1,
                y: game.player.position.y,
            };
            if let Some(enemy) = game.current_room.get_enemy_at_position(new_position) {
                PlayerController::handle_enemy_collistion(game, enemy, stdout);
                return;
            }

            PlayerController::handle_no_enemy_collistion(game, stdout);
            game.move_player(1, 0, stdout);
        }
    }

    pub fn move_down(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.y == 6
            && game.current_room.doors.contains(&Door::BOTTOM)
            && (4..10).contains(&game.player.position.x)
        {
            PlayerController::handle_no_enemy_collistion(game, stdout);
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
            let new_position = Position {
                x: game.player.position.x,
                y: game.player.position.y + 1,
            };
            if let Some(enemy) = game.current_room.get_enemy_at_position(new_position) {
                PlayerController::handle_enemy_collistion(game, enemy, stdout);
                return;
            }

            PlayerController::handle_no_enemy_collistion(game, stdout);
            game.move_player(0, 1, stdout);
        }
    }

    pub fn move_left(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        if game.player.position.x == 2
            && game.current_room.doors.contains(&Door::LEFT)
            && (3..6).contains(&game.player.position.y)
        {
            PlayerController::handle_no_enemy_collistion(game, stdout);
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
            let new_position = Position {
                x: game.player.position.x - 1,
                y: game.player.position.y,
            };
            if let Some(enemy) = game.current_room.get_enemy_at_position(new_position) {
                PlayerController::handle_enemy_collistion(game, enemy, stdout);
                return;
            }

            PlayerController::handle_no_enemy_collistion(game, stdout);
            game.move_player(-1, 0, stdout);
        }
    }

    fn handle_enemy_collistion(game: &mut Game, enemy: Enemy, stdout: &mut RawTerminal<Stdout>) {
        Dialog::clear_fight_dialog(&mut game.player, stdout);
        Dialog::show_fight_dialog(&mut game.player, enemy, stdout);
        game.fighting_enemy = Some(enemy);
    }

    fn handle_no_enemy_collistion(game: &mut Game, stdout: &mut RawTerminal<Stdout>) {
        Dialog::clear_fight_dialog(&mut game.player, stdout);
        game.fighting_enemy = None;
    }
}
