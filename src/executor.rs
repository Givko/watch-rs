use std::process::{Command, Output};

pub enum ExecutionError {
    ExecutionError,
}

pub trait Executor {
    fn execute_once(&self, command: &str, args: &[String]) -> Result<Output, ExecutionError>;
}

pub struct CommandExecutor;

impl Executor for CommandExecutor {
    fn execute_once(&self, command: &str, args: &[String]) -> Result<Output, ExecutionError> {
        if let Ok(output) = Command::new(command).args(args).output() {
            Ok(output)
        } else {
            Err(ExecutionError::ExecutionError)
        }
    }
}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {}
    }
}
