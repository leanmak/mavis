use std::{io, sync::mpsc::Receiver};

use ratatui::DefaultTerminal;

use crate::{algorithm::{AlgorithmResult, AlgorithmType}, event::{handle_key_press, Event}, grid::{Grid, GridState, Node, NodeType}, sidebar::Sidebar, ui::draw};

pub struct App {
    pub exit: bool,
    pub sidebar: Sidebar,
    pub grid: Grid,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            sidebar: Sidebar::new(),
            grid: Grid::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> io::Result<()> {
        let mut tick = 0;
        while !self.exit {
            if let GridState::Generating(b_algorithm) = &mut self.grid.state {    
                // somehow increasing the tick difference makes the entire thing go faster.
                let tick_diff = match b_algorithm.borrow().algorithm_type() {
                    AlgorithmType::MazeGeneration => 50,
                    AlgorithmType::Pathfinding => 15
                };
                
                let curr_step = b_algorithm.borrow_mut().step(&mut self.grid.content);
                if matches!(curr_step, AlgorithmResult::Done(_)) || matches!(curr_step, AlgorithmResult::Impossible) {
                    self.grid.state = GridState::Idle;

                    if let AlgorithmResult::Done(Some(path)) = curr_step {
                        for coord in path {
                            self.grid.content[coord.1 as usize][coord.0 as usize] = Node { node_type: NodeType::Path };
                        }

                        tick = 0; // THIS WAS THE ONLY WAY.
                    }
                }

                if tick % tick_diff == 0 {
                    terminal.draw(|frame| draw(self, frame))?;
                }
                tick += 1;
            } else {
                terminal.draw(|frame| draw(self, frame))?;
                match rx.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                    Event::KeyPress(key_code) => handle_key_press(self, key_code),
                };
            }
        }

        Ok(())
    }
}