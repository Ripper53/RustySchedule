#![windows_subsystem = "windows"]
use std::{io::{self, Write}, sync::mpsc::{channel, Receiver, Sender}, thread::JoinHandle, time::Duration};

use args::{ScheduleCli, ScheduleCommand};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use directories::ProjectDirs;
use notify_rust::Notification;
use rusty_schedule_core::{Notifier, NotifierBuilder};
#[cfg(feature = "tray")]
use tray_icon::{TrayIconBuilder, TrayIcon, TrayIconEvent, ClickType, Icon, menu::{Menu, MenuItem, MenuEvent, Submenu}};
#[cfg(feature = "tray")]
use winit::event_loop::EventLoopBuilder;

mod args;
#[cfg(feature = "tui")]
mod tui;

fn main() -> std::io::Result<()> {
    let command = ScheduleCli::parse();
    if let Some(command) = command.command {
        match command {
            ScheduleCommand::Run => run(),
            #[cfg(feature = "tui")]
            ScheduleCommand::UserInterface => {
                tui::tui_setup()?;
            },
        }
    } else {
        run()
    }
}

fn run() -> io::Result<()> {
    if let Some(dirs) = ProjectDirs::from("", "", "Rusty Notifier") {
        let data_path = dirs.data_dir();
        let (listener_sender, listener_receiver) = channel();
        let listener_handler = match Notifier::load(data_path.join("reminders.json")) {
            Ok(notifier) => listen(notifier, listener_receiver),
            Err(e) => panic!("Error loading reminders: {e}"),
        };
        #[cfg(feature = "tray")]
        create_tray_icon();
        #[cfg(not(feature = "tray"))]
        controls(
            listener_handler,
            listener_sender,
        )?;
        Ok(())
    } else {
        panic!("No home directory found");
    }
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

#[cfg(not(feature = "tray"))]
fn controls(
    listener_handler: JoinHandle<()>,
    listener_sender: Sender<ReminderEvent>,
) -> io::Result<()> {
    println!("Reminders listening... Press ESC to stop.");
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Esc => {
                        listener_sender.send(ReminderEvent::Exit).unwrap();
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
        if listener_handler.is_finished() {
            break;
        }
    }
    println!(" Closed!");
    Ok(())
}

#[cfg(feature = "tray")]
fn create_tray_icon() {
    let tray_menu = Menu::new();
    let quit_menu_item = Box::new(MenuItem::new("Quit", true, None));

    tray_menu.append(quit_menu_item.as_ref());
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu));
    let tray_icon = if let Some(icon) = load_icon() {
        tray_icon.with_icon(icon)
    } else {
        //panic!("NO ICON");
        tray_icon
    };
    let tray_icon = tray_icon
        .with_tooltip("Reminders")
        .build()
        .unwrap();
    let event_loop = EventLoopBuilder::new().build().unwrap();
    event_loop.run(move |event, event_loop| {
        if event_loop.exiting() {
            return;
        }
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id() == quit_menu_item.id() {
                event_loop.exit();
            }
        }
    });
}

#[cfg(feature = "tray")]
fn load_icon() -> Option<Icon> {
    let (icon_rgba, icon_width, icon_height) = rusty_schedule_app_macro::preload_icon!();
    match Icon::from_rgba(icon_rgba, icon_width, icon_height) {
        Ok(icon) => Some(icon),
        Err(e) => {
            println!("{e}");
            None
        },
    }
}
