use std::cmp::Ordering;
use std::fmt::{self, Display};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    pub struct Direction: u8 {
        const LEFT  = 0x1;
        const RIGHT = 0x2;
        const UP    = 0x4;
        const DOWN  = 0x8;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    #[inline]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn raw_adjacent(self, direction: Direction) -> Self {
        let mut output = self;

        if direction & Direction::LEFT == Direction::LEFT {
            output.x -= 1;
        }
        if direction & Direction::RIGHT == Direction::RIGHT {
            output.x += 1;
        }
        if direction & Direction::UP == Direction::UP {
            output.y += 1;
        }
        if direction & Direction::DOWN == Direction::DOWN {
            output.y -= 1;
        }

        output
    }

    pub fn adjacent(self, direction: Direction) -> Option<Self> {
        let mut output = self;

        if direction & Direction::LEFT == Direction::LEFT {
            if output.x == 0 {
                return None;
            }
            output.x -= 1;
        }
        if direction & Direction::RIGHT == Direction::RIGHT {
            output.x += 1;
        }
        if direction & Direction::UP == Direction::UP {
            output.y += 1;
        }
        if direction & Direction::DOWN == Direction::DOWN {
            if output.y == 0 {
                return None;
            }
            output.y -= 1;
        }

        Some(output)
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        })
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct CoordinateIter {
    current: Coordinate,
    end: Coordinate,
    row_limit: usize,
}

impl CoordinateIter {
    pub fn new() -> Self {
        Self {
            current: Coordinate::new(0, 0),
            end: Coordinate::new(8, 8),
            row_limit: 8,
        }
    }
}

impl Iterator for CoordinateIter {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let output = self.current;

        if self.current.x >= self.row_limit {
            self.current = Coordinate::new(0, self.current.y + 1);
        } else {
            self.current = Coordinate::new(self.current.x + 1, self.current.y);
        }

        Some(output)
    }
}
