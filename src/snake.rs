use super::*;
const CAPACITY: usize = 500;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Snake<'game> {
    segments: [(usize, usize); CAPACITY],
    head: usize,
    len: usize,
    dir: Direction,
    board: &'game mut [[CellType; SIZE]; SIZE],
}

impl<'game> Snake<'game> {
    pub fn new(board: &'game mut [[CellType; SIZE]; SIZE]) -> Snake<'game> {
        let starting_snake = [(0usize, 0usize), (1usize, 0usize)];
        let dir = Direction::Right;
        let mut segments = [(0usize, 0usize); CAPACITY];
        let len = starting_snake.len();
        let head = starting_snake.len() - 1;
        for (st, seg) in starting_snake.into_iter().zip(segments.iter_mut()) {
            *seg = st;
            board[st.0][st.1] = CellType::Snake;
        }
        Snake {
            segments,
            head,
            len,
            dir,
            board,
        }
    }

    pub fn move_snake(&mut self, dir: Option<Direction>) {
        let mut head = self.head;
        let dir = if let Some(d) = dir { d } else { self.dir };
        let (mut new_head_x, mut new_head_y) = self.segments[head];
        let mut out_of_bounds = false;
        match dir {
            Direction::Left => if new_head_x == 0 { out_of_bounds = true; } else { new_head_x -= 1; },
            Direction::Up => if new_head_y == 0 { out_of_bounds = true; } else { new_head_y -= 1; },
            Direction::Right => if new_head_x == SIZE - 1 { out_of_bounds = true; } else { new_head_x += 1; },
            Direction::Down => if new_head_y == SIZE - 1 { out_of_bounds = true; } else { new_head_y += 1; },
        }
        if out_of_bounds { panic!("Out of bounds!"); }
        match self.board[new_head_x][new_head_y] {
            CellType::Snake => { panic!("Ate yourself!"); },
            CellType::Food => { self.len += 1; },
            CellType::Empty => {
                // clean out tail
                let tail_index = if head < self.len { CAPACITY - self.len + head } else { head - self.len };
                let (tail_x, tail_y) = self.segments[tail_index];
                self.board[tail_x][tail_y] = CellType::Empty;
            }
        }
        self.board[new_head_x][new_head_y] = CellType::Snake;
        self.dir = dir;
        head = (self.head + 1) % CAPACITY;
        self.segments[head] = (new_head_x, new_head_y);
    }

    fn legal_turns(dir: Direction) -> (Direction, Direction) {
        match dir {
            Direction::Left | Direction::Right => (Direction::Down, Direction::Up),
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
        }
    }
}
