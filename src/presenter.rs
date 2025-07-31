use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::Write;
use std::{io::Error, process::Output};
pub trait Presenter {
    fn clear_screen(&mut self) -> Result<(), Error>;
    fn render_output(&mut self, output: &Output) -> Result<(), Error>;
}

pub struct TerminalPresenter<W: Write, E: Write> {
    stdout: W,
    stderr: E,
}

impl<W: Write, E: Write> Presenter for TerminalPresenter<W, E> {
    fn clear_screen(&mut self) -> Result<(), Error> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    fn render_output(&mut self, output: &Output) -> Result<(), Error> {
        self.stdout.write_all(&output.stdout)?;
        self.stdout.flush()?;
        if !output.stderr.is_empty() {
            self.stderr.write_all(&output.stderr)?;
            self.stderr.flush()?;
        };

        Ok(())
    }
}

impl<W: Write, E: Write> TerminalPresenter<W, E> {
    pub fn new(stdout: W, stderr: E) -> Self {
        TerminalPresenter { stderr, stdout }
    }
}
