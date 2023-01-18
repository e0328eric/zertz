use std::ops::{Deref, DerefMut};

use crossterm::style::Stylize;
use zertz_core::{board::*, coordinate::Coordinate, server::Server};

use crate::error;
use crate::Terminal;

const X_PADDING: u16 = 4;
const Y_PADDING: u16 = 2;

#[allow(unused)]
pub struct Renderer {
    terminal: Terminal,
    center: (u16, u16),
    origin: (u16, u16),
}

impl Renderer {
    pub fn new(terminal: Terminal) -> Self {
        let (width, height) = (terminal.width, terminal.height);

        Self {
            terminal,
            center: (width / 2, height / 2),
            origin: (width / 2 - X_PADDING * 2, height / 2 + Y_PADDING * 2),
        }
    }

    pub fn draw_board(&mut self, server: &Server) -> error::Result<()> {
        let board = server.get_current_board();
        let (orig_x, orig_y) = self.origin;

        for x in 0..9 {
            for y in 0..9 {
                let drawing = match board[Coordinate::new(x as usize, y as usize)] {
                    Ring::Empty => " ".reset(),
                    Ring::Vacant => "O".bold(),
                    Ring::Occupied(Marble::White) => "@".bold().white(),
                    Ring::Occupied(Marble::Gray) => "@".bold().grey(),
                    Ring::Occupied(Marble::Black) => "@".bold().black(),
                };
                self.draw(
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
    type Target = Terminal;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
