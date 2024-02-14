use std::{io::{self, stdout}, str::FromStr};

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use directories::ProjectDirs;
use ratatui::{backend::CrosstermBackend, Terminal};
use rusty_schedule_core::{NotifierBuilder, Reminder};
use tui_input::Input;
use chrono::NaiveTime;
use tui_textarea::TextArea;

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
        terminal.draw(|frame| render::render(frame, &mut state))?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub struct UserInterfaceState<'a> {
    tasks: Vec<Task<'a>>,
    focused_task_index: usize,
    focus_type: TaskFocus,
}

impl<'a> UserInterfaceState<'a> {
    fn new() -> Self {
        UserInterfaceState {
            tasks: vec![Task::new()],
            focused_task_index: 0,
            focus_type: TaskFocus::default(),
        }
    }
    pub fn tasks_mut(&mut self) -> impl Iterator<Item = &mut Task<'a>> {
        self.tasks.iter_mut()
    }
    pub fn tasks_count(&self) -> usize {
        self.tasks.len()
    }
    pub fn selected_index(&self) -> usize {
        self.focused_task_index
    }
    pub fn get_focused_task_mut(&mut self) -> &mut Task<'a> {
        &mut self.tasks[self.focused_task_index]
    }
    pub fn add_task(&mut self) {
        self.tasks.push(Task::new());
    }
    pub fn remove_selected_task(&mut self) {
        if self.tasks.len() == 1 {
            self.tasks[0].title.reset();
            self.tasks[0].content.select_all();
            self.tasks[0].content.cut();
            self.tasks[0].time.select_all();
            self.tasks[0].time.cut();
        } else {
            self.tasks.remove(self.focused_task_index);
        }
    }
    pub fn increment_task_selection(&mut self) {
        if self.focused_task_index < self.tasks.len() - 1 {
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
    pub fn change_focus_type(&mut self) {
        self.focus_type = match self.focus_type {
            TaskFocus::Title => TaskFocus::Content,
            TaskFocus::Content => TaskFocus::Time,
            TaskFocus::Time => TaskFocus::Title,
        };
    }
    pub fn save(&self) -> io::Result<()> {
        let mut builder = NotifierBuilder::default();
        for task in self.tasks.iter() {
            let time = if let Ok(time) = NaiveTime::from_str(&task.time.yank_text()) {
                time
            } else {
                NaiveTime::default()
            };
            builder = builder.notify(time, Reminder {
                title: task.title.value().into(),
                content: task.content.yank_text(),
            });
        }
        let notifier = builder.finish();
        if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
            let data_path = dirs.data_dir();
            notifier.save(data_path.join("reminders.json"))?;
            Ok(())
        } else {
            panic!("No home directory found");
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum TaskFocus {
    #[default]
    Title,
    Content,
    Time,
}

pub struct Task<'a> {
    title: Input,
    title_placeholder: String,
    content: TextArea<'a>,
    time: TextArea<'a>,
}

impl Task<'_> {
    fn new() -> Self {
        let mut content = TextArea::default();
        content.set_placeholder_text("CONTENT");
        let mut time = TextArea::default();
        time.set_placeholder_text("0:00");
        Task {
            title: "".into(),
            title_placeholder: "".into(),
            content,
            time,
        }
    }
}
