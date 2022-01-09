use super::*;
pub const CAPACITY: usize = 500;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Snake {
    segments: [(usize, usize); CAPACITY],
    head: usize,
    len: usize,
    dir: Direction,
}

impl Snake {
    pub fn new(starting_snake: &[(usize, usize)], dir: Direction) -> Snake {
        let mut segments = [(0usize, 0usize); CAPACITY];
        let len = starting_snake.len();
        let head = starting_snake.len() - 1;
        for (st, seg) in starting_snake.into_iter().zip(segments.iter_mut()) {
            *seg = *st;
        }
        Snake {
            segments,
            head,
            len,
            dir,
        }
    }

    pub fn next_square(&mut self, dir: Option<Direction>) -> (usize, usize) {
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
        self.dir = dir;
        head = (self.head + 1) % CAPACITY;
        self.segments[head] = (new_head_x, new_head_y);
        (new_head_x, new_head_y)
    }

    pub fn eat(&mut self) {
        self.len += 1;
    }

    pub fn old_tail(&self) -> (usize, usize) {
        let tail_index = if self.head < self.len {
            CAPACITY - self.len + self.head
        } else {
            self.head - self.len
        };
        self.segments[tail_index]
    }

    fn legal_turns(dir: Direction) -> (Direction, Direction) {
        match dir {
            Direction::Left | Direction::Right => (Direction::Down, Direction::Up),
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
        }
    }
}
