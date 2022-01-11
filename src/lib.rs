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
    let mut square;
    let mut keys = termion::async_stdin().keys();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000 / FRAMERATE));
        square = snake.move_snake(process_key(keys.next()));
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

