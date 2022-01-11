use std::error;
use crate::snake::*;
use crate::io::*;
mod snake;
pub mod io;

const SIZE: usize = 30;
const MIDDLE: usize = SIZE / 2;
const FRAMERATE: u64 = 2;

#[derive(Debug, Clone, Copy)]
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

pub fn run() {
    let (mut board, mut snake) = setup(&[(MIDDLE, MIDDLE), (MIDDLE, MIDDLE + 1)], &[(2, 4), (2, 5)], Direction::Right);
    let writer = stdout();
    let mut writer = writer.lock().into_raw_mode().unwrap();
    write!(writer, "{}", termion::cursor::Hide).unwrap();
    let mut keys = termion::async_stdin().keys();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000 / FRAMERATE));
        // TODO: remove row, col, use a mut tuple to allocate once instead
        let (row, col) = match snake.move_snake(process_key(keys.next())) {
            Ok(cell) => cell,
            Err(e) => {
                end_screen(e.into(), &mut writer);
                panic!("Shouldn't be reachable");
            }
        };
        match board[row][col] {
            CellType::Snake => { 
                end_screen(BumpedTailError.into(), &mut writer);
            },
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

