use std::{io::{self, Write}, sync::mpsc::{channel, Receiver, Sender}, thread::JoinHandle, time::Duration};

use args::ScheduleCommand;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use directories::ProjectDirs;
use notify_rust::Notification;
use rusty_schedule_core::{Notifier, NotifierBuilder};

mod args;
#[cfg(feature = "tui")]
mod tui;

fn main() -> std::io::Result<()> {
    let command = ScheduleCommand::parse();
    match command {
        ScheduleCommand::Run => {
            if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
                let data_path = dirs.data_dir();
                let (sender, receiver) = channel();
                let handler = match Notifier::load(data_path.join("reminders.json")) {
                    Ok(notifier) => listen(notifier, receiver),
                    Err(e) => panic!("Error loading reminders: {e}"),
                };
                controls(handler, sender)?;
            } else {
                panic!("No home directory found");
            }
        },
        #[cfg(feature = "tui")]
        ScheduleCommand::UserInterface => {
            tui::tui_setup()?;
        },
    }
    Ok(())
}

enum ReminderEvent {
    Exit,
}

fn listen(mut notifier: Notifier, mut receiver: Receiver<ReminderEvent>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        'main: loop {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    ReminderEvent::Exit => break 'main,
                }
            }
            for reminder in notifier.check_reminders() {
                Notification::new()
                    .appname("REMINDER")
                    .summary(&reminder.title)
                    .body(&reminder.content)
                    .timeout(0)
                    .show().unwrap();
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    })
}

fn controls(listener: JoinHandle<()>, sender: Sender<ReminderEvent>) -> io::Result<()> {
    println!("Reminders listening... Press ESC to stop.");
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Esc => {
                        sender.send(ReminderEvent::Exit).unwrap();
                        break;
                    },
                    _ => {},
                }
            }
        }
    }
    print!("Closing listeners...");
    io::stdout().flush();
    loop {
        if listener.is_finished() {
            break;
        }
    }
    println!(" Closed!");
    Ok(())
}
