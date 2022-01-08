mod snake;
use crate::snake::*;

const SIZE: usize = 30;
static mut BOARD: [[CellType; SIZE]; SIZE] = [[CellType::Empty; SIZE]; SIZE];

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Snake,
    Food,
    Empty,
}
