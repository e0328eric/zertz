use std::thread;
use std::time;

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind},
    style::Stylize,
};
use zertz_core::app::App;

use crate::error::{self, ZertzTerminalError};
use crate::terminal_handler::titlebox::TitleBox;
use crate::terminal_handler::TerminalHandler;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PlayHandlerState {
    #[default]
    Running,
    QuitGame,
}

#[allow(dead_code)]
pub struct PlayHandler {
    app: App,
    terminal_handler: TerminalHandler,
    origin: (u16, u16),
}

impl PlayHandler {
    pub fn new(app: App, terminal_handler: TerminalHandler) -> Self {
        let origin = terminal_handler.get_board_origin();
        Self {
            app,
            terminal_handler,
            origin,
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
        let delta = time::Duration::from_millis(10);

        let mut key_event = None;
        let mut mouse_event = None;

        let output = if self.terminal_handler.poll(delta)? {
            self.event_handler(
                self.terminal_handler.read()?,
                &mut key_event,
                &mut mouse_event,
            )? == PlayHandlerState::QuitGame
        } else {
            false
        };

        match self.render_game(key_event, mouse_event) {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err);
                return Ok(true);
            }
        }
        thread::sleep(ten_millis);

        Ok(output)
    }

    pub fn render_game(
        &mut self,
        key_event: Option<KeyEvent>,
        mouse_event: Option<MouseEvent>,
    ) -> error::Result<()> {
        let (center_x, center_y) = self.terminal_handler.center;

        // Begin Drawing
        self.terminal_handler.clear()?;

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

        // End Drawing
        self.terminal_handler.flush()?;

        Ok(())
    }

    fn event_handler(
        &mut self,
        event: Event,
        key_event_option: &mut Option<KeyEvent>,
        mouse_event_option: &mut Option<MouseEvent>,
    ) -> error::Result<PlayHandlerState> {
        match event {
            Event::Key(key_event) => {
                *key_event_option = Some(key_event);
                self.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) => {
                *mouse_event_option = Some(mouse_event);
                self.handle_mouse_event(mouse_event)
            }
            _ => Ok(PlayHandlerState::default()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> error::Result<PlayHandlerState> {
        let KeyEvent {
            code,
            modifiers,
            kind: _kind,
            state: _state,
        } = key_event;

        if quit_game(code, modifiers) {
            return Ok(PlayHandlerState::QuitGame);
        }

        Ok(PlayHandlerState::default())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> error::Result<PlayHandlerState> {
        let MouseEvent {
            kind,
            column,
            row,
            modifiers: _modifiers,
        } = mouse_event;

        Ok(PlayHandlerState::default())
    }
}

fn quit_game(code: KeyCode, modifiers: KeyModifiers) -> bool {
    matches!(
        (code, modifiers),
        (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('c' | 'd'), KeyModifiers::CONTROL)
    )
}
