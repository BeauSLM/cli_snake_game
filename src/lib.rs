use termion::{async_stdin, color, style};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Read, Write, stdout};
use std::process;
use std::thread::sleep;
use std::time::Duration;

use crate::snake::*;

mod snake;

const SIZE: usize = 30;
const FRAMERATE: u64 = 2;

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Snake,
    Food,
    Empty,
}

pub fn run() {
    let mut board = [[(CellType::Empty); SIZE]; SIZE];
    // let mut snake = Snake::new(&mut board);
    let writer = stdout();
    let mut writer = writer.lock().into_raw_mode().unwrap();
    loop {
        let mut reader = async_stdin().keys();
        sleep(Duration::from_millis(1000 / FRAMERATE));
        if let Some(key_res) = reader.last() {
            let last_key = key_res.unwrap();
            // snake.move_snake(
            //     match last_key {
            //         Key::Up | Key::Char('w') => Some(Direction::Up),
            //         Key::Down | Key::Char('s') => Some(Direction::Up),
            //         Key::Left | Key::Char('a') => Some(Direction::Up),
            //         Key::Right | Key::Char('d') => Some(Direction::Up),
            //         Key::Char('q') => process::exit(0), // TODO: quit screen!
            //         _ => None,
            //     }
            // );
        }
        let mut print_string = String::from("");
        for row in board.iter() {
            for cell in row {
                let push_char = match cell {
                    CellType::Snake => format!("{}*", color::Fg(color::Green)).as_str(),
                    CellType::Food => format!("{}0", color::Fg(color::Green)).as_str(),
                    _ => &format!("{} ", style::Reset).to_owned(),
                };
            }
            print_string += "\n";
        }
        write!(writer, "{}", print_string);
        writer.flush().unwrap();
    }
}
