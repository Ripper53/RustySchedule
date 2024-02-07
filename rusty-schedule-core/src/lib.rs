use std::{collections::HashMap, fs::File, io::{self, Read, Write}, path::Path, time::SystemTime};

use chrono::{Local, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};


/// Notifies use of reminders.
#[derive(Serialize, Deserialize)]
pub struct Notifier {
    reminders: HashMap<NaiveTime, Reminder>,
    /// Latest reminder that was notified.
    #[serde(skip)]
    latest_notified: Option<NaiveTime>,
}

#[derive(thiserror::Error, Debug)]
pub enum NotifierLoadError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error)
}

impl Notifier {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, NotifierLoadError> {
        let mut file = File::open(path)?;
        let mut read = Vec::default();
        file.read_to_end(&mut read)?;
        let notifier = serde_json::de::from_slice(&read)?;
        Ok(notifier)
    }
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::create(path)?;
        let saved_data = serde_json::ser::to_vec(self).unwrap();
        file.write_all(&saved_data)?;
        Ok(())
    }
    /// Iterator of reminders yet to be notified.
    pub fn check_reminders(&mut self) -> impl Iterator<Item = &Reminder> {
        let time = Local::now().naive_local().time();
        let latest_notified = self.latest_notified.clone();
        let reminders = self.reminders.iter()
            .filter_map(move |(reminder_time, reminder)| {
                if let Some(latest_notified) = latest_notified {
                    if *reminder_time > latest_notified && *reminder_time <= time {
                        Some(reminder)
                    } else {
                        None
                    }
                } else {
                    if *reminder_time <= time {
                        Some(reminder)
                    } else {
                        None
                    }
                }
            });
        self.latest_notified = Some(time);
        reminders
    }
}

#[derive(Default)]
pub struct NotifierBuilder {
    reminders: HashMap<NaiveTime, Reminder>,
}

impl NotifierBuilder {
    pub fn notify(mut self, time: NaiveTime, reminder: Reminder) -> Self {
        self.reminders.insert(time, reminder);
        self
    }
    pub fn finish(self) -> Notifier {
        Notifier {
            reminders: self.reminders,
            latest_notified: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Reminder {
    pub title: String,
    pub content: String,
}