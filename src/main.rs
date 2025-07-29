use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[arg(short, long)]
    command: String,
}

fn main() {
    let cli = Cli::parse();

    let Ok(output) = Command::new(cli.command).status() else {
        println!("Error");
        return;
    };
}
