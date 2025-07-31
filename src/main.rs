mod executor;
mod parser;
mod presenter;
mod terminator;
mod watch;

use std::io::{stderr, stdout};

use crate::{
    executor::CommandExecutor, parser::CommandParser, presenter::TerminalPresenter,
    terminator::CommandTerminator, watch::Watch,
};

fn main() {
    let parser = CommandParser::new();
    let terminator = CommandTerminator::default();
    let presenter = TerminalPresenter::new(stdout(), stderr());
    let command_executor = CommandExecutor::new();
    let mut watch = Watch::new(parser, presenter, command_executor, terminator);
    let _ = watch.run();
}
