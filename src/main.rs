mod command_executor;
mod command_parser;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use std::thread;
use std::{
    process::Command,
    sync::{Arc, atomic::AtomicBool},
};

use command_parser::{CommandConfig, CommandParser};

fn main() {
    let is_interrupted: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let handler_interrupted = Arc::clone(&is_interrupted);
    if ctrlc::try_set_handler(move || {
        handler_interrupted.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .is_err()
    {
        eprintln!("Failed to execute");
        return;
    };

    let parser = CommandParser::new();
    let Some(command) = parser.get_command() else {
        eprintln!("No command provided");
        return;
    };
    let args = parser.get_arguments();
    let sleep_duration = parser.get_sleep_duration();

    while !is_interrupted.load(std::sync::atomic::Ordering::Relaxed) {
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
