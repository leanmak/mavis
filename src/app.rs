use std::{io, rc::Rc, sync::mpsc::{Receiver, Sender}};

use ratatui::DefaultTerminal;

use crate::{algorithm::{AlgorithmResult, AlgorithmType}, event::{handle_key_press, Event}, grid::{Grid, GridState, Node, NodeType}, sidebar::Sidebar, ui::draw, utils::abs_to_grid};

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

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>, tx: Sender<Event>) -> io::Result<()> {
        let mut tick = 0;
        while !self.exit {
            match &mut self.grid.state {
                GridState::Generating(algorithm) => {
                    let tick_diff = match algorithm.borrow().algorithm_type() {
                        AlgorithmType::MazeGeneration => 50,
                        AlgorithmType::Pathfinding => 15
                    };

                    let curr_step = algorithm.borrow_mut().step(&mut self.grid.content);
                    if matches!(curr_step, AlgorithmResult::Done(_)) || matches!(curr_step, AlgorithmResult::Impossible) {
                        self.grid.state = GridState::Idle;
                        
                        if let AlgorithmResult::Done(Some(path)) = curr_step {
                            for coord in path {
                                self.grid.content[coord.1 as usize][coord.0 as usize] = Node { node_type: NodeType::Path };
                            }

                            tick = 0; // to make the terminal draw when it's done
                        }

                        // clear markers
                        if let Some(_) = self.grid.markers.start {
                            self.grid.markers.start = None;
                        }
                    }

                    if tick % tick_diff == 0 {
                        terminal.draw(|frame| draw(self, frame))?;
                    }

                    tick += 1;
                },
                GridState::PlacingMarkers(algorithm) => {
                    // hasn't placed anything yet, just skip to render terminal.
                    if self.grid.markers.start == None {
                        tx.send(Event::Empty).expect("Should be able to send empty event.");
                        tick = 0;
                    }

                    match rx.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                        Event::MousePress(position) => {
                            let Some(grid_start) = self.grid.grid_start else {
                                panic!("Grid should be initialized");
                            };

                            let Some(grid_end) = self.grid.grid_end else {
                                panic!("Grid should be initialized");
                            };

                            // out of bounds.
                            if position.0 < grid_start.0 || position.0 > grid_end.0 || position.1 < grid_start.1 || position.1 > grid_end.1 {
                                continue;
                            }

                            if self.grid.markers.start == None {
                                self.grid.markers.start = Some(position);
                            } else {
                                self.grid.markers.end = Some(position);

                                let Some(start) = self.grid.markers.start else {
                                    panic!("Start should be valid");
                                };

                                let new_algo = Rc::clone(algorithm);
                                new_algo.borrow_mut().init(abs_to_grid(start, grid_start), abs_to_grid(position, grid_start));

                                self.grid.state = GridState::Generating(new_algo);
                            }
                        }
                        _ => {},
                    };

                    if tick % 10 == 0 {
                        terminal.draw(|frame| draw(self, frame))?;
                    }
                    tick += 1;
                },
                GridState::Idle => {
                    terminal.draw(|frame| draw(self, frame))?;
                    match rx.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                        Event::KeyPress(key_code) => handle_key_press(self, key_code),
                        _ => {},
                    };
                },
            }
        }

        Ok(())
    }
}