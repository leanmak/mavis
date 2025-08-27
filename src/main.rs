use std::{io::{self, stdout}, sync::mpsc, thread};

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};

use crate::{app::App, event::{loop_key_events, Event}};

mod app;
mod ui;
mod event;
mod sidebar;
mod algorithm;
mod grid;
mod utils;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    
    // enable mouse detection
    execute!(stdout(), EnableMouseCapture)?;
    
    let mut app = App::new();

    // Event-loop threads
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let key_event_tx = event_tx.clone();
    thread::spawn(|| {
        loop_key_events(key_event_tx).expect("Failed to loop events.");
    });

    let app_result = app.run(&mut terminal, event_rx, event_tx);
    
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    app_result
}
