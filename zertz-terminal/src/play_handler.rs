use std::time;

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use zertz_core::coordinate::CoordinateIter;
use zertz_core::error::ZertzCoreError;
use zertz_core::{
    app::{App, GameInputData, GameOutputData},
    board::Marble,
    game::GameState,
};

use crate::coordinate::Coordinate;
use crate::error;
use crate::error::ZertzTerminalError;
use crate::renderer::{game_board::GameBoard, RenderData, RendererState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayHandlerState {
    GetPutCoord,
    GetRemoveCoord,
    GetMarble,
    GetCatchData,
    RunGame,
    QuitGame,
}

#[allow(dead_code)]
pub struct PlayHandler {
    app: App,
    game_board: GameBoard,
    state: PlayHandlerState,
    renderer_state: RendererState,
    input_data: Option<GameInputData>,
    output_data: Option<GameOutputData>,
    game_origin: Coordinate,
    explain_primary_text: String,
    explain_supplimentary_text: String,
}

impl PlayHandler {
    pub fn new(app: App, center: Coordinate, origin: Coordinate) -> (Self, RenderData) {
        let game_board = GameBoard::new(&app.get_current_board(), origin.x, origin.y);

        let play_handler = Self {
            app,
            game_board,
            state: PlayHandlerState::GetPutCoord,
            renderer_state: RendererState::default(),
            input_data: None,
            output_data: None,
            game_origin: origin,
            explain_primary_text: String::with_capacity(center.x as usize * 2),
            explain_supplimentary_text: String::with_capacity(center.x as usize * 2),
        };
        let init_render_data = RenderData {
            state: RendererState::DrawIntro,
            game_board: play_handler.game_board,
            players_score: play_handler.app.players_score,
            total_marble: play_handler.app.total_marble,
            explain_primary_text: play_handler.explain_primary_text.clone(),
            explain_supplimentary_text: play_handler.explain_supplimentary_text.clone(),
        };

        (play_handler, init_render_data)
    }

    pub fn is_game_end(&self) -> bool {
        self.state == PlayHandlerState::QuitGame
    }

    pub fn run_game(&mut self, event: Event) -> error::Result<Option<RenderData>> {
        match self.state {
            PlayHandlerState::QuitGame => return Ok(None),
            PlayHandlerState::GetPutCoord
            | PlayHandlerState::GetRemoveCoord
            | PlayHandlerState::GetMarble
            | PlayHandlerState::GetCatchData => self.main_game_event_handle(event)?,
            PlayHandlerState::RunGame => {
                for coord in CoordinateIter::new() {
                    self.game_board[coord].focused = false;
                    self.game_board[coord].selected = false;
                }

                match self.app.get_game_state() {
                    GameState::PutMarble => {
                        match self.app.play(&self.input_data) {
                            Ok(()) => {}
                            // TODO: display errors
                            Err(
                                ZertzCoreError::InvalidInputData
                                | ZertzCoreError::InvalidPuttingMarble
                                | ZertzCoreError::InvalidRingToRemove,
                            ) => {
                                self.app.rewind();
                                self.state = PlayHandlerState::GetPutCoord;
                                return Ok(Some(RenderData {
                                    state: RendererState::RedrawEntire,
                                    game_board: self.game_board,
                                    players_score: self.app.players_score,
                                    total_marble: self.app.total_marble,
                                    explain_primary_text: self.explain_primary_text.clone(),
                                    explain_supplimentary_text: self
                                        .explain_supplimentary_text
                                        .clone(),
                                }));
                            }
                            Err(err) => return Err(err.into()),
                        }
                        self.game_board.update(&self.app.get_current_board());
                        self.output_data = self.app.get_output();
                    }
                    GameState::CatchMarble => {
                        match self.app.play(&self.input_data) {
                            Ok(()) => {}
                            Err(ZertzCoreError::InvalidInputData) => {
                                self.app.rewind();
                                self.state = PlayHandlerState::GetCatchData;
                                return Ok(Some(RenderData {
                                    state: RendererState::RedrawEntire,
                                    game_board: self.game_board,
                                    players_score: self.app.players_score,
                                    total_marble: self.app.total_marble,
                                    explain_primary_text: self.explain_primary_text.clone(),
                                    explain_supplimentary_text: self
                                        .explain_supplimentary_text
                                        .clone(),
                                }));
                            }
                            Err(err) => return Err(err.into()),
                        }
                        self.game_board.update(&self.app.get_current_board());
                        self.output_data = self.app.get_output();
                    }
                    GameState::CheckIsCatchable | GameState::FoundSequentialMove => {
                        self.app.play(&None)?;
                        self.output_data = self.app.get_output();
                        match self.app.get_game_state() {
                            GameState::PutMarble => self.state = PlayHandlerState::GetPutCoord,
                            GameState::CatchMarble => self.state = PlayHandlerState::GetCatchData,
                            _ => unreachable!(),
                        }
                    }
                    GameState::GameEnd(_winner) => todo!(),
                }
            }
        }

        return Ok(Some(RenderData {
            state: self.renderer_state,
            game_board: self.game_board,
            players_score: self.app.players_score,
            total_marble: self.app.total_marble,
            explain_primary_text: self.explain_primary_text.clone(),
            explain_supplimentary_text: self.explain_supplimentary_text.clone(),
        }));
    }

    fn main_game_event_handle(&mut self, event: Event) -> error::Result<()> {
        match self.state {
            PlayHandlerState::GetPutCoord => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the ring (the 'O' characters) where the marble is put.");
                self.renderer_state = RendererState::UpdateExplanation;
            }
            PlayHandlerState::GetRemoveCoord => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the ring (the 'X' characters) to remove.");
                for coord in self.app.get_removable_rings().into_iter() {
                    self.game_board[coord].focused = true;
                }
                self.renderer_state = RendererState::UpdateExplanation;
            }
            PlayHandlerState::GetMarble => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the color of the marble to put on:");
                self.explain_supplimentary_text
                    .push_str("Input w/W for white, g/G for gray, and b/B for black.");
                self.renderer_state = RendererState::UpdateExplanation;
            }
            PlayHandlerState::GetCatchData => {
                todo!("play_handler::io_action");
            }
            _ => unreachable!(),
        }

        match event {
            Event::Key(key_event) => self.handle_key_event(key_event)?,
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event)?,
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> error::Result<()> {
        let KeyEvent {
            code,
            modifiers,
            kind: _kind,
            state: _state,
        } = key_event;

        if quit_game(code, modifiers) {
            self.state = PlayHandlerState::QuitGame;
            return Ok(());
        }

        match self.state {
            PlayHandlerState::GetPutCoord => {}
            PlayHandlerState::GetRemoveCoord => {}
            PlayHandlerState::GetMarble => {
                if modifiers == KeyModifiers::NONE {
                    match code {
                        KeyCode::Char('w') => {
                            self.input_data
                                .as_mut()
                                .map(|input_data| input_data.marble = Some(Marble::White));
                            self.state = PlayHandlerState::RunGame;
                        }
                        KeyCode::Char('g') => {
                            self.input_data
                                .as_mut()
                                .map(|input_data| input_data.marble = Some(Marble::Gray));
                            self.state = PlayHandlerState::RunGame;
                        }
                        KeyCode::Char('b') => {
                            self.input_data
                                .as_mut()
                                .map(|input_data| input_data.marble = Some(Marble::Black));
                            self.state = PlayHandlerState::RunGame;
                        }
                        _ => {}
                    }
                }
            }
            PlayHandlerState::GetCatchData => {}
            _ => unreachable!(),
        }

        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> error::Result<()> {
        let MouseEvent {
            kind,
            column,
            row,
            modifiers,
        } = mouse_event;

        match self.state {
            PlayHandlerState::GetPutCoord => {
                if (kind, modifiers)
                    == (MouseEventKind::Down(MouseButton::Left), KeyModifiers::NONE)
                {
                    let valid_coord = if let Some(coord) =
                        Coordinate::new(column, row).into_core_coord(self.game_origin)
                    {
                        coord
                    } else {
                        return Ok(());
                    };

                    self.game_board[valid_coord].selected = true;

                    self.input_data = Some(GameInputData {
                        put_coord: Some(valid_coord),
                        remove_coord: None,
                        marble: None,
                        catch_data: None,
                    });
                    self.state = PlayHandlerState::GetRemoveCoord;
                }
            }
            PlayHandlerState::GetRemoveCoord => {
                if (kind, modifiers)
                    == (MouseEventKind::Down(MouseButton::Left), KeyModifiers::NONE)
                {
                    let valid_coord = if let Some(coord) =
                        Coordinate::new(column, row).into_core_coord(self.game_origin)
                    {
                        coord
                    } else {
                        return Ok(());
                    };

                    self.game_board[valid_coord].selected = true;
                    for coord in self.app.get_removable_rings().into_iter() {
                        if coord != valid_coord {
                            self.game_board[coord].focused = false;
                        }
                    }

                    self.input_data
                        .as_mut()
                        .map(|input_data| input_data.remove_coord = Some(valid_coord));
                    self.state = PlayHandlerState::GetMarble;
                }
            }
            PlayHandlerState::GetMarble => {}
            PlayHandlerState::GetCatchData => {}
            _ => unreachable!(),
        }

        Ok(())
    }

    fn clear_explain_text(&mut self) {
        self.explain_primary_text.clear();
        self.explain_supplimentary_text.clear();
    }
}

fn quit_game(code: KeyCode, modifiers: KeyModifiers) -> bool {
    matches!(
        (code, modifiers),
        (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('c' | 'd'), KeyModifiers::CONTROL)
    )
}
