use crate::grid::Node;

pub mod maze;

pub type Coord = (i32, i32); // (x, y)

#[derive(PartialEq)]
pub enum AlgorithmResult {
    ModifiedGrid,
    Done,
    Impossible
}

pub trait Algorithm {
    fn step(&mut self, grid: &mut Vec<Vec<Node>>) -> AlgorithmResult;
}