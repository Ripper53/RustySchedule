use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use super::{TaskFocus, UserInterfaceState};

pub fn tui_input<'a>(state: &mut UserInterfaceState<'a>) -> io::Result<Option<InputReturn>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            state.save()?;
                        },
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            state.add_task();
                        },
                        KeyCode::Char('d') | KeyCode::Char('D') => {
                            state.remove_selected_task();
                        },
                        KeyCode::Char('j') | KeyCode::Char('J') => {
                            state.increment_task_selection();
                        },
                        KeyCode::Char('k') | KeyCode::Char('K') => {
                            state.decrement_task_selection();
                        },
                        _ => {},
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => return Ok(Some(InputReturn::Exit)),
                        KeyCode::Enter if !key.modifiers.contains(KeyModifiers::SHIFT) => state.change_focus_type(),
                        _ => {
                            match state.focus_type() {
                                TaskFocus::Title => {
                                    let selected_task = state.get_focused_task_mut();
                                    selected_task.title.handle_event(&event);
                                },
                                TaskFocus::Content => {
                                    let selected_task = state.get_focused_task_mut();
                                    selected_task.content.input(event);
                                },
                                TaskFocus::Time => {
                                    let selected_task = state.get_focused_task_mut();
                                    selected_task.time.input(event);
                                },
                            }
                        },
                    }
                }
            }
        }
    }
    Ok(None)
}

pub enum InputReturn {
    Exit,
}
