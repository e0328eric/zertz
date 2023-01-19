pub mod rect;
pub mod shape;
mod terminal;
pub mod titlebox;

use std::ops::{Deref, DerefMut};

use crossterm::style::Stylize;
use zertz_core::{app::App, board::*, coordinate::Coordinate};

use crate::error;
use crate::renderer::shape::Shape;

const X_PADDING: u16 = 4;
const Y_PADDING: u16 = 2;

#[allow(unused)]
pub struct Renderer {
    terminal: terminal::Terminal,
    center: (u16, u16),
    origin: (u16, u16),
}

impl Renderer {
    pub fn new() -> error::Result<Self> {
        let terminal = terminal::Terminal::new()?;
        let (width, height) = (terminal.width, terminal.height);

        Ok(Self {
            terminal,
            center: (width / 2, height / 2),
            origin: (width / 2 - X_PADDING * 2, height / 2 + Y_PADDING * 2),
        })
    }

    pub fn enable_raw_mode(&mut self) -> error::Result<()> {
        self.terminal.enable_raw_mode()
    }

    pub fn disable_raw_mode(&mut self) -> error::Result<()> {
        self.terminal.disable_raw_mode()
    }

    pub fn draw_shape(&mut self, shape: impl Shape) -> error::Result<()> {
        shape.draw(&mut self.terminal)
    }

    pub fn draw_board(&mut self, server: &App) -> error::Result<()> {
        let board = server.get_current_board();
        let (orig_x, orig_y) = self.origin;

        for x in 0..9 {
            for y in 0..9 {
                let drawing = match board[Coordinate::new(x as usize, y as usize)] {
                    Ring::Empty => ".".reset(),
                    Ring::Vacant => "O".bold(),
                    Ring::Occupied(Marble::White) => "@".bold().white(),
                    Ring::Occupied(Marble::Gray) => "@".bold().grey(),
                    Ring::Occupied(Marble::Black) => "@".bold().black(),
                };
                self.terminal.draw_object(
                    drawing,
                    orig_x + X_PADDING * x - X_PADDING / 2 * y,
                    orig_y - Y_PADDING * y,
                )?;
            }
        }

        Ok(())
    }
}

impl Deref for Renderer {
    type Target = terminal::Terminal;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
