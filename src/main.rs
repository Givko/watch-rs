use clap::Parser;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::thread;
use std::{io::stdout, time};
use std::{
    process::Command,
    sync::{Arc, atomic::AtomicBool},
};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Interval in seconds between command executions
    #[arg(short = 'n', long = "interval", default_value = "2")]
    interval: u64,

    /// Command to execute
    #[arg(required = true)]
    command: Vec<String>,
}

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

    let cli = Cli::parse();
    let command = &cli.command[0];
    let args = &cli.command[1..];

    let sleep_duration = time::Duration::from_secs(cli.interval);
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
