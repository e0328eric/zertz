mod kind;
pub type BoardKind = kind::BoardKind;

use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};

use crate::coordinate::Coordinate;

#[derive(Clone, Copy, Default)]
pub(crate) enum Ring {
    Empty,
    #[default]
    Vacant,
    Occupied(Marble),
}

impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Empty, _) | (_, Self::Empty) => false,
            _ => true,
        }
    }
}

impl Debug for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Vacant => write!(f, "O"),
            Self::Occupied(Marble::White) => write!(f, "\x1b[107;30m@\x1b[0m"),
            Self::Occupied(Marble::Gray) => write!(f, "\x1b[100;30m@\x1b[0m"),
            Self::Occupied(Marble::Black) => write!(f, "\x1b[97;40m@\x1b[0m"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Marble {
    White,
    Gray,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Board {
    pub kind: BoardKind,
    pub data: [Ring; 81],
}

impl Board {
    pub(crate) fn new(kind: BoardKind) -> Self {
        let data = {
            let mut data = [Ring::Empty; 81];
            let tmp = match kind {
                BoardKind::Rings37 => kind::RINGS37_BOARD,
                BoardKind::Rings40 => kind::RINGS40_BOARD,
                BoardKind::Rings43 => kind::RINGS43_BOARD,
                BoardKind::Rings44 => kind::RINGS44_BOARD,
                BoardKind::Rings48 => kind::RINGS48_BOARD,
                BoardKind::Rings61 => kind::RINGS61_BOARD,
            }
            .into_iter()
            .flatten()
            .map(|byte| match byte {
                0 => Ring::Empty,
                1 => Ring::Vacant,
                _ => unreachable!(),
            })
            .enumerate();

            for (idx, ring) in tmp {
                data[idx] = ring;
            }

            data
        };

        Self { kind, data }
    }

    pub(crate) fn get(&self, coord: Coordinate) -> Option<&Ring> {
        if coord.x >= 9 || coord.y >= 9 {
            return None;
        }
        self.data.get(coord.x + 9 * coord.y)
    }

    pub(crate) fn get_option(&self, coord: Option<Coordinate>) -> Option<&Ring> {
        if let Some(coord) = coord {
            self.get(coord)
        } else {
            None
        }
    }
}

impl Index<Coordinate> for Board {
    type Output = Ring;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.x < 9 && coord.y < 9);
        &self.data[coord.x + 9 * coord.y]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.x < 9 && coord.y < 9);
        &mut self.data[coord.x + 9 * coord.y]
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last_draw_num = match self.kind {
            BoardKind::Rings61 => 9,
            BoardKind::Rings43 | BoardKind::Rings48 => 8,
            _ => 7,
        };
        write!(f, "\n    ")?;
        for i in 0..last_draw_num {
            write!(f, "{} ", i)?;
        }
        if self.kind == BoardKind::Rings61 {
            write!(f, "\n  8 ")?;
            for i in (9 * 8)..(9 * 8 + last_draw_num) {
                write!(f, "{:?} ", &self.data[i])?;
            }
        }
        if self.kind != BoardKind::Rings37 {
            write!(f, "\n  7  ")?;
            for i in (9 * 7)..(9 * 7 + last_draw_num) {
                write!(f, "{:?} ", &self.data[i])?;
            }
        }
        write!(f, "\n  6   ")?;
        for i in (9 * 6)..(9 * 6 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  5    ")?;
        for i in (9 * 5)..(9 * 5 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  4     ")?;
        for i in (9 * 4)..(9 * 4 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  3      ")?;
        for i in (9 * 3)..(9 * 3 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  2       ")?;
        for i in (9 * 2)..(9 * 2 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  1        ")?;
        for i in (9 * 1)..(9 * 1 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n  0         ")?;
        for i in (9 * 0)..(9 * 0 + last_draw_num) {
            write!(f, "{:?} ", &self.data[i])?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}
