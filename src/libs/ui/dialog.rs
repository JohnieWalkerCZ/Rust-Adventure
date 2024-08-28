use crate::libs::config::{win_probability_function, DIALOG_COLUMN, DIALOG_LINE};
use crate::libs::consts::Position;
use crate::libs::enemy::Enemy;
use crate::libs::player::Player;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct Dialog {}

impl Dialog {
    fn show_dialog(content: Vec<String>, stdout: &mut RawTerminal<Stdout>) {
        if content.len() == 0 {
            return;
        }
        let len = content
            .iter()
            .map(|str| str.len() as u16)
            .collect::<Vec<u16>>()
            .into_iter()
            .max();
        if let Some(len) = len {
            Dialog::draw_top(len, stdout);
            let num_lines = content.len() as u16;
            Dialog::draw_content(content, len, num_lines, stdout);
            Dialog::draw_bottom(len, num_lines, stdout);
        }
        return;
    }

    fn draw_top(content_length: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut top = String::new();
        for _ in 0..(content_length + 4) {
            top.push_str("▒");
        }

        write!(
            stdout,
            "{}{top}",
            termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE)
        )
        .expect("Failed drawing top of dialog");
    }

    fn draw_content(
        content: Vec<String>,
        len: u16,
        num_lines: u16,
        stdout: &mut RawTerminal<Stdout>,
    ) {
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE + 1)
        )
        .expect("Failed to move to the start of first margin");
        Dialog::draw_margin(len, stdout);
        for (index, line) in content.iter().enumerate() {
            let num_padding_chars = len - line.len() as u16;
            let padding = (0..num_padding_chars).map(|_| " ").collect::<String>();
            write!(
                stdout,
                "{}▒ {line}{padding} ▒",
                termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE + 2 + index as u16)
            )
            .expect("Failed to display dialog content");
        }
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE + 2 + num_lines)
        )
        .expect("Failed to move to the start of first margin");
        Dialog::draw_margin(len, stdout);
    }

    fn draw_margin(len: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut margin = String::from("▒ ");

        for _ in 0..len {
            margin.push_str(" ");
        }

        margin.push_str(" ▒");
        write!(stdout, "{margin}").expect("Failed to draw bottom of dialog");
    }

    fn draw_bottom(content_length: u16, num_lines: u16, stdout: &mut RawTerminal<Stdout>) {
        let mut bottom = String::new();
        for _ in 0..(content_length + 4) {
            bottom.push_str("▒")
        }
        write!(
            stdout,
            "{}{bottom}",
            termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE + 2 + num_lines + 1)
        )
        .expect("Failed to draw bottom of dialog");
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
                termion::cursor::Goto(DIALOG_COLUMN, DIALOG_LINE + i as u16)
            )
            .expect("Failed to clear dialog");
        }
    }

    // Fight dialogs
    fn get_fight_dialog_content(player: &mut Player, enemy: Enemy) -> Vec<String> {
        let line1 = format!("Do you want to fight enemy on level {}", enemy.level);
        let line2 = format!(
            "Your level is {}, probability to win is {}%",
            player.level,
            win_probability_function(player.level, enemy.level) * 100.0
        );
        let line3 = String::from("Y/n");

        vec![line1, line2, line3]
    }

    pub fn show_fight_dialog(player: &mut Player, enemy: Enemy, stdout: &mut RawTerminal<Stdout>) {
        Dialog::show_dialog(Dialog::get_fight_dialog_content(player, enemy), stdout);
        player.set_fighting(true);
    }

    pub fn clear_fight_dialog(player: &mut Player, stdout: &mut RawTerminal<Stdout>) {
        let enemy = Enemy {
            level: 0,
            position: Position { x: 1, y: 1 },
        };
        Dialog::clear_dialog(Dialog::get_fight_dialog_content(player, enemy), stdout);
        player.set_fighting(false);
    }
}
