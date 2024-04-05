use std::{io::{self, Write}, sync::mpsc::{channel, Receiver, Sender}, thread::JoinHandle, time::Duration};

use args::ScheduleCommand;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use directories::ProjectDirs;
use notify_rust::Notification;
use rusty_schedule_core::{Notifier, NotifierBuilder};
#[cfg(feature = "tray")]
use tray_icon::{TrayIconBuilder, TrayIcon, TrayIconEvent, menu::{Menu, MenuEvent}};

mod args;
#[cfg(feature = "tui")]
mod tui;

fn main() -> std::io::Result<()> {
    let command = ScheduleCommand::parse();
    match command {
        ScheduleCommand::Run => {
            if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
                let data_path = dirs.data_dir();
                let (listener_sender, listener_receiver) = channel();
                #[cfg(feature = "tray")]
                let (tray_sender, tray_receiver) = channel();
                #[cfg(feature = "tray")]
                let tray_handler = create_tray_icon(tray_receiver);
                let listener_handler = match Notifier::load(data_path.join("reminders.json")) {
                    Ok(notifier) => listen(notifier, listener_receiver),
                    Err(e) => panic!("Error loading reminders: {e}"),
                };
                controls(
                    listener_handler,
                    listener_sender,
                    #[cfg(feature = "tray")]
                    tray_handler,
                    #[cfg(feature = "tray")]
                    tray_sender,
                )?;
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

fn controls(
    listener_handler: JoinHandle<()>,
    listener_sender: Sender<ReminderEvent>,
    #[cfg(feature = "tray")]
    tray_handler: JoinHandle<()>,
    #[cfg(feature = "tray")]
    tray_sender: Sender<ReminderEvent>,
) -> io::Result<()> {
    println!("Reminders listening... Press ESC to stop.");
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Esc => {
                        listener_sender.send(ReminderEvent::Exit).unwrap();
                        tray_sender.send(ReminderEvent::Exit).unwrap();
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
        if listener_handler.is_finished() && tray_handler.is_finished() {
            break;
        }
    }
    println!(" Closed!");
    Ok(())
}

#[cfg(feature = "tray")]
fn create_tray_icon(mut receiver: Receiver<ReminderEvent>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let tray_menu = Menu::new();
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Reminders")
            .build()
            .unwrap();
        loop {
            if let Ok(event) = receiver.try_recv() {
                match event {
                    ReminderEvent::Exit => break,
                }
            }
            if let Ok(event) = MenuEvent::receiver().try_recv() {
                println!("{:?}", event);
            }
            if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                println!("{:?}", event);
            }
        }
    })
}
