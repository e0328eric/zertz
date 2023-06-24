use std::ops::{Index, IndexMut};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, SetStyle, Stylize},
};
use zertz_core::{
    board::*,
    coordinate::{Coordinate as CoreCoordinate, CoordinateIter},
};

use super::{shape::Shape, terminal::Terminal};
use crate::{coordinate::Coordinate, error};

#[derive(Debug, Clone, Copy, Default)]
pub struct VisualRing {
    pub kind: Ring,
    pub focused: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    board: [VisualRing; 81],
    origin: Coordinate,
    style: ContentStyle,
}

impl GameBoard {
    pub fn new(orig_board: &Board, x: u16, y: u16) -> Self {
        let board = {
            let mut tmp = [VisualRing::default(); 81];
            for (idx, ring) in orig_board.data.into_iter().enumerate() {
                tmp[idx].kind = ring;
            }

            tmp
        };
        Self {
            board,
            origin: Coordinate::new(x, y),
            style: ContentStyle::new(),
        }
    }

    pub fn update(&mut self, orig_board: &Board) {
        for (idx, ring) in orig_board.data.into_iter().enumerate() {
            self.board[idx].kind = ring;
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
            let drawing = {
                let VisualRing {
                    kind,
                    focused,
                    selected,
                } = self.board[usize::from(coord)];
                match (kind, focused, selected) {
                    (Ring::Empty, _, _) => ".".reset(),
                    (Ring::Vacant, false, true) => "O".yellow().bold(),
                    (Ring::Vacant, true, false) => "X".green().bold(),
                    (Ring::Vacant, true, true) => "X".yellow().bold(),
                    (Ring::Vacant, _, _) => "O".bold(),
                    (Ring::Occupied(Marble::White), _, _) => "@".bold().black().on_white(),
                    (Ring::Occupied(Marble::Gray), _, _) => "@".bold().white().on_dark_grey(),
                    (Ring::Occupied(Marble::Black), _, _) => "@".bold().white().on_black(),
                }
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

impl Index<CoreCoordinate> for GameBoard {
    type Output = VisualRing;

    fn index(&self, index: CoreCoordinate) -> &Self::Output {
        &self.board[usize::from(index)]
    }
}

impl IndexMut<CoreCoordinate> for GameBoard {
    fn index_mut(&mut self, index: CoreCoordinate) -> &mut Self::Output {
        &mut self.board[usize::from(index)]
    }
}
