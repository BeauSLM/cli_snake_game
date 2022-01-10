use super::*;
const CAPACITY: usize = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub(crate) struct Snake {
    segments: [(usize, usize); CAPACITY],
    head: usize,
    len: usize,
    dir: Direction,
}

impl Snake {
    pub(crate) fn new(starting_snake: &[(usize, usize)], dir: Direction) -> Snake {
        assert!(starting_snake.len() < CAPACITY);
        let mut segments = [(0usize, 0usize); CAPACITY];
        let len = starting_snake.len();
        let head = len - 1;
        for (seg, start) in segments.iter_mut().zip(starting_snake) {
            *seg = *start;
        }
        Snake {
            segments,
            head,
            len,
            dir,
        }
    }

    pub(crate) fn next_square(&mut self, dir: Option<Direction>) -> (usize, usize) {
        let mut head = self.head;
        let mut dir = if let Some(d) = dir { d } else { self.dir };
        if !self.legal_turns().contains(&dir) { dir = self.dir; }
        let (mut new_row, mut new_col) = self.segments[head];
        let mut out_of_bounds = false;
        match dir {
            Direction::Left => if new_col == 0 { out_of_bounds = true; } else { new_col -= 1; },
            Direction::Up => if new_row == 0 { out_of_bounds = true; } else { new_row -= 1; },
            Direction::Right => if new_col == SIZE - 1 { out_of_bounds = true; } else { new_col += 1; },
            Direction::Down => if new_row == SIZE - 1 { out_of_bounds = true; } else { new_row += 1; },
        }
        if out_of_bounds { panic!("Out of bounds!"); }
        self.dir = dir;
        head = (head + 1) % CAPACITY;
        self.segments[head] = (new_row, new_col);
        self.head = head;
        (new_row, new_col)
    }

    pub(crate) fn eat(&mut self) {
        self.len += 1;
    }

    pub(crate) fn old_tail(&self) -> (usize, usize) {
        let tail_index = if self.head < self.len {
            CAPACITY - self.len + self.head
        } else {
            self.head - self.len
        };
        self.segments[tail_index]
    }

    fn legal_turns(&self) -> [Direction; 2] {
        match self.dir {
            Direction::Left | Direction::Right => [Direction::Down, Direction::Up],
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        }
    }
}
