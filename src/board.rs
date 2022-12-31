use std::ops::{Index, IndexMut};

use crate::union_find::UnionFind;

#[derive(Debug, Clone, Copy, Default)]
pub enum Ring {
    None,
    #[default]
    Vacant,
    Occupied(Marble),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Marble {
    White,
    Grey,
    Black,
}

#[derive(Debug)]
pub struct Board {
    data: Vec<Ring>,
    // components: UnionFind<Coordinate>,
}

impl Board {
    pub fn new() -> Self {
        let data = vec![Ring::default(); 64];
        let mut output = Self { data };

        for x in 0..7 {
            for y in 0..7 {
                if x > y + 3 || y > x + 3 {
                    output[(x, y)] = Ring::None;
                }
            }
        }

        output
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Ring;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.0 + 9 * idx.1]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.data[idx.0 + 9 * idx.1]
    }
}
