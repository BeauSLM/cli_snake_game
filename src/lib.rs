mod snake;
use crate::snake::*;

const SIZE: usize = 30;

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Snake,
    Food,
    Empty,
}

pub fn run() {
    let board = [[(0usize, 0usize); SIZE]; SIZE];
}
