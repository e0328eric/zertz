use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, BoardKind, Marble, Ring},
    coordinate::Coordinate,
    error::{self, ZertzCoreError},
    game::{CatchableMove, Game, GameState, MarbleCount, Player},
};

#[derive(Default)]
pub struct GameInputData {
    put_coord: Option<Coordinate>,
    remove_coord: Option<Coordinate>,
    marble: Option<Marble>,
    catch_data: Option<CatchableMove>,
}

impl GameInputData {
    pub fn put_marble_data(
        put_coord: Coordinate,
        remove_coord: Coordinate,
        marble: Marble,
    ) -> Self {
        Self {
            put_coord: Some(put_coord),
            remove_coord: Some(remove_coord),
            marble: Some(marble),
            catch_data: None,
        }
    }

    pub fn catch_marble_data(catch_data: CatchableMove) -> Self {
        Self {
            put_coord: None,
            remove_coord: None,
            marble: None,
            catch_data: Some(catch_data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GameOutputData {
    pub movable_list: Option<Vec<CatchableMove>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct History {
    board: Board,
    current_player: Player,
    players_score: [MarbleCount; 2],
    total_marble: MarbleCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    game: Game,
    game_history: Vec<History>,
    prev_game_history: Option<History>,
    #[serde(skip)]
    output_data: GameOutputData,
}

impl Server {
    pub fn new(kind: BoardKind) -> Self {
        Self {
            game: Game::new(kind),
            game_history: Vec::with_capacity(100),
            prev_game_history: None,
            output_data: GameOutputData::default(),
        }
    }

    pub fn play(&mut self, data: GameInputData) -> error::Result<()> {
        let list_all_catchable = self.game.list_all_catchable();

        match self.game.game_state {
            GameState::CheckIsCatchable => {
                if list_all_catchable.is_empty() {
                    self.game.game_state = GameState::PutMarble;
                } else {
                    self.game.game_state = GameState::CatchMarble;
                }

                self.output_data = GameOutputData {
                    movable_list: Some(list_all_catchable),
                };
            }
            GameState::PutMarble => {
                if let GameInputData {
                    put_coord: Some(put_coord),
                    remove_coord: Some(remove_coord),
                    marble: Some(marble),
                    ..
                } = data
                {
                    self.game.put_marble(put_coord, remove_coord, marble)?;
                    self.game_history.push(self.get_game_history());
                    self.prev_game_history = None;
                    self.output_data = GameOutputData::default();
                } else {
                    return Err(ZertzCoreError::InvalidInputData);
                }
            }
            GameState::CatchMarble => {
                if let GameInputData {
                    catch_data: Some(catch_data),
                    ..
                } = data
                {
                    let movable_list = self.game.catch_marble(catch_data)?;
                    self.game_history.push(self.get_game_history());
                    self.prev_game_history = None;
                    self.output_data = GameOutputData { movable_list };
                } else {
                    return Err(ZertzCoreError::InvalidInputData);
                }
            }
            GameState::GameEnd(_) => {}
        }

        Ok(())
    }

    pub fn rewind(&mut self) {
        if let Some(History {
            board,
            current_player,
            players_score,
            total_marble,
        }) = self.prev_game_history
        {
            self.game.board = board;
            self.game.current_player = current_player;
            self.game.players_score = players_score;
            self.game.total_marble = total_marble;
        } else if let Some(
            game_history @ History {
                board,
                current_player,
                players_score,
                total_marble,
            },
        ) = self.game_history.pop()
        {
            self.game.board = board;
            self.game.current_player = current_player;
            self.game.players_score = players_score;
            self.game.total_marble = total_marble;
            self.prev_game_history = Some(game_history);
        }

        self.game.calculate_components();
        self.game.game_state = GameState::CheckIsCatchable;
    }

    pub fn force_rewind(&mut self) {
        if let Some(
            game_history @ History {
                board,
                current_player,
                players_score,
                total_marble,
            },
        ) = self.game_history.pop()
        {
            self.game.board = board;
            self.game.current_player = current_player;
            self.game.players_score = players_score;
            self.game.total_marble = total_marble;
            self.prev_game_history = Some(game_history);
        }

        self.game.calculate_components();
        self.game.game_state = GameState::CheckIsCatchable;
    }

    #[inline]
    pub fn get_game_state(&self) -> GameState {
        self.game.game_state
    }

    pub fn get_current_board(&self) -> Board {
        self.game.board
    }

    pub fn load(json_str: impl AsRef<str>) -> error::Result<Self> {
        let mut server = serde_json::from_str::<Self>(json_str.as_ref())
            .map_err(|err| ZertzCoreError::LoadFailed(err))?;
        server.game.calculate_components();
        Ok(server)
    }

    pub fn save(&self) -> error::Result<String> {
        serde_json::to_string(self).map_err(|err| ZertzCoreError::SaveFailed(err))
    }

    pub fn load_without_history(json_str: impl AsRef<str>) -> error::Result<Self> {
        let mut server = serde_json::from_str::<Self>(json_str.as_ref())
            .map_err(|err| ZertzCoreError::LoadFailed(err))?;
        server.game.calculate_components();
        Ok(server)
    }

    pub fn save_without_history(&self) -> error::Result<String> {
        serde_json::to_string(self).map_err(|err| ZertzCoreError::SaveFailed(err))
    }

    fn get_game_history(&self) -> History {
        History {
            board: self.game.board,
            current_player: self.game.current_player,
            players_score: self.game.players_score,
            total_marble: self.game.total_marble,
        }
    }
}
