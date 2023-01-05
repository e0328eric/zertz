use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};

use crate::coordinate::{Coordinate, CoordinateIter};

#[derive(Clone, Copy, Default)]
pub enum Ring {
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

pub struct Board {
    pub(crate) data: Vec<Ring>,
}

impl Board {
    pub fn new() -> Self {
        let data = vec![Ring::default(); 49];
        let mut output = Self { data };

        for coord in CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6) {
            if coord.x > coord.y + 3 || coord.y > coord.x + 3 {
                output[coord] = Ring::Empty;
            }
        }

        output
    }

    pub fn get(&self, coord: Coordinate) -> Option<&Ring> {
        if coord.x >= 7 || coord.y >= 7 {
            return None;
        }
        self.data.get(coord.x + 7 * coord.y)
    }

    pub fn get_option(&self, coord: Option<Coordinate>) -> Option<&Ring> {
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
        &self.data[coord.x + 7 * coord.y]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        &mut self.data[coord.x + 7 * coord.y]
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "{:?}", &self.data[7 * 6..7 * 6 + 7])?;
        writeln!(f, "  {:?}", &self.data[7 * 5..7 * 5 + 7])?;
        writeln!(f, "    {:?}", &self.data[7 * 4..7 * 4 + 7])?;
        writeln!(f, "      {:?}", &self.data[7 * 3..7 * 3 + 7])?;
        writeln!(f, "        {:?}", &self.data[7 * 2..7 * 2 + 7])?;
        writeln!(f, "          {:?}", &self.data[7 * 1..7 * 1 + 7])?;
        writeln!(f, "            {:?}", &self.data[7 * 0..7 * 0 + 7])?;

        Ok(())
    }
}
