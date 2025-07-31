mod command_executor;
mod command_parser;
mod command_terminator;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use std::process::Command;
use std::thread;

use command_parser::{CommandConfig, CommandParser};

use crate::command_terminator::{CommandTerminator, Terminator};

fn main() {
    let terminator = CommandTerminator::new();
    if terminator.listen_for_interrupt().is_err() {
        eprint!("Failed to start interrupt listener");
        return;
    }

    let parser = CommandParser::new();
    let Some(command) = parser.get_command() else {
        eprintln!("No command provided");
        return;
    };

    let args = parser.get_arguments();
    let sleep_duration = parser.get_sleep_duration();
    while !terminator.is_interrupted() {
        let Ok(_) = execute!(stdout(), Clear(ClearType::All)) else {
            println!("Failed to clear terminal");
            continue;
        };

        match Command::new(command).args(args).output() {
            Ok(output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                };
            }
            Err(e) => {
                eprintln!("Failed to execute command: {e}");
            }
        }

        thread::sleep(sleep_duration);
    }
}
