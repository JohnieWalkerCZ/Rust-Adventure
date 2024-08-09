use libs::controller::PlayerController;
use libs::game::Game;
use libs::ui::print_minimap;
use std::io::{stdin, stdout, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod libs;

fn main() {
    const DEBUG: bool = true;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .expect("Failed to do pres-start chores");
    stdout.flush().unwrap();

    let mut game = Game::start();

    print_minimap(&game.rooms, &game, &mut stdout);

    game.current_room
        .render_room(game.player.position.clone(), &mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc | Key::Char('q') => break,
            Key::Up => PlayerController::move_up(&mut game, &mut stdout),
            Key::Right => PlayerController::move_right(&mut game, &mut stdout),
            Key::Down => PlayerController::move_down(&mut game, &mut stdout),
            Key::Left => PlayerController::move_left(&mut game, &mut stdout),
            _ => continue,
        }
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).expect("Failed move to 1 1");
        if DEBUG {
            print_minimap(&game.rooms, &game, &mut stdout);
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).expect("");
}
