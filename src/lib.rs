use termion::{async_stdin, color, style, clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, StdoutLock};
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
    assert!(food.len() + segments.len() < SIZE * SIZE);
    let mut board = [[CellType::Empty; SIZE]; SIZE];
    for (row, col) in segments {
        let (row, col) = (*row, *col);
        assert_eq!(board[row][col], CellType::Empty);
        board[row][col] = CellType::Snake;
    }

    for (row, col) in food {
        let (row, col) = (*row, *col);
        assert_eq!(board[row][col], CellType::Empty);
        board[row][col] = CellType::Food;
    }
    let snake = Snake::new(segments, dir);
    (board, snake)
}

pub fn run() {
    let (mut board, mut snake) = setup(&[(0, 0), (0, 1)], &[(0, 4)], Direction::Right);
    // let writer = stdout();
    // let mut writer = writer.lock().into_raw_mode().unwrap();
    let mut square;
    loop {
        let mut reader = async_stdin().keys();
        sleep(Duration::from_millis(1000 / FRAMERATE));
        if let Some(key_res) = reader.last() {
            let last_key = key_res.unwrap();
            square = snake.next_square(
                match last_key {
                    Key::Up | Key::Char('w') => Some(Direction::Up),
                    Key::Down | Key::Char('s') => Some(Direction::Down),
                    Key::Left | Key::Char('a') => Some(Direction::Left),
                    Key::Right | Key::Char('d') => Some(Direction::Right),
                    Key::Char('q') => process::exit(0), // TODO: quit screen!
                    _ => None,
                }
            );
        } else { square = snake.next_square(None); }
        let (row, col) = square;
        match board[row][col] {
            CellType::Snake => { panic!("Ate yourself!"); },
            CellType::Food => {
                snake.eat();
            },
            CellType::Empty => {
                let (row, col) = snake.old_tail();
                assert_eq!(board[row][col], CellType::Snake);
                board[row][col] = CellType::Empty;
            }
        }
        board[square.0][square.1] = CellType::Snake;
        display(&board);
        // write!(writer, "{}", print_string).unwrap();
        // writer.flush().unwrap();
    }
}

pub fn display(board: &[[CellType; SIZE]; SIZE]) {
    println!("{}", clear::All);
    let mut print_string = String::with_capacity(SIZE);
    for row in board.iter() {
        for col in row {
            print_string.push(match col {
                &CellType::Snake => '*',
                &CellType::Food => '0',
                _ => ' '
            });
        }
        println!("{}", print_string);
        print_string.clear();
    }
}

