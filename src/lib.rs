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

// TODO: remove equality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Snake,
    Food,
    Empty,
}

pub fn setup(segments: &[(usize, usize)], food: &[(usize, usize)], dir: Direction) -> ([[CellType; SIZE]; SIZE], Snake) {
    assert!(segments.len() < CAPACITY);
    assert!(food.len() + segments.len() < SIZE * SIZE);
    let mut board = [[CellType::Empty; SIZE]; SIZE];
    for (x, y) in segments {
        let (x, y) = (*x, *y);
        assert_eq!(board[x][y], CellType::Empty);
        board[x][y] = CellType::Snake;
    }

    for (x, y) in food {
        let (x, y) = (*x, *y);
        assert_eq!(board[x][y], CellType::Empty);
        board[x][y] = CellType::Food;
    }
    let snake = Snake::new(segments, dir);
    (board, snake)
}

pub fn run() {
    let (mut board, mut snake) = setup(&[], &[], Direction::Right);
    let writer = stdout();
    let mut writer = writer.lock().into_raw_mode().unwrap();
    let mut square;
    loop {
        let mut reader = async_stdin().keys();
        sleep(Duration::from_millis(1000 / FRAMERATE));
        if let Some(key_res) = reader.last() {
            let last_key = key_res.unwrap();
            square = snake.next_square(
                match last_key {
                    Key::Up | Key::Char('w') => Some(Direction::Up),
                    Key::Down | Key::Char('s') => Some(Direction::Up),
                    Key::Left | Key::Char('a') => Some(Direction::Up),
                    Key::Right | Key::Char('d') => Some(Direction::Up),
                    Key::Char('q') => process::exit(0), // TODO: quit screen!
                    _ => None,
                }
            );
        } else { square = snake.next_square(None); }
        match board[square.0][square.1] {
            CellType::Snake => { panic!("Ate yourself!"); },
            CellType::Food => {
                snake.eat();
            },
            CellType::Empty => {
                let (x, y) = snake.old_tail();
                assert_eq!(board[x][y], CellType::Snake);
                board[x][y] = CellType::Empty;
            }
        }
        board[square.0][square.1] = CellType::Snake;
        let mut print_string = String::from("");
        for row in board.iter() {
            for cell in row {
                unimplemented!();
            }
            print_string += "\n";
        }
        write!(writer, "{}", print_string).unwrap();
        writer.flush().unwrap();
    }
}
