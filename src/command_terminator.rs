use std::sync::{Arc, atomic::AtomicBool};
#[derive(Debug)]
pub enum TerminateError {
    InterrupFailed,
}

pub trait Terminator {
    fn is_interrupted(&self) -> bool;
    fn listen_for_interrupt(&self) -> Result<(), TerminateError>;
}

pub struct CommandTerminator {
    is_interrupted: Arc<AtomicBool>,
}

impl Terminator for CommandTerminator {
    fn is_interrupted(&self) -> bool {
        self.is_interrupted
            .load(std::sync::atomic::Ordering::Relaxed)
    }
    fn listen_for_interrupt(&self) -> Result<(), TerminateError> {
        let internal_is_interrupted = Arc::clone(&self.is_interrupted);
        if ctrlc::set_handler(move || {
            internal_is_interrupted.store(true, std::sync::atomic::Ordering::Relaxed);
        })
        .is_err()
        {
            Err(TerminateError::InterrupFailed)
        } else {
            Ok(())
        }
    }
}

impl CommandTerminator {
    pub fn new() -> Self {
        CommandTerminator {
            is_interrupted: Arc::new(AtomicBool::new(false)),
        }
    }
}
