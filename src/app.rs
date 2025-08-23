use std::{io, sync::mpsc::Receiver};

use ratatui::DefaultTerminal;

use crate::{algorithm::AlgorithmResult, event::{handle_key_press, Event}, grid::{Grid, GridState}, sidebar::Sidebar, ui::draw};

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
        while !self.exit {
            terminal.draw(|frame| draw(self, frame))?;

            if let GridState::Generating(ref mut b_algorithm) = self.grid.state {
                if b_algorithm.step(&mut self.grid.content) == AlgorithmResult::Done {
                    self.grid.state = GridState::Idle;
                }
            } else {
                match rx.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                    Event::KeyPress(key_code) => handle_key_press(self, key_code),
                };
            }
        }

        Ok(())
    }
}