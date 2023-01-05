#![allow(unused)]

use crate::board::*;
use crate::coordinate::{Coordinate, CoordinateIter, Direction};
use crate::error::{self, ZertzError};
use crate::union_find::UnionFind;

const FIRST_PLAYER: usize = 0;
const SECOND_PLAYER: usize = 1;
const MAIN_EMPTY_COORD: Coordinate = Coordinate::new(6, 0);

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
    current_player: usize,
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
            current_player: FIRST_PLAYER,
            player_action: PlayerAction::PutMarble,
        };
        output.calculate_components();

        output
    }

    pub fn catch_marble(
        &mut self,
        catcher_cood: Coordinate,
        to_move: Coordinate,
    ) -> error::Result<()> {
        todo!()
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
        self.remove_isolated_island();

        Ok(())
    }

    fn remove_ring(&mut self, coord: Coordinate) -> error::Result<()> {
        if !self.valid_to_remove_ring(coord) {
            return Err(ZertzError::InvalidRingToRemove);
        }

        let ring = &mut self.board[coord];
        match ring {
            Ring::Vacant => *ring = Ring::Empty,
            _ => return Err(ZertzError::InvalidRingToRemove),
        }

        self.calculate_components();

        Ok(())
    }

    fn valid_to_remove_ring(&self, coord: Coordinate) -> bool {
        let up_right = coord.adjacent(Direction::UP | Direction::RIGHT);
        let up = coord.adjacent(Direction::UP);
        let left = coord.adjacent(Direction::LEFT);
        let left_down = coord.adjacent(Direction::LEFT | Direction::DOWN);
        let down = coord.adjacent(Direction::DOWN);
        let right = coord.adjacent(Direction::RIGHT);

        (matches!(self.board.get_option(up_right), None | Some(Ring::Empty))
            && matches!(self.board.get_option(up), None | Some(Ring::Empty)))
            || (matches!(self.board.get_option(left), None | Some(Ring::Empty))
                && matches!(self.board.get_option(up), None | Some(Ring::Empty)))
            || (matches!(self.board.get_option(left), None | Some(Ring::Empty))
                && matches!(self.board.get_option(left_down), None | Some(Ring::Empty)))
            || (matches!(self.board.get_option(down), None | Some(Ring::Empty))
                && matches!(self.board.get_option(left_down), None | Some(Ring::Empty)))
            || (matches!(self.board.get_option(down), None | Some(Ring::Empty))
                && matches!(self.board.get_option(right), None | Some(Ring::Empty)))
            || (matches!(self.board.get_option(up_right), None | Some(Ring::Empty))
                && matches!(self.board.get_option(right), None | Some(Ring::Empty)))
    }

    fn calculate_components(&mut self) {
        self.components.clear();

        let mut right_coord;
        let mut up_coord;
        let mut up_right_coord;
        for coord in CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6) {
            if self.board[coord] == Ring::Empty {
                self.components.union(&coord, &MAIN_EMPTY_COORD);
            }

            right_coord = coord.raw_adjacent(Direction::RIGHT);
            up_coord = coord.raw_adjacent(Direction::UP);
            up_right_coord = coord.raw_adjacent(Direction::RIGHT | Direction::UP);

            match self.board.get(right_coord) {
                Some(Ring::Empty) => self.components.union(&right_coord, &MAIN_EMPTY_COORD),
                Some(ring) if *ring == self.board[coord] => {
                    self.components.union(&coord, &right_coord)
                }
                _ => {}
            }

            match self.board.get(up_coord) {
                Some(Ring::Empty) => self.components.union(&up_coord, &MAIN_EMPTY_COORD),
                Some(ring) if *ring == self.board[coord] => {
                    self.components.union(&coord, &up_coord)
                }
                _ => {}
            }

            match self.board.get(up_right_coord) {
                Some(Ring::Empty) => self.components.union(&up_right_coord, &MAIN_EMPTY_COORD),
                Some(ring) if *ring == self.board[coord] => {
                    self.components.union(&coord, &up_right_coord)
                }
                _ => {}
            }
        }
    }

    fn remove_isolated_island(&mut self) {
        let main_components = self.components.components();

        let mut component_members;
        'components: for main_coord in main_components {
            if let Ring::Empty = self.board[main_coord] {
                continue;
            }

            component_members = Vec::with_capacity(81);

            for coord in CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6) {
                if self.components.find(&main_coord) != self.components.find(&coord) {
                    continue;
                }

                if let Ring::Vacant = self.board[coord] {
                    continue 'components;
                }

                component_members.push(coord);
            }

            for coord in component_members {
                match self.board[coord] {
                    Ring::Occupied(Marble::White) => {
                        self.players[self.current_player].score.white_count += 1
                    }
                    Ring::Occupied(Marble::Gray) => {
                        self.players[self.current_player].score.gray_count += 1
                    }
                    Ring::Occupied(Marble::Black) => {
                        self.players[self.current_player].score.black_count += 1
                    }
                    _ => unreachable!(),
                }
                self.board[coord] = Ring::Empty;
                self.components.union(&coord, &MAIN_EMPTY_COORD);
            }
        }
    }
}
