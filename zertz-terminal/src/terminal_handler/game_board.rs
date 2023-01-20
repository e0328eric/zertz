use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, SetStyle, Stylize},
};
use zertz_core::{board::*, coordinate::Coordinate};

use super::{shape::Shape, terminal::Terminal};
use crate::error;

const X_PADDING: u16 = 4;
const Y_PADDING: u16 = 2;

pub struct GameBoard {
    board: Board,
    origin: (u16, u16),
    style: ContentStyle,
}

impl GameBoard {
    pub fn new(board: Board, x: u16, y: u16) -> Self {
        Self {
            board,
            origin: (x, y),
            style: ContentStyle::new(),
        }
    }
}

impl Stylize for GameBoard {
    type Styled = Self;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl Shape for GameBoard {
    fn draw(&self, terminal: &mut Terminal) -> error::Result<()> {
        for x in 0..9 {
            for y in 0..9 {
                let drawing = match self.board[Coordinate::new(x as usize, y as usize)] {
                    Ring::Empty => ".".reset(),
                    Ring::Vacant => "O".bold(),
                    Ring::Occupied(Marble::White) => "@".bold().white(),
                    Ring::Occupied(Marble::Gray) => "@".bold().grey(),
                    Ring::Occupied(Marble::Black) => "@".bold().black(),
                };

                queue!(
                    terminal.stdout,
                    SetStyle(self.style),
                    MoveTo(
                        self.origin.0 + X_PADDING * x - X_PADDING / 2 * y,
                        self.origin.1 - Y_PADDING * y
                    ),
                    Print(drawing),
                )?;
            }
        }

        Ok(())
    }
}

impl AsRef<ContentStyle> for GameBoard {
    fn as_ref(&self) -> &ContentStyle {
        &self.style
    }
}

impl AsMut<ContentStyle> for GameBoard {
    fn as_mut(&mut self) -> &mut ContentStyle {
        &mut self.style
    }
}
