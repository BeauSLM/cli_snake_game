use termion::{async_stdin};
pub use termion::{clear, style, color, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, StdoutLock};
use std::process;
pub use std::thread::sleep;
pub use std::time::Duration;

use crate::snake::*;

mod snake;

const SIZE: usize = 30;
const MIDDLE: usize = SIZE / 2;
const FRAMERATE: u64 = 2;

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

// TODO: remove equality
#[derive(Debug, Clone, Copy)]
enum CellType {
    Snake,
    Food,
    Empty,
}

fn setup(segments: &[(usize, usize)], food: &[(usize, usize)], dir: Direction) -> ([[CellType; SIZE]; SIZE], Snake) {
    assert!(food.len() + segments.len() < SIZE * SIZE);
    let mut board = [[CellType::Empty; SIZE]; SIZE];
    for (row, col) in segments {
        let (row, col) = (*row, *col);
        board[row][col] = CellType::Snake;
    }

    for (row, col) in food {
        let (row, col) = (*row, *col);
        board[row][col] = CellType::Food;
    }
    let snake = Snake::new(segments, dir);
    (board, snake)
}

// XXX: lord almighty this needs to be split up
pub fn run() {
    let (mut board, mut snake) = setup(&[(MIDDLE, MIDDLE), (MIDDLE, MIDDLE + 1)], &[(2, 4), (2, 5)], Direction::Right);
    let writer = stdout();
    let mut writer = writer.lock().into_raw_mode().unwrap();
    write!(writer, "{}", cursor::Hide).unwrap();
    let mut square;
    let mut reader = async_stdin().keys();
    loop {
        sleep(Duration::from_millis(1000 / FRAMERATE));
        square = snake.move_snake(
            match reader.next().map(|dir| dir.unwrap()) {
            Some(Key::Up) | Some(Key::Char('w')) => Some(Direction::Up),
            Some(Key::Down) | Some(Key::Char('s')) => Some(Direction::Down),
            Some(Key::Left) | Some(Key::Char('a')) => Some(Direction::Left),
            Some(Key::Right) | Some(Key::Char('d')) => Some(Direction::Right),
            Some(Key::Char('q')) => process::exit(0), // TODO: quit screen!
            _ => None,
        });
        let (row, col) = square;
        match board[row][col] {
            CellType::Snake => { panic!("Ate yourself!"); },
            CellType::Food => {
                snake.eat();
            },
            CellType::Empty => {
                let (row, col) = snake.old_tail();
                board[row][col] = CellType::Empty;
            }
        }
        board[row][col] = CellType::Snake;
        display(&board, &mut writer);
    }
}

fn display(board: &[[CellType; SIZE]; SIZE], writer: &mut RawTerminal<StdoutLock>) {
    write!(writer, "{} {}", clear::All, cursor::Goto(1, 1)).unwrap();
    for row in board {
        for cell in row {
            // XXX: write to the buffer with a variable somehow please god
            match cell {
                &CellType::Snake => write!(writer, "{}*{}", color::Fg(color::Green), style::Reset).unwrap(),
                &CellType::Food => write!(writer, "{}0{}", color::Fg(color::Red), style::Reset).unwrap(),
                _ => write!(writer, " ").unwrap(),
            };
        }
        write!(writer, "\r\n").unwrap();
    }
    writer.flush().unwrap();
}

