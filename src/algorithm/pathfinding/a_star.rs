use std::{ collections::{ BinaryHeap, HashMap }, i32 };
use crate::{algorithm::{Algorithm, AlgorithmResult, Coord}, grid::{Node, NodeType}};

#[derive(Eq, PartialEq, Clone)]
pub struct AStarNode {
    coordinates: Coord,
    g: i32,
    h: i32,
    f: i32,
    pub parent: Option<Coord>,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // we need a min-heap, not max
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar {
    open_set: BinaryHeap<AStarNode>,
    pub nodes: HashMap<Coord, AStarNode>,
    pub end_coordinates: Coord,
}

impl AStar {
    pub fn new(start: Coord, end: Coord) -> Self {
        let mut open_set: BinaryHeap<AStarNode> = BinaryHeap::new();
        let mut nodes: HashMap<Coord, AStarNode> = HashMap::new();

        let man_dist = AStar::manhattan_distance(start, end) as i32;
        let start_node = AStarNode {
            coordinates: start,
            g: 0,
            h: man_dist,
            f: man_dist,
            parent: None,
        };

        open_set.push(start_node.clone());
        nodes.insert(start, start_node);

        Self {
            open_set: open_set,
            nodes: nodes,
            end_coordinates: end,
        }
    }

    fn reconstruct_path(&self) -> Vec<Coord> {
        let mut path = Vec::new();
        let mut current = Some(self.end_coordinates);

        while let Some(coord) = current {
            path.push(coord);
            current = self.nodes.get(&coord).and_then(|node| node.parent);
        }

        path.reverse();
        path
    }

    fn manhattan_distance(from: Coord, to: Coord) -> u32 {
        to.0.abs_diff(from.0) + to.1.abs_diff(from.1)
    }

    fn get_neighbors(&self, grid: &mut Vec<Vec<Node>>, node: &AStarNode) -> Vec<Coord> {
        let mut neighbors = Vec::new();

        let directions = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];

        let height = grid.len();
        let width = grid[0].len();

        for (dy, dx) in directions {
            let nc: Coord = (node.coordinates.0 + dx, node.coordinates.1 + dy);

            if 0 <= nc.1 && nc.1 < (height as i32) && 0 <= nc.0 && nc.0 < (width as i32) {
                if grid[nc.1 as usize][nc.0 as usize].node_type != NodeType::Wall {
                    neighbors.push(nc);
                }
            }
        }

        neighbors
    }
}

impl Algorithm for AStar {
    fn step(&mut self, grid: &mut Vec<Vec<Node>>) -> AlgorithmResult {
        if let Some(curr_node) = self.open_set.pop() {
            if curr_node.coordinates == self.end_coordinates {
                return AlgorithmResult::Done(Some(self.reconstruct_path()));
            }

            grid[curr_node.coordinates.1 as usize][curr_node.coordinates.0 as usize] = Node { node_type: NodeType::Visited };

            for neighbor in self.get_neighbors(grid, &curr_node) {
                let neighbor_node = self.nodes.entry(neighbor).or_insert(AStarNode {
                    coordinates: neighbor,
                    g: i32::MAX,
                    h: 0,
                    f: i32::MAX,
                    parent: None,
                });

                let tentative_g = curr_node.g + 1;

                if tentative_g < neighbor_node.g {
                    let h_score = AStar::manhattan_distance(neighbor, self.end_coordinates);

                    neighbor_node.g = tentative_g;
                    neighbor_node.f = tentative_g + h_score as i32;
                    neighbor_node.parent = Some(curr_node.coordinates);

                    self.open_set.push(neighbor_node.clone());
                }
            }
        }

        AlgorithmResult::Impossible
    }
}
