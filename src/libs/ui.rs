use super::consts::Door;
use super::enemy::Enemy;
use super::game::Game;
use super::player::Player;
use super::room::{Room, RoomPosition};
use std::collections::HashMap;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub fn print_minimap(
    rooms: &HashMap<RoomPosition, Room>,
    game: &Game,
    stdout: &mut RawTerminal<Stdout>,
) {
    if rooms.is_empty() {
        write!(stdout, "{}Empty map", termion::cursor::Goto(15, 1))
            .expect("Failed to move to start of map");
        return;
    }
    let mut curr_line = 2;
    let (min_x, max_x, min_y, max_y) = get_map_bounds(rooms);

    for y in (min_y..=max_y).rev() {
        // Print rooms and horizontal connections
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = RoomPosition { x, y };
            if let Some(room) = rooms.get(&pos) {
                if room == &game.current_room {
                    line.push_str("█");
                } else {
                    line.push_str("░");
                }

                let doors = &room.doors;
                if x < max_x
                    && doors.contains(&Door::RIGHT)
                    && rooms
                        .get(&RoomPosition { x: x + 1, y })
                        .map_or(false, |r| r.doors.contains(&Door::LEFT))
                {
                    // print!("↔");
                    line.push_str("-");
                } else {
                    line.push_str(" ");
                }
            } else {
                line.push_str("  ");
            }
        }
        write!(stdout, "{}{line}", termion::cursor::Goto(15, curr_line))
            .expect("Failed to move to next line");
        curr_line += 1;
        line = String::new();
        // Print vertical connections
        if y > min_y {
            for x in min_x..=max_x {
                let pos = RoomPosition { x, y };
                let pos_below = RoomPosition { x, y: y - 1 };
                if rooms
                    .get(&pos)
                    .map_or(false, |r| r.doors.contains(&Door::BOTTOM))
                    && rooms
                        .get(&pos_below)
                        .map_or(false, |r| r.doors.contains(&Door::TOP))
                {
                    line.push_str("| ");
                } else {
                    line.push_str("  ");
                }
            }
            write!(stdout, "{}{line}", termion::cursor::Goto(15, curr_line))
                .expect("Failed to move to next line");
            curr_line += 1;
        }
    }
}

fn get_map_bounds(rooms: &HashMap<RoomPosition, Room>) -> (i8, i8, i8, i8) {
    let mut min_x = i8::MAX;
    let mut max_x = i8::MIN;
    let mut min_y = i8::MAX;
    let mut max_y = i8::MIN;

    for &position in rooms.keys() {
        let x = position.x;
        let y = position.y;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    (min_x, max_x, min_y, max_y)
}

pub fn show_fight_dialog(player: Player, enemy: Enemy, stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}AAAAA", termion::cursor::Goto(1, 9));
}
