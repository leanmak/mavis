use std::{io, sync::mpsc, thread};

use crate::{app::App, event::{loop_key_events, Event}};

mod app;
mod ui;
mod event;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    
    let mut app = App::new();

    // Event-loop threads
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let key_event_tx = event_tx.clone();
    thread::spawn(|| {
        loop_key_events(key_event_tx).expect("Failed to loop events.");
    });

    let app_result = app.run(&mut terminal, event_rx);
    
    ratatui::restore();

    app_result
}
