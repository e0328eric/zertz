pub mod game_board;
pub mod rect;
pub mod shape;
mod terminal;
pub mod titlebox;

use std::fmt::Display;
use std::mem::MaybeUninit;
use std::time;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute, queue,
    style::{Print, ResetColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use zertz_core::game::MarbleCount;

use crate::coordinate::Coordinate;
use crate::error::{self, ZertzTerminalError};

use self::{game_board::GameBoard, shape::Shape, titlebox::TitleBox};

const BOX_WIDTH: u16 = 55;
const BOX_HEIGHT: u16 = 21;
pub const X_OFFSET: u16 = 19;
pub const Y_OFFSET: u16 = 18;
pub const PRIMARY_TEXT_OFFSET: u16 = 14;
pub const SUPPLIMENTARY_TEXT_OFFSET: u16 = 15;

#[derive(Clone)]
pub struct RenderData {
    pub state: RendererState,
    pub game_board: GameBoard,
    pub players_score: [MarbleCount; 2],
    pub total_marble: MarbleCount,
    pub explain_primary_text: String,
    pub explain_supplimentary_text: String,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum RendererState {
    #[default]
    DefaultState,
    DrawIntro,
    RedrawEntire,
    UpdateExplanation,
    ErasePrevExplanation,
    DrawWarningMsgbox,
}

#[allow(unused)]
pub struct Renderer {
    terminal: terminal::Terminal,
    state: RendererState,
    center: Coordinate,
    origin: Coordinate,
    prevent_update: bool,
}

impl Renderer {
    pub fn new(
        maybe_center: &mut MaybeUninit<Coordinate>,
        maybe_origin: &mut MaybeUninit<Coordinate>,
    ) -> error::Result<Self> {
        let terminal = terminal::Terminal::new()?;
        let (width, height) = (terminal.width, terminal.height);

        let (origin_x, x_overflow) = (width >> 1).overflowing_sub(BOX_WIDTH + 10);
        let (origin_y, y_overflow) = (height >> 1).overflowing_sub(BOX_HEIGHT - 5);

        if x_overflow || y_overflow {
            Err(ZertzTerminalError::InappropriateTerminalSize)
        } else {
            let center = *maybe_center.write(Coordinate::new(width >> 1, height >> 1));
            let origin =
                *maybe_origin.write(Coordinate::new(origin_x + X_OFFSET, origin_y + Y_OFFSET));
            Ok(Self {
                terminal,
                state: RendererState::default(),
                center,
                origin,
                prevent_update: false,
            })
        }
    }

    pub fn enable_raw_mode(&mut self) -> error::Result<()> {
        enable_raw_mode()?;
        execute!(
            self.terminal.stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            Hide
        )?;
        Ok(())
    }

    pub fn disable_raw_mode(&mut self) -> error::Result<()> {
        execute!(
            self.terminal.stdout,
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        )?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw_object<D>(&mut self, drawee: &D, row: u16, col: u16) -> error::Result<()>
    where
        D: Display + Stylize,
    {
        Ok(queue!(
            self.terminal.stdout,
            MoveTo(row, col),
            Print(drawee),
            ResetColor,
        )?)
    }

    pub fn draw_shape(&mut self, shape: &impl Shape) -> error::Result<()> {
        shape.draw(&mut self.terminal)
    }

    #[allow(dead_code)]
    pub fn draw_axis(&mut self) -> error::Result<()> {
        let Coordinate { x, y } = self.center;

        self.draw_object(&"0", x, y)?;
        for i in (1..).take_while(|n| n * 5 < x) {
            self.draw_object(&format!("{}", i * 5), x + i * 5, y)?;
            self.draw_object(&format!("-{}", i * 5), x - i * 5, y)?;
        }
        for i in (1..).take_while(|n| n * 2 < y) {
            self.draw_object(&format!("{}", i * 2), x, y + i * 2)?;
            self.draw_object(&format!("-{}", i * 2), x, y - i * 2)?;
        }

        Ok(())
    }

    pub fn render(&mut self, render_data: &RenderData) -> error::Result<Event> {
        let delta = time::Duration::from_millis(10);

        let event = loop {
            if self.terminal.poll(delta)? {
                break self.terminal.read()?;
            }

            let RenderData {
                state,
                game_board,
                players_score,
                total_marble,
                explain_primary_text,
                explain_supplimentary_text,
            } = render_data;

            self.state = *state;
            self.render_branch(
                game_board,
                *players_score,
                *total_marble,
                explain_primary_text,
                explain_supplimentary_text,
            )?;
        };

        Ok(event)
    }

    fn render_branch(
        &mut self,
        game_board: &GameBoard,
        players_score: [MarbleCount; 2],
        total_marble: MarbleCount,
        explain_primary_text: &str,
        explain_supplimentary_text: &str,
    ) -> error::Result<()> {
        match self.state {
            RendererState::DefaultState => {}
            RendererState::DrawIntro => return self.render_game_intro(),
            RendererState::RedrawEntire => {
                self.state = RendererState::ErasePrevExplanation;
                self.prevent_update = false;
            }
            RendererState::UpdateExplanation => {
                self.state = RendererState::ErasePrevExplanation;
            }
            RendererState::DrawWarningMsgbox => todo!(),
            _ => unreachable!(),
        }

        self.render_game(
            game_board,
            players_score,
            total_marble,
            explain_primary_text,
            explain_supplimentary_text,
        )?;

        Ok(())
    }

    fn render_game(
        &mut self,
        game_board: &GameBoard,
        players_score: [MarbleCount; 2],
        total_marble: MarbleCount,
        explain_primary_text: &str,
        explain_supplimentary_text: &str,
    ) -> error::Result<()> {
        // Begin Drawing
        self.terminal.clear()?;

        self.draw_object(
            &"Zertz Board Game".bold(),
            self.center.x - 8,
            self.center.y - 24,
        )?;

        if self.state == RendererState::ErasePrevExplanation && !self.prevent_update {
            self.terminal
                .clear_line(self.center.y + PRIMARY_TEXT_OFFSET)?;
            self.terminal
                .clear_line(self.center.y + SUPPLIMENTARY_TEXT_OFFSET)?;
            self.state = RendererState::default();
            self.prevent_update = true;
        }

        self.draw_object(
            &explain_primary_text,
            self.center.x - explain_primary_text.len() as u16 / 2,
            self.center.y + PRIMARY_TEXT_OFFSET,
        )?;
        self.draw_object(
            &explain_supplimentary_text,
            self.center.x - explain_supplimentary_text.len() as u16 / 2,
            self.center.y + SUPPLIMENTARY_TEXT_OFFSET,
        )?;
        self.draw_shape(&TitleBox::new(
            self.origin.x - X_OFFSET,
            self.origin.y - Y_OFFSET,
            BOX_WIDTH,
            BOX_HEIGHT,
            "[ Board ]",
        ))?;
        self.draw_shape(game_board)?;

        // End Drawing
        self.terminal.flush()?;

        Ok(())
    }

    // TODO: fully implement this
    fn render_game_intro(&mut self) -> error::Result<()> {
        let msg = "Press any key to start a game";
        self.draw_object(
            &msg.bold(),
            self.center.x - msg.len() as u16 / 2,
            self.center.y + PRIMARY_TEXT_OFFSET,
        )?;
        Ok(())
    }
}
