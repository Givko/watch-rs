use std::thread;

use crate::{
    executor::Executor, parser::CommandConfig, presenter::Presenter, terminator::Terminator,
};

pub enum ExecutionError {
    InterruptListenerFailed,
    CommandNotProvided,
    ExecutionFailed,
    PresenterFailed,
}

pub struct Watch<C: CommandConfig, P: Presenter, E: Executor, T: Terminator> {
    command_config: C,
    terminator: T,
    command_executor: E,
    presenter: P,
}

impl<C: CommandConfig, P: Presenter, E: Executor, T: Terminator> Watch<C, P, E, T> {
    pub fn new(command_config: C, presenter: P, command_executor: E, terminator: T) -> Self {
        Watch {
            command_config,
            presenter,
            command_executor,
            terminator,
        }
    }

    pub fn run(&mut self) -> Result<(), ExecutionError> {
        let Ok(_) = self.terminator.listen_for_interrupt() else {
            return Err(ExecutionError::InterruptListenerFailed);
        };
        let Some(command) = self.command_config.get_command() else {
            return Err(ExecutionError::CommandNotProvided);
        };

        let args = self.command_config.get_arguments();
        let sleep_duration = self.command_config.get_sleep_duration();

        while !self.terminator.is_interrupted() {
            if self.presenter.clear_screen().is_err() {
                continue;
            }
            let Ok(output) = self.command_executor.execute_once(command, args) else {
                return Err(ExecutionError::ExecutionFailed);
            };
            let Ok(_) = self.presenter.render_output(&output) else {
                return Err(ExecutionError::PresenterFailed);
            };

            thread::sleep(sleep_duration);
        }
        Ok(())
    }
}
