use std::{io::{self, Write}, time::Duration};

use args::ScheduleCommand;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use directories::ProjectDirs;
use notify_rust::Notification;
use rusty_schedule_core::{Notifier, NotifierBuilder};
use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::JoinHandle};

mod args;
mod tui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let command = ScheduleCommand::parse();
    match command {
        ScheduleCommand::Run => {
            if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
                let data_path = dirs.data_dir();
                let (sender, receiver) = channel(128);
                let handler = match Notifier::load(data_path.join("reminders.json")) {
                    Ok(notifier) => listen(notifier, receiver),
                    Err(e) => panic!("Error loading reminders: {e}"),
                };
                controls(handler, sender).await?;
            } else {
                panic!("No home directory found");
            }
        },
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
    tokio::spawn(async move {
        'main: loop {
            while let Some(event) = receiver.recv().await {
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
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
}

async fn controls(listener: JoinHandle<()>, sender: Sender<ReminderEvent>) -> io::Result<()> {
    println!("Reminders listening... Press ESC to stop.");
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Esc => {
                        sender.send(ReminderEvent::Exit).await.unwrap();
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
