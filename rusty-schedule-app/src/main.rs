use std::time::Duration;

use args::ScheduleCommand;
use clap::Parser;
use directories::ProjectDirs;
use notify_rust::Notification;
use rusty_schedule_core::{Notifier, NotifierBuilder};
use tokio::task::JoinHandle;

mod args;
mod tui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let command = ScheduleCommand::parse();
    match command {
        ScheduleCommand::Run => {
            //let notifier = NotifierBuilder::default()
                //.notify)
            if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
                let data_path = dirs.data_dir();
                let handler = match Notifier::load(data_path.join("schedule.json")) {
                    Ok(notifier) => listen(notifier),
                    Err(_) => {
                        let notifier = NotifierBuilder::default()
                            .finish();
                        listen(notifier)
                    },
                };
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

fn listen(mut notifier: Notifier) -> JoinHandle<()> {
    tokio::spawn(async move {
        for reminder in notifier.check_reminders() {
            Notification::new()
                .appname("REMINDER")
                .summary(&reminder.title)
                .body(&reminder.content)
                .timeout(0)
                .show().unwrap();
        }
        std::thread::sleep(Duration::from_millis(200));
    })
}
