use std::cell::Cell;
use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

// macro include
use crate::append_occupied_coordinate;

use crate::{
    board::*,
    coordinate::{Coordinate, CoordinateIter, Direction},
    error::{self, ZertzCoreError},
    union_find::UnionFind,
};

const MAIN_EMPTY_COORD: Coordinate = Coordinate::new(8, 0);

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub(crate) struct MarbleCount {
    white_count: usize,
    gray_count: usize,
    black_count: usize,
}

impl MarbleCount {
    fn inc(&mut self, marble: Marble) {
        match marble {
            Marble::White => self.white_count += 1,
            Marble::Gray => self.gray_count += 1,
            Marble::Black => self.black_count += 1,
        }
    }

    fn dec(&mut self, marble: Marble) -> bool {
        match marble {
            Marble::White => {
                if self.white_count == 0 {
                    return false;
                }
                self.white_count -= 1;
            }
            Marble::Gray => {
                if self.gray_count == 0 {
                    return false;
                }
                self.gray_count -= 1;
            }
            Marble::Black => {
                if self.black_count == 0 {
                    return false;
                }
                self.black_count -= 1;
            }
        }

        true
    }

    #[inline]
    fn is_win(&self) -> bool {
        self.white_count >= 4
            || self.gray_count >= 5
            || self.black_count >= 6
            || (self.white_count >= 3 && self.gray_count >= 3 && self.black_count >= 3)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Player {
    Alice,
    Bob,
    Tie,
}

impl Player {
    fn change_player(&mut self) {
        match self {
            Self::Alice => *self = Self::Bob,
            Self::Bob => *self = Self::Alice,
            _ => {}
        }
    }
}

impl From<Player> for usize {
    fn from(player: Player) -> usize {
        match player {
            Player::Alice => 0,
            Player::Bob => 1,
            Player::Tie => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameState {
    CheckIsCatchable,
    PutMarble,
    CatchMarble,
    GameEnd(Player),
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub(crate) board: Board,
    pub(crate) board_replace_history: Vec<Board>,
    #[serde(skip)]
    pub(crate) components: UnionFind<Coordinate>,
    pub(crate) current_player: Player,
    pub(crate) game_state: GameState,
    pub(crate) players_score: [MarbleCount; 2],
    pub(crate) repeat_count: Cell<usize>,
    pub(crate) total_marble: MarbleCount,
}

// ╭──────────────────────────────────────────────────────────╮
// │                      Basic Game Api                      │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    pub(crate) fn new(kind: BoardKind) -> Self {
        let mut output = Self {
            board: Board::new(kind),
            board_replace_history: Vec::with_capacity(20),
            components: UnionFind::from(CoordinateIter::new().collect::<Vec<_>>()),
            current_player: Player::Alice,
            game_state: GameState::PutMarble,
            players_score: [MarbleCount::default(); 2],
            repeat_count: Cell::new(0),
            total_marble: MarbleCount {
                white_count: 6,
                gray_count: 8,
                black_count: 10,
            },
        };
        output.calculate_components();

        output
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                   Catching Marble Api                    │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    pub(crate) fn catch_marble(
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

        self.board_replace_history.push(self.board);

        let list_catchable = self.list_catchable_once(marble_land_coord);
        Ok(if list_catchable.is_empty() {
            self.current_player.change_player();
            self.game_state = if let Some(winner) = self.who_is_win(&self.board) {
                GameState::GameEnd(winner)
            } else {
                GameState::PutMarble
            };

            None
        } else {
            self.game_state = if let Some(winner) = self.who_is_win(&self.board) {
                GameState::GameEnd(winner)
            } else {
                GameState::CatchMarble
            };
            Some(list_catchable)
        })
    }

    pub(crate) fn list_all_catchable(&self) -> Vec<CatchableMove> {
        let mut output = Vec::with_capacity(81);

        for coord in CoordinateIter::new() {
            if let Ring::Occupied(_) = self.board[coord] {
                output.push(self.list_catchable_once(coord));
            }
        }

        output.into_iter().flatten().collect()
    }

    fn list_catchable_once(&self, coord: Coordinate) -> Vec<CatchableMove> {
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
    pub(crate) fn put_marble(
        &mut self,
        put_coord: Coordinate,
        remove_coord: Coordinate,
        marble: Marble,
    ) -> error::Result<()> {
        if let Ring::Vacant = self.board[put_coord] {
            if !self.total_marble.dec(marble)
                && !self.players_score[usize::from(self.current_player)].dec(marble)
            {
                return Err(ZertzCoreError::InvalidPuttingMarble);
            }
            self.board[put_coord] = Ring::Occupied(marble);
        } else {
            return Err(ZertzCoreError::InvalidPuttingMarble);
        }

        self.remove_ring(remove_coord)?;
        self.remove_isolated_island();
        self.current_player.change_player();
        self.game_state = GameState::CheckIsCatchable;

        Ok(())
    }

    fn remove_ring(&mut self, coord: Coordinate) -> error::Result<()> {
        let list_removable = self.collect_removable_rings();

        if !list_removable.is_empty() {
            if !self.valid_to_remove_ring(coord) {
                return Err(ZertzCoreError::InvalidRingToRemove);
            }

            self.board_replace_history.clear();
            self.repeat_count.set(0);

            let ring = &mut self.board[coord];
            match ring {
                Ring::Vacant => *ring = Ring::Empty,
                _ => return Err(ZertzCoreError::InvalidRingToRemove),
            }
            self.calculate_components();
        }

        Ok(())
    }

    fn collect_removable_rings(&self) -> Vec<Coordinate> {
        let mut output = Vec::with_capacity(81);

        for coord in CoordinateIter::new() {
            if self.valid_to_remove_ring(coord) {
                output.push(coord);
            }
        }

        output
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

    pub(crate) fn calculate_components(&mut self) {
        self.components.clear();

        let mut right_coord;
        let mut up_coord;
        let mut up_right_coord;
        for coord in CoordinateIter::new() {
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

            for coord in CoordinateIter::new() {
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
                    Ring::Occupied(marble) => {
                        self.players_score[usize::from(self.current_player)].inc(marble);
                    }
                    _ => unreachable!(),
                }
                self.board[coord] = Ring::Empty;
                self.components.union(&coord, &MAIN_EMPTY_COORD);
            }
        }
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                     Check who is win                     │
// ╰──────────────────────────────────────────────────────────╯

impl Game {
    fn who_is_win(&self, board: &Board) -> Option<Player> {
        if self.players_score[0].is_win() {
            return Some(Player::Alice);
        } else if self.players_score[1].is_win() {
            return Some(Player::Bob);
        } else if self.board_replace_history.contains(board) {
            let count = self.repeat_count.get();
            self.repeat_count.set(count + 1);

            if self.repeat_count.get() >= 3 {
                return Some(Player::Tie);
            }
        }

        None
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
        writeln!(f, "\n{:-^63}\n", "\x1b[1m\x1b[97m <Marble Leftoff> \x1b[0m")?;
        writeln!(f, "{:#?}", self.total_marble)?;
        writeln!(f, "\x1b[1m[{:#?} is playing]\x1b[0m", self.current_player)?;

        Ok(())
    }
}
