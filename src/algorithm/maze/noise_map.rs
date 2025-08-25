use crate::{algorithm::{Algorithm, AlgorithmResult, AlgorithmType, Coord}, grid::{Node, NodeType}};
use rand::prelude::*;

pub struct NoiseMap {
    next: Coord,
    fill_percentage: i32,
    rng: ThreadRng,
}

impl NoiseMap {
    pub fn new(f: i32) -> Self {
        Self {
            next: (0, 0),
            fill_percentage: f,
            rng: rand::rng()
        }
    }
}

impl Algorithm for NoiseMap {
    fn step(&mut self, grid: &mut Vec<Vec<Node>>) -> AlgorithmResult {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let node = &mut grid[self.next.1 as usize][self.next.0 as usize];

        if self.rng.random_range(0..=100) <= self.fill_percentage {
            node.node_type = NodeType::Wall;
        } else {
            node.node_type = NodeType::Empty;
        }

        if self.next.0 == width-1 && self.next.1 == height-1 {
            AlgorithmResult::Done(None)
        } else {
            let x: i32;
            let y: i32;

            if self.next.0 == width-1 {
                x = 0;
                y = self.next.1 + 1;
            } else {
                x = self.next.0 + 1;
                y = self.next.1;
            }

            self.next = (x, y);
            AlgorithmResult::ModifiedGrid
        }
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::MazeGeneration
    }
}