use std::io::{self, Stdout, Write};
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use crate::error;

pub struct Terminal {
    pub(super) stdout: Stdout,
    pub(super) width: u16,
    pub(super) height: u16,
}

impl Terminal {
    pub(super) fn new() -> error::Result<Self> {
        let (width, height) = size()?;

        Ok(Self {
            stdout: io::stdout(),
            width: width - 1,
            height: height - 1,
        })
    }

    pub fn enable_raw_mode(&mut self) -> error::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;
        Ok(())
    }

    pub fn disable_raw_mode(&mut self) -> error::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen, DisableMouseCapture, Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn read(&self) -> error::Result<Event> {
        Ok(read()?)
    }

    pub fn poll(&self, timeout: Duration) -> error::Result<bool> {
        Ok(poll(timeout)?)
    }

    pub fn clear(&mut self) -> error::Result<()> {
        Ok(execute!(self.stdout, Clear(ClearType::Purge))?)
    }

    pub fn clear_line(&mut self, row: u16) -> error::Result<()> {
        Ok(execute!(
            self.stdout,
            MoveTo(0, row),
            Clear(ClearType::CurrentLine)
        )?)
    }

    pub fn flush(&mut self) -> error::Result<()> {
        Ok(self.stdout.flush()?)
    }
}
