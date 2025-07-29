use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[arg(required = true)]
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let command = &cli.command[0];
    let args = &cli.command[1..];
    let output = Command::new(command).args(args).output().expect("Error");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
