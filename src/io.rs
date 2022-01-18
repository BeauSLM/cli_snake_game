use crate::*;
pub(crate) use termion::raw::{IntoRawMode, RawTerminal};
pub(crate) use std::io::{Write, StdoutLock, stdout};
pub(crate) use termion::input::TermRead;
use termion::color;
use termion::event::Key;

// This might come in handy later
/* const KEYS: [Key; 8] = [
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Char('w'),
    Key::Char('s'),
    Key::Char('a'),
    Key::Char('d'),
]; */

pub fn start_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // Please note that I used an ascii generator, I'm shameless
    println!(
        r#"
        _________              __              ________                       
        /   _____/ ____ _____  |  | __ ____    /  _____/_____    _____   ____  
        \_____  \ /    \\__  \ |  |/ // __ \  /   \  ___\__  \  /     \_/ __ \ 
        /        \   |  \/ __ \|    <\  ___/  \    \_\  \/ __ \|  Y Y  \  ___/ 
        /_______  /___|  (____  /__|_ \\___  >  \______  (____  /__|_|  /\___  >
                \/     \/     \/     \/    \/          \/     \/      \/     \/ 
        By Beau McCartney"#
    );
    std::thread::sleep(std::time::Duration::from_secs(2));
}

pub(crate) fn process_key(key: Option<Result<Key, std::io::Error>>) -> Option<Direction> {
    match key.map(|dir| dir.unwrap()) {
        Some(Key::Up) | Some(Key::Char('w')) => Some(Direction::Up),
        Some(Key::Down) | Some(Key::Char('s')) => Some(Direction::Down),
        Some(Key::Left) | Some(Key::Char('a')) => Some(Direction::Left),
        Some(Key::Right) | Some(Key::Char('d')) => Some(Direction::Right),
        Some(Key::Char('q')) => std::process::exit(0), // TODO: quit screen!
        _ => None,
    }
}

pub(crate) fn display(board: &[[CellType; SIZE]; SIZE], writer: &mut RawTerminal<StdoutLock>) {
    write!(writer, "{} {}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for row in board {
        for cell in row {
            // XXX: write to the buffer with a variable somehow please god
            match cell {
                &CellType::Snake => write!(writer, "{}*{}", color::Fg(color::Green), termion::style::Reset).unwrap(),
                &CellType::Food => write!(writer, "{}0{}", color::Fg(color::Red), termion::style::Reset).unwrap(),
                _ => write!(writer, " ").unwrap(),
            };
        }
        write!(writer, "\r\n").unwrap();
    }
    writer.flush().unwrap();
}

pub fn game_over(status: Box<dyn error::Error>, writer: &mut RawTerminal<StdoutLock>, score: u16) {
    writer.suspend_raw_mode().unwrap();
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // Please note that I used an ascii generator, I'm shameless
    println!(
        r#" 
        ________                        ________                     
       /  _____/_____    _____   ____   \_____  \___  __ ___________ 
      /   \  ___\__  \  /     \_/ __ \   /   |   \  \/ // __ \_  __ \
      \    \_\  \/ __ \|  Y Y  \  ___/  /    |    \   /\  ___/|  | \/
       \______  (____  /__|_|  /\___  > \_______  /\_/  \___  >__|   
              \/     \/      \/     \/          \/          \/       "#
    );

    println!("{}", status);
    println!("Score: {}", score);
    std::thread::sleep(std::time::Duration::from_secs(2));
    std::process::exit(0);
}

pub fn victory(writer: &mut RawTerminal<StdoutLock>) {
    writer.suspend_raw_mode().unwrap();
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // Please note that I used an ascii generator, I'm shameless
    println!(
        r#"
           ____   ____.__        __                       ._. 
           \   \ /   /|__| _____/  |_  ___________ ___.__.| | 
            \   Y   / |  |/ ___\   __\/  _ \_  __ <   |  || | 
             \     /  |  \  \___|  | (  <_> )  | \/\___  | \| 
              \___/   |__|\___  >__|  \____/|__|   / ____| __ 
                              \/                   \/      \/ "#
    );

    std::thread::sleep(std::time::Duration::from_secs(2));
    std::process::exit(0);
}
