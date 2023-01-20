use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, SetStyle, Stylize},
};
use zertz_core::{board::*, coordinate::CoordinateIter};

use super::{shape::Shape, terminal::Terminal};
use crate::{coordinate::Coordinate, error};

pub struct GameBoard {
    board: Board,
    origin: Coordinate,
    style: ContentStyle,
}

impl GameBoard {
    pub fn new(board: Board, x: u16, y: u16) -> Self {
        Self {
            board,
            origin: Coordinate::new(x, y),
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
        for coord in CoordinateIter::new() {
            let drawing = match self.board[coord] {
                Ring::Empty => ".".reset(),
                Ring::Vacant => "O".bold(),
                Ring::Occupied(Marble::White) => "@".bold().white(),
                Ring::Occupied(Marble::Gray) => "@".bold().grey(),
                Ring::Occupied(Marble::Black) => "@".bold().black(),
            };

            let render_coord = Coordinate::from_core_coord(coord, self.origin);
            queue!(
                terminal.stdout,
                SetStyle(self.style),
                MoveTo(render_coord.x, render_coord.y),
                Print(drawing),
            )?;
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
