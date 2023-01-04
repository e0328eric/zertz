use crate::board::*;
use crate::coordinate::{Coordinate, CoordinateIter, Direction};
use crate::error::{self, ZertzError};
use crate::union_find::UnionFind;

#[derive(Debug, Clone, Copy, Default)]
struct Score {
    white_count: usize,
    gray_count: usize,
    black_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Player {
    score: Score,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum PlayerTurn {
    First = 0,
    Second = 1,
}

#[derive(Debug, Clone, Copy)]
enum PlayerAction {
    PutMarble,
    CatchMarble,
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub components: UnionFind<Coordinate>,
    players: [Player; 2],
    player_turn: PlayerTurn,
    player_action: PlayerAction,
}

impl Game {
    pub fn new() -> Self {
        let mut output = Self {
            board: Board::new(),
            components: UnionFind::from(
                CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6)
                    .collect::<Vec<_>>(),
            ),
            players: [Player::default(); 2],
            player_turn: PlayerTurn::First,
            player_action: PlayerAction::PutMarble,
        };
        output.calculate_components();

        output
    }

    pub fn playrer_action(
        &mut self,
        main_coord: Coordinate,
        remove_coord: Option<Coordinate>,
        marble: Option<Marble>,
    ) -> error::Result<()> {
        match self.player_action {
            PlayerAction::PutMarble => {
                self.put_marble(main_coord, remove_coord.unwrap(), marble.unwrap())?
            }
            PlayerAction::CatchMarble => todo!(),
        }

        Ok(())
    }

    pub fn put_marble(
        &mut self,
        put_coord: Coordinate,
        remove_coord: Coordinate,
        marble: Marble,
    ) -> error::Result<()> {
        if let Ring::Vacant = self.board[put_coord] {
            self.board[put_coord] = Ring::Occupied(marble);
        } else {
            return Err(ZertzError::InvalidPuttingMarble);
        }

        self.remove_ring(remove_coord)?;

        Ok(())
    }

    fn remove_ring(&mut self, coord: Coordinate) -> error::Result<()> {
        if !self.valid_to_remove(coord) {
            return Err(ZertzError::InvalidRingToRemove);
        }

        let ring = &mut self.board[coord];
        match ring {
            Ring::Vacant => *ring = Ring::None,
            _ => return Err(ZertzError::InvalidRingToRemove),
        }

        self.calculate_components();

        Ok(())
    }

    fn valid_to_remove(&self, coord: Coordinate) -> bool {
        let up_right = coord.adjacent(Direction::UP | Direction::RIGHT);
        let up = coord.adjacent(Direction::UP);
        let left = coord.adjacent(Direction::LEFT);
        let left_down = coord.adjacent(Direction::LEFT | Direction::DOWN);
        let down = coord.adjacent(Direction::DOWN);
        let right = coord.adjacent(Direction::RIGHT);

        (matches!(self.board.get_option(up_right), None | Some(Ring::None))
            && matches!(self.board.get_option(up), None | Some(Ring::None)))
            || (matches!(self.board.get_option(left), None | Some(Ring::None))
                && matches!(self.board.get_option(up), None | Some(Ring::None)))
            || (matches!(self.board.get_option(left), None | Some(Ring::None))
                && matches!(self.board.get_option(left_down), None | Some(Ring::None)))
            || (matches!(self.board.get_option(down), None | Some(Ring::None))
                && matches!(self.board.get_option(left_down), None | Some(Ring::None)))
            || (matches!(self.board.get_option(down), None | Some(Ring::None))
                && matches!(self.board.get_option(right), None | Some(Ring::None)))
            || (matches!(self.board.get_option(up), None | Some(Ring::None))
                && matches!(self.board.get_option(right), None | Some(Ring::None)))
    }

    fn calculate_components(&mut self) {
        self.components.clear();

        let mut right_coord;
        let mut up_coord;
        let mut up_right_coord;
        for coord in CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(5, 5), 5) {
            if self.board[coord] == Ring::None {
                self.components.union(&coord, &Coordinate::new(6, 0));
            }

            right_coord = coord.raw_adjacent(Direction::RIGHT);
            up_coord = coord.raw_adjacent(Direction::UP);
            up_right_coord = coord.raw_adjacent(Direction::RIGHT | Direction::UP);

            match self.board[right_coord] {
                Ring::None => self.components.union(&right_coord, &Coordinate::new(6, 0)),
                ring if ring == self.board[coord] => self.components.union(&coord, &right_coord),
                _ => {}
            }

            match self.board[up_coord] {
                Ring::None => self.components.union(&up_coord, &Coordinate::new(6, 0)),
                ring if ring == self.board[coord] => self.components.union(&coord, &up_coord),
                _ => {}
            }

            match self.board[up_right_coord] {
                Ring::None => self
                    .components
                    .union(&up_right_coord, &Coordinate::new(6, 0)),
                ring if ring == self.board[coord] => self.components.union(&coord, &up_right_coord),
                _ => {}
            }
        }
    }
}
