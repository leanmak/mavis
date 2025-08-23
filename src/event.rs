use std::{ io, sync::mpsc::Sender };

use crossterm::event::{ KeyCode, KeyEventKind };

use crate::app::App;

pub enum Event {
    KeyPress(KeyCode),
}

pub fn loop_key_events(tx: Sender<Event>) -> io::Result<()> {
    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    tx.send(Event::KeyPress(key_event.code)).expect(
                        "Should be able to send key press event to receiver."
                    );
                }
            }
            _ => {}
        }
    }
}

pub fn handle_key_press(app: &mut App, key: KeyCode) {
    if key == KeyCode::Char('q') {
        app.exit = true;
    } else if key == KeyCode::Up {
        app.sidebar.prev();
    } else if key == KeyCode::Down {
        app.sidebar.next();
    } else if key == KeyCode::Enter {
        app.sidebar.select();
    }
}
