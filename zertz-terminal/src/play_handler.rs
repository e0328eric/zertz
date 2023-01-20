use std::thread;
use std::time;

use crossterm::{
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
        MouseEventKind,
    },
    style::Stylize,
};
use zertz_core::{
    app::{App, GameInputData, GameOutputData},
    game::GameState,
};

use crate::coordinate::Coordinate;
use crate::error::{self, ZertzTerminalError};
use crate::terminal_handler::titlebox::TitleBox;
use crate::terminal_handler::TerminalHandler;

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
    origin: Coordinate,
    state: PlayHandlerState,
}

impl PlayHandler {
    pub fn new(app: App, terminal_handler: TerminalHandler) -> Self {
        let origin = terminal_handler.get_board_origin();
        Self {
            app,
            terminal_handler,
            origin,
            state: PlayHandlerState::StartGame,
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

        // Begin Drawing
        self.terminal_handler.clear()?;

        let mut key_event = None;
        let mut mouse_event = None;

        let mut input_data: Option<GameInputData> = None;
        let mut output_data: Option<GameOutputData> = None;
        match self.state {
            PlayHandlerState::QuitGame => return Ok(true),
            PlayHandlerState::StartGame => self.state = PlayHandlerState::GetPutCoord,
            PlayHandlerState::GetPutCoord
            | PlayHandlerState::GetRemoveCoord
            | PlayHandlerState::GetMarble
            | PlayHandlerState::GetCatchData => self.io_action(
                &mut input_data,
                &output_data,
                &mut key_event,
                &mut mouse_event,
            )?,
            PlayHandlerState::RunGame => match self.app.get_game_state() {
                GameState::PutMarble | GameState::CatchMarble => {
                    self.app.play(input_data)?;
                    output_data = self.app.get_output();
                }
                GameState::CheckIsCatchable | GameState::FoundSequentialMove => {
                    self.app.play(None)?;
                    output_data = self.app.get_output();
                    match self.app.get_game_state() {
                        GameState::PutMarble => self.state = PlayHandlerState::GetPutCoord,
                        GameState::CatchMarble => self.state = PlayHandlerState::GetCatchData,
                        _ => unreachable!(),
                    }
                }
                GameState::GameEnd(winner) => todo!(),
            },
        }

        match self.render_game(key_event, mouse_event) {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err);
                return Ok(true);
            }
        }

        // End Drawing
        self.terminal_handler.flush()?;
        thread::sleep(ten_millis);

        Ok(false)
    }

    pub fn render_game(
        &mut self,
        key_event: Option<KeyEvent>,
        mouse_event: Option<MouseEvent>,
    ) -> error::Result<()> {
        let Coordinate {
            x: center_x,
            y: center_y,
        } = self.terminal_handler.center;

        self.terminal_handler.draw_object(
            "Zertz Board Game".bold(),
            center_x - 8,
            center_y - 24,
        )?;
        self.terminal_handler.draw_board(&self.app)?;

        #[cfg(debug_assertions)]
        {
            self.terminal_handler
                .draw_shape(TitleBox::new(0, 0, 98, 8, "< Debug >").bold().blue())?;
            self.terminal_handler
                .draw_shape(TitleBox::new(1, 1, 96, 3, "< Keyboard >").bold().red())?;
            self.terminal_handler
                .draw_shape(TitleBox::new(1, 4, 96, 3, "< Mouse >").bold().green())?;

            self.terminal_handler.draw_object(
                format!("{:<94}", format!("{:?}", key_event)),
                2,
                2,
            )?;
            self.terminal_handler.draw_object(
                format!("{:<94}", format!("{:?}", mouse_event)),
                2,
                5,
            )?;
        }

        Ok(())
    }

    fn io_action(
        &mut self,
        input_data: &mut Option<GameInputData>,
        _output_data: &Option<GameOutputData>,
        key_event_option: &mut Option<KeyEvent>,
        mouse_event_option: &mut Option<MouseEvent>,
    ) -> error::Result<()> {
        let delta = time::Duration::from_millis(10);

        match self.state {
            PlayHandlerState::GetPutCoord => {}
            PlayHandlerState::GetRemoveCoord => {}
            PlayHandlerState::GetMarble => {}
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
        }

        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> error::Result<()> {
        Ok(())
    }
}

fn quit_game(code: KeyCode, modifiers: KeyModifiers) -> bool {
    matches!(
        (code, modifiers),
        (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('c' | 'd'), KeyModifiers::CONTROL)
    )
}
