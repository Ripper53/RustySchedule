use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Rusty Schedule", author = "Albar", version = "0.1", about = "Schedule notifications", long_about = None)]
pub struct ScheduleCli {
    #[command(subcommand)]
    pub command: Option<ScheduleCommand>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ScheduleCommand {
    Run,
    #[cfg(feature = "tui")]
    UserInterface,
}
