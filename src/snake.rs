use std::error;
use super::*;
const CAPACITY: usize = SIZE * SIZE - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub struct OutOfBoundsError;

impl std::fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Out of bounds!")
    }
}


impl error::Error for OutOfBoundsError {}

// this is a struct because i may have multiple snakes at some point
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

    pub(crate) fn move_snake(&mut self, dir: Option<Direction>) -> Result<(usize, usize), OutOfBoundsError> {
        let mut head = self.head;
        let dir = dir.map_or(self.dir, |d| if self.legal_turns().contains(&d) {d} else {self.dir});
        let (mut new_row, mut new_col) = self.segments[head];
        let mut out_of_bounds = false;
        match dir {
            Direction::Left => if new_col == 0 { out_of_bounds = true; } else { new_col -= 1; },
            Direction::Up => if new_row == 0 { out_of_bounds = true; } else { new_row -= 1; },
            Direction::Right => if new_col == SIZE - 1 { out_of_bounds = true; } else { new_col += 1; },
            Direction::Down => if new_row == SIZE - 1 { out_of_bounds = true; } else { new_row += 1; },
        }
        // TODO: go to loss screen
        if out_of_bounds { return Err(OutOfBoundsError.into()); }
        self.dir = dir;
        head = (head + 1) % CAPACITY;
        self.segments[head] = (new_row, new_col);
        self.head = head;
        Ok((new_row, new_col))
    }

    pub(crate) fn eat(&mut self) -> bool {
        self.len += 1;
        self.len < CAPACITY
    }

    pub(crate) fn old_tail(&self) -> (usize, usize) {
        let tail_index = if self.head < self.len {
            CAPACITY - self.len + self.head
        } else {
            self.head - self.len
        };
        self.segments[tail_index]
    }

    // TODO: remove this, its scuffed
    fn legal_turns(&self) -> [Direction; 2] {
        match self.dir {
            Direction::Left | Direction::Right => [Direction::Down, Direction::Up],
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        }
    }

    // TODO: fancy check here!
    fn is_legal_turn(&self, dir: Direction) {
        unimplemented!();
    }
}
