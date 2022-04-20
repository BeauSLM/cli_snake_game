use std::error;
use rand::distributions::{DistIter, Uniform};
use rand::prelude::{Distribution, ThreadRng};

use crate::snake::*;
use crate::io::*;
mod snake;
pub mod io;

const SIZE: usize = 20;
const MIDDLE: usize = SIZE / 2;
const FRAMERATE: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Snake,
    Food,
    Empty,
}

#[derive(Debug)]
pub(crate) struct BumpedTailError;

impl std::fmt::Display for BumpedTailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bumped your tail!")
    }
}

impl error::Error for BumpedTailError {}

fn setup(rng: &mut DistIter<Uniform<usize>, ThreadRng, usize>) -> ([[CellType; SIZE]; SIZE], Snake) {
    let mut board = [[CellType::Empty; SIZE]; SIZE];
    let starting_snake = [(MIDDLE, MIDDLE), (MIDDLE, MIDDLE + 1)];
    let dir = Direction::Right;
    for (row, col) in starting_snake {
        board[row][col] = CellType::Snake;
    }

    generate_food(&mut board, rng);
    let snake = Snake::new(&starting_snake, dir);
    (board, snake)
}

pub fn run() {
    // initialize variables
    let mut rng = Uniform::new_inclusive(0usize, SIZE - 1).sample_iter(rand::thread_rng());
    let (mut board, mut snake) = setup(&mut rng);
    let writer = stdout();
    let mut writer = writer.lock().into_raw_mode().unwrap();
    write!(writer, "{}", termion::cursor::Hide).unwrap();
    let mut score = 0u16;
    let mut keys = termion::async_stdin().keys();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000 / FRAMERATE));
        // TODO: remove row, col, use a mut tuple to allocate once instead
        // TODO: handle moving the snake entirely in the snake - if you can
        let (row, col) = match snake.move_snake(process_key(keys.next())) {
            Ok(cell) => cell,
            Err(e) => {
                game_over(e.into(), &mut writer, score);
                unreachable!()
            }
        };
        match board[row][col] {
            CellType::Snake => { 
                game_over(BumpedTailError.into(), &mut writer, score);
            },
            CellType::Food => {
                if !snake.eat() { victory(&mut writer); }
                score += 1;
                generate_food(&mut board, &mut rng);
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

fn generate_food(board: &mut [[CellType; SIZE]; SIZE] ,rng: &mut DistIter<Uniform<usize>, ThreadRng, usize>) {
    let mut placed = false;
    let mut cell: (usize, usize);
    while !placed {
        cell = (rng.next().unwrap(), rng.next().unwrap());
        if CellType::Snake != board[cell.0][cell.1] {
            board[cell.0][cell.1] = CellType::Food;
            placed = true;
        }

        cell = (rng.next().unwrap(), rng.next().unwrap());
        if CellType::Snake != board[cell.0][cell.1] {
            board[cell.0][cell.1] = CellType::Food;
            placed = true;
        }
    }
}
