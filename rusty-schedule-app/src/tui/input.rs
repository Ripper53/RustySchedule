use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::UserInterfaceState;

pub fn tui_input(state: &mut UserInterfaceState) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                    },
                    _ => {},
                }
            }
        }
    }
    Ok(())
}
