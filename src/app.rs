use std::{io, sync::mpsc::Receiver};

use ratatui::DefaultTerminal;

use crate::{event::{handle_key_press, Event}, sidebar::Sidebar, ui::draw};

pub struct App {
    pub exit: bool,
    pub sidebar: Sidebar,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            sidebar: Sidebar::new()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| draw(self, frame))?;

            match rx.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                Event::KeyPress(key_code) => handle_key_press(self, key_code),
            };
        }

        Ok(())
    }
}