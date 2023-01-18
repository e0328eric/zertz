use std::fmt::Display;
use std::io::{self, Stdout, Write};
use std::time::Duration;

#[allow(unused_imports)]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, MouseEvent},
    execute, queue,
    style::{Print, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

#[allow(unused_imports)]
use crate::error::{self, ZertzTerminalError};

pub struct Terminal {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn new() -> error::Result<Self> {
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

    pub fn draw<D>(&mut self, drawee: D, row: u16, col: u16) -> error::Result<()>
    where
        D: Display + Stylize,
    {
        Ok(queue!(self.stdout, MoveTo(row, col), Print(drawee),)?)
    }

    pub fn clear(&mut self) -> error::Result<()> {
        Ok(execute!(self.stdout, Clear(ClearType::Purge))?)
    }

    pub fn flush(&mut self) -> error::Result<()> {
        Ok(self.stdout.flush()?)
    }
}
