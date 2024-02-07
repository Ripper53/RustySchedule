use std::io::{self, stdout};

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui_input::Input;

mod input;
mod render;

pub fn tui_setup() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    loop {
        let mut state = UserInterfaceState::new();
        input::tui_input(&mut state)?;
        terminal.draw(|frame| render::render(frame, state));
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

pub struct UserInterfaceState {
    input: Input,
}

impl UserInterfaceState {
    fn new() -> Self {
        UserInterfaceState {
            input: Input::default(),
        }
    }
}
