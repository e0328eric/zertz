mod rendering;

use std::thread;
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
use crate::terminal_handler::{game_board::GameBoard, TerminalHandler};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayHandlerState {
    StartGame,
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
    terminal_handler: TerminalHandler,
    game_board: GameBoard,
    center: Coordinate,
    origin: Coordinate,
    state: PlayHandlerState,
    input_data: Option<GameInputData>,
    output_data: Option<GameOutputData>,
    explain_primary_text: String,
    explain_supplimentary_text: String,
}

impl PlayHandler {
    pub fn new(app: App, terminal_handler: TerminalHandler) -> Self {
        let center = terminal_handler.center;
        let origin = terminal_handler.get_board_origin();

        let game_board = GameBoard::new(&app.get_current_board(), origin.x, origin.y);

        Self {
            app,
            terminal_handler,
            game_board,
            center,
            origin,
            state: PlayHandlerState::StartGame,
            input_data: None,
            output_data: None,
            explain_primary_text: String::with_capacity(center.x as usize * 2),
            explain_supplimentary_text: String::with_capacity(center.x as usize * 2),
        }
    }

    pub fn enable_raw_mode(&mut self) -> error::Result<()> {
        self.terminal_handler.enable_raw_mode()
    }

    pub fn disable_raw_mode(&mut self) -> error::Result<()> {
        self.terminal_handler.disable_raw_mode()
    }

    pub fn run_game(&mut self) -> error::Result<bool> {
        let ten_millis = time::Duration::from_millis(10);

        let mut key_event = None;
        let mut mouse_event = None;

        match self.state {
            PlayHandlerState::QuitGame => return Ok(true),
            PlayHandlerState::StartGame => self.state = PlayHandlerState::GetPutCoord,
            PlayHandlerState::GetPutCoord
            | PlayHandlerState::GetRemoveCoord
            | PlayHandlerState::GetMarble
            | PlayHandlerState::GetCatchData => self.io_action(&mut key_event, &mut mouse_event)?,
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
                                return Ok(false);
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
                                return Ok(false);
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

        match self.render_game(key_event, mouse_event) {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err);
                return Ok(true);
            }
        }

        thread::sleep(ten_millis);

        Ok(false)
    }

    fn io_action(
        &mut self,
        key_event_option: &mut Option<KeyEvent>,
        mouse_event_option: &mut Option<MouseEvent>,
    ) -> error::Result<()> {
        let delta = time::Duration::from_millis(10);

        match self.state {
            PlayHandlerState::GetPutCoord => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the ring (the 'O' characters) where the marble is put.");
            }
            PlayHandlerState::GetRemoveCoord => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the ring (the 'X' characters) to remove.");
                for coord in self.app.get_removable_rings().into_iter() {
                    self.game_board[coord].focused = true;
                }
            }
            PlayHandlerState::GetMarble => {
                self.clear_explain_text();
                self.explain_primary_text
                    .push_str("Select the color of the marble to put on:");
                self.explain_supplimentary_text
                    .push_str("Input w/W for white, g/G for gray, and b/B for black.");
            }
            PlayHandlerState::GetCatchData => {}
            _ => unreachable!(),
        }

        if self.terminal_handler.poll(delta)? {
            let event = self.terminal_handler.read()?;
            match event {
                Event::Key(key_event) => {
                    *key_event_option = Some(key_event);
                    self.handle_key_event(key_event)?;
                }
                Event::Mouse(mouse_event) => {
                    *mouse_event_option = Some(mouse_event);
                    self.handle_mouse_event(mouse_event)?;
                }
                _ => {}
            }
        };

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
                        Coordinate::new(column, row).into_core_coord(self.origin)
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
                        Coordinate::new(column, row).into_core_coord(self.origin)
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
