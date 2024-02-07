use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Rusty Schedule", author = "Albar", version = "0.1", about = "Schedule notifications", long_about = None)]
pub enum ScheduleCommand {
    Run,
    UserInterface,
}
