use super::config::win_probability_function;
use super::consts::{Door, Position};
use super::enemy::{self, Enemy};
use super::game::Game;
use super::player::Player;
use super::room::{Room, RoomPosition};
use std::collections::HashMap;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct Minimap {}

impl Minimap {
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
        let (min_x, max_x, min_y, max_y) = Minimap::get_map_bounds(rooms);

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
}

pub struct Dialog {}

impl Dialog {
    fn show_dialog(content: Vec<String>, stdout: &mut RawTerminal<Stdout>) {
        if content.len() == 0 {
            return;
        }
        let len = content
            .iter()
            .map(|str| str.len())
            .collect::<Vec<usize>>()
            .into_iter()
            .max();
        if let Some(len) = len {
            Dialog::draw_top(len as u16, stdout);
            let num_lines = content.len();
            Dialog::draw_content(content, len as u16, num_lines as u16, stdout);
            Dialog::draw_bottom(len as u16, num_lines as u16, stdout);
        }
        return;
    }

    fn draw_top(content_length: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut top = String::new();
        for _ in 0..(content_length + 4) {
            top.push_str("▒");
        }

        write!(stdout, "{}{top}", termion::cursor::Goto(1, 9))
            .expect("Failed drawing top of dialog");
    }

    fn draw_content(
        content: Vec<String>,
        len: u16,
        num_lines: u16,
        stdout: &mut RawTerminal<Stdout>,
    ) {
        write!(stdout, "{}", termion::cursor::Goto(1, 9 + 1))
            .expect("Failed to move to the start of first margin");
        Dialog::draw_margin(len, stdout);
        for (index, line) in content.iter().enumerate() {
            let num_padding_chars = len - line.len() as u16;
            let padding = (0..num_padding_chars).map(|_| " ").collect::<String>();
            write!(
                stdout,
                "{}▒ {line}{padding} ▒",
                termion::cursor::Goto(1, 9 + 2 + index as u16)
            )
            .expect("Failed to display dialog content");
        }
        write!(stdout, "{}", termion::cursor::Goto(1, 9 + 2 + num_lines))
            .expect("Failed to move to the start of first margin");
        Dialog::draw_margin(len, stdout);
    }

    fn draw_margin(len: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut margin = String::from("▒ ");

        for _ in 0..len {
            margin.push_str(" ");
        }

        margin.push_str(" ▒");
        write!(stdout, "{margin}");
    }

    fn draw_bottom(content_length: u16, num_lines: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut bottom = String::new();
        for _ in 0..(content_length + 4) {
            bottom.push_str("▒")
        }
        write!(
            stdout,
            "{}{bottom}",
            termion::cursor::Goto(1, 9 + 2 + num_lines + 1)
        );
    }

    fn clear_dialog(lines: Vec<String>, stdout: &mut RawTerminal<Stdout>) {
        let num_lines = 2 + lines.len() + 2; // Top margin + content + bottom margin
        let max_line_len = lines
            .iter()
            .map(|str| str.len())
            .collect::<Vec<usize>>()
            .into_iter()
            .max()
            .expect("Failed to get max lines")
            + 4; // Acounting for margins

        let clear_line = (0..max_line_len).map(|_| " ").collect::<String>();

        for i in 0..num_lines {
            write!(
                stdout,
                "{}{clear_line}",
                termion::cursor::Goto(1, 9 + i as u16)
            );
        }
    }

    // Fight dialogs
    fn get_fight_dialog_content(player: Player, enemy: Enemy) -> Vec<String> {
        let line1 = format!("Do you want to fight enemy on level {}", enemy.level);
        let line2 = format!(
            "Your level is {}, probability to win is {}%",
            player.level,
            win_probability_function(player.level, enemy.level) * 100.0
        );
        let line3 = String::from("Y/n");

        vec![line1, line2, line3]
    }

    pub fn show_fight_dialog(player: Player, enemy: Enemy, stdout: &mut RawTerminal<Stdout>) {
        Dialog::show_dialog(Dialog::get_fight_dialog_content(player, enemy), stdout);
    }

    pub fn clear_fight_dialog(player: Player, stdout: &mut RawTerminal<Stdout>) {
        let enemy = Enemy {
            level: 0,
            position: Position { x: 1, y: 1 },
        };
        Dialog::clear_dialog(Dialog::get_fight_dialog_content(player, enemy), stdout);
    }
}
