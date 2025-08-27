use crate::grid::Node;

pub mod maze;
pub mod pathfinding;

pub type Coord = (i32, i32); // (x, y)

#[derive(PartialEq)]
pub enum AlgorithmResult {
    ModifiedGrid,
    Done(Option<Vec<Coord>>), // possible path can be returned
    Impossible
}

pub enum AlgorithmType {
    MazeGeneration,
    Pathfinding,
}

pub trait Algorithm {
    fn init(&mut self, start: Coord, end: Coord) { }
    fn step(&mut self, grid: &mut Vec<Vec<Node>>) -> AlgorithmResult;
    fn algorithm_type(&self) -> AlgorithmType;
}