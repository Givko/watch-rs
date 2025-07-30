use clap::Parser;
use std::time::Duration;

pub trait CommandConfig {
    /// Returns the command which needs to be executed
    /// if no command was parsed None will be returned
    fn get_command(&self) -> Option<&String>;

    /// Returns the arguments of the command, if any.
    /// If no arguments were passed - &[] will be returned
    fn get_arguments(&self) -> &[String];

    /// The sleep duration will be returned
    /// Duration which we need to wait between executions by default is it 2 seconds
    fn get_sleep_duration(&self) -> Duration;
}

pub struct CommandParser {
    /// Internal parsed command from clap
    parser: Command,
}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser {
            parser: Command::parse(),
        }
    }
}

impl CommandConfig for CommandParser {
    fn get_command(&self) -> Option<&String> {
        self.parser.command.first()
    }

    fn get_arguments(&self) -> &[String] {
        self.parser.command.get(1..).unwrap_or_default()
    }

    fn get_sleep_duration(&self) -> Duration {
        Duration::from_secs(self.parser.interval)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Command {
    /// Interval in seconds between command executions
    #[arg(short = 'n', long = "interval", default_value = "2")]
    interval: u64,

    /// Command to execute
    #[arg(required = true)]
    command: Vec<String>,
}
