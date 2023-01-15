use std::fmt::{self, Debug, Display};

// macro include
use crate::append_occupied_coordinate;

use crate::board::*;
use crate::coordinate::{Coordinate, CoordinateIter, Direction};
use crate::error::{self, ZertzCoreError};
use crate::union_find::UnionFind;

const MAIN_EMPTY_COORD: Coordinate = Coordinate::new(6, 0);

#[derive(Debug, Clone, Copy, Default)]
struct Score {
    white_count: usize,
    gray_count: usize,
    black_count: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Player {
    Alice,
    Bob,
}

impl Player {
    fn change_player(&mut self) {
        match self {
            Self::Alice => *self = Self::Bob,
            Self::Bob => *self = Self::Alice,
        }
    }
}

impl From<Player> for usize {
    fn from(player: Player) -> usize {
        match player {
            Player::Alice => 0,
            Player::Bob => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameAction {
    PutMarble,
    CatchMarble(Vec<CatchableMove>),
}

#[derive(Debug, Clone, Copy)]
pub struct CatchableMove {
    start_coord: Coordinate,
    catched_coord: Coordinate,
    marble_land_coord: Coordinate,
}

impl Display for CatchableMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{} -> {}}}", self.start_coord, self.marble_land_coord)
    }
}

pub struct Game {
    board: Board,
    components: UnionFind<Coordinate>,
    players_score: [Score; 2],
    current_player: Player,
}

// ╭──────────────────────────────────────────────────────────╮
// │                      Basic Game Api                      │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    pub fn new() -> Self {
        let mut output = Self {
            board: Board::new(),
            components: UnionFind::from(
                CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6)
                    .collect::<Vec<_>>(),
            ),
            players_score: [Score::default(); 2],
            current_player: Player::Alice,
        };
        output.calculate_components();

        output
    }

    pub fn give_main_state(&self) -> GameAction {
        let list_all_catchable = self.list_all_catchable();

        if list_all_catchable.is_empty() {
            GameAction::PutMarble
        } else {
            GameAction::CatchMarble(list_all_catchable)
        }
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                   Catching Marble Api                    │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    pub fn catch_marble(
        &mut self,
        catch_data: CatchableMove,
    ) -> error::Result<Option<Vec<CatchableMove>>> {
        let CatchableMove {
            start_coord,
            catched_coord,
            marble_land_coord,
        } = catch_data;

        match self.board[catched_coord] {
            Ring::Occupied(Marble::White) => {
                self.players_score[usize::from(self.current_player)].white_count += 1
            }
            Ring::Occupied(Marble::Gray) => {
                self.players_score[usize::from(self.current_player)].gray_count += 1
            }
            Ring::Occupied(Marble::Black) => {
                self.players_score[usize::from(self.current_player)].black_count += 1
            }
            _ => return Err(ZertzCoreError::FailedToCatchMarble),
        }

        self.board[marble_land_coord] = self.board[start_coord];
        self.board[start_coord] = Ring::Vacant;
        self.board[catched_coord] = Ring::Vacant;

        let list_catchable = self.list_catchable_once(marble_land_coord);

        Ok(if list_catchable.is_empty() {
            self.current_player.change_player();
            None
        } else {
            Some(list_catchable)
        })
    }

    fn list_all_catchable(&self) -> Vec<CatchableMove> {
        let mut output = Vec::with_capacity(49);

        for coord in CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(6, 6), 6) {
            if let Ring::Occupied(_) = self.board[coord] {
                output.push(self.list_catchable_once(coord));
            }
        }

        output.into_iter().flatten().collect()
    }

    pub fn list_catchable_once(&self, coord: Coordinate) -> Vec<CatchableMove> {
        let mut output = Vec::with_capacity(6);

        append_occupied_coordinate!(self: output, coord, Direction::UP | Direction::RIGHT);
        append_occupied_coordinate!(self: output, coord, Direction::UP);
        append_occupied_coordinate!(self: output, coord, Direction::LEFT);
        append_occupied_coordinate!(self: output, coord, Direction::LEFT | Direction::DOWN);
        append_occupied_coordinate!(self: output, coord, Direction::DOWN);
        append_occupied_coordinate!(self: output, coord, Direction::RIGHT);

        output
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                    Putting Marble Api                    │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    pub fn put_marble(
        &mut self,
        put_coord: Coordinate,
        remove_coord: Coordinate,
        marble: Marble,
    ) -> error::Result<()> {
        if let Ring::Vacant = self.board[put_coord] {
            self.board[put_coord] = Ring::Occupied(marble);
        } else {
            return Err(ZertzCoreError::InvalidPuttingMarble);
        }

        self.remove_ring(remove_coord)?;
        self.remove_isolated_island();
        self.current_player.change_player();

        Ok(())
    }

    fn remove_ring(&mut self, coord: Coordinate) -> error::Result<()> {
        if !self.valid_to_remove_ring(coord) {
            return Err(ZertzCoreError::InvalidRingToRemove);
        }

        let ring = &mut self.board[coord];
        match ring {
            Ring::Vacant => *ring = Ring::Empty,
            _ => return Err(ZertzCoreError::InvalidRingToRemove),
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
                        self.players_score[usize::from(self.current_player)].white_count += 1
                    }
                    Ring::Occupied(Marble::Gray) => {
                        self.players_score[usize::from(self.current_player)].gray_count += 1
                    }
                    Ring::Occupied(Marble::Black) => {
                        self.players_score[usize::from(self.current_player)].black_count += 1
                    }
                    _ => unreachable!(),
                }
                self.board[coord] = Ring::Empty;
                self.components.union(&coord, &MAIN_EMPTY_COORD);
            }
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:=<50}", "")?;
        write!(f, "\n{:-^63}", "\x1b[1m\x1b[97m <Board> \x1b[0m")?;
        write!(f, "{:?}", self.board)?;
        writeln!(
            f,
            "\n{:-^63}\n",
            "\x1b[1m\x1b[97m <Player Informations> \x1b[0m"
        )?;
        writeln!(f, "{:#?}", self.players_score)?;

        Ok(())
    }
}
