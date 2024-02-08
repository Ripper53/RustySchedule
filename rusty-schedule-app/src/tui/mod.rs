use std::io::{self, stdout};

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui_input::Input;

use self::input::InputReturn;

mod input;
mod render;

pub fn tui_setup() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout_handle = stdout();
    stdout_handle.execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout_handle))?;
    let mut state = UserInterfaceState::new();
    loop {
        if let Some(input) = input::tui_input(&mut state)? {
            match input {
                InputReturn::Exit => break,
            }
        }
        terminal.draw(|frame| render::render(frame, &state));
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub struct UserInterfaceState {
    tasks: Vec<Task>,
    focused_task_index: usize,
    focus_type: TaskFocus,
}

impl UserInterfaceState {
    fn new() -> Self {
        UserInterfaceState {
            tasks: vec![Task::new()],
            focused_task_index: 0,
            focus_type: TaskFocus::default(),
        }
    }
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
    pub fn selected_index(&self) -> usize {
        self.focused_task_index
    }
    pub fn get_focused_task_mut(&mut self) -> &mut Task {
        &mut self.tasks[self.focused_task_index]
    }
    pub fn add_task(&mut self) {
        self.tasks.push(Task::new());
    }
    pub fn remove_selected_task(&mut self) {
        if self.tasks.len() == 1 {
            self.tasks[0].title.reset();
            self.tasks[0].content.reset();
        } else {
            self.tasks.remove(self.focused_task_index);
        }
    }
    pub fn increment_task_selection(&mut self) {
        if self.focused_task_index < self.tasks().len() - 1 {
            self.focus_type = TaskFocus::default();
            self.focused_task_index += 1;
        }
    }
    pub fn decrement_task_selection(&mut self) {
        if self.focused_task_index != 0 {
            self.focus_type = TaskFocus::default();
            self.focused_task_index -= 1;
        }
    }
    pub fn focus_type(&self) -> TaskFocus {
        self.focus_type
    }
}

#[derive(Clone, Copy, Default)]
pub enum TaskFocus {
    #[default]
    Title,
    Content,
}

pub struct Task {
    title: Input,
    content: Input,
}

impl Task {
    fn new() -> Self {
        Task {
            title: "".into(),
            content: "".into(),
        }
    }
}
