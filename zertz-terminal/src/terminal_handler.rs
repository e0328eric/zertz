pub mod game_board;
pub mod rect;
pub mod shape;
mod terminal;
pub mod titlebox;

use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, ResetColor, Stylize},
};
use zertz_core::app::App;

use crate::coordinate::Coordinate;
use crate::error;
use crate::terminal_handler::{game_board::GameBoard, shape::Shape};

use self::titlebox::TitleBox;

const BOX_WIDTH: u16 = 55;
const BOX_HEIGHT: u16 = 21;
pub const X_OFFSET: u16 = 19;
pub const Y_OFFSET: u16 = 18;

#[allow(unused)]
pub struct TerminalHandler {
    terminal: terminal::Terminal,
    pub center: Coordinate,
    origin: Coordinate,
}

impl TerminalHandler {
    pub fn new() -> error::Result<Self> {
        let terminal = terminal::Terminal::new()?;
        let (width, height) = (terminal.width, terminal.height);
        let standard_x = width / 2 - BOX_WIDTH - 10;
        let standard_y = height / 2 - BOX_HEIGHT + 4;

        Ok(Self {
            terminal,
            center: Coordinate::new(width / 2, height / 2),
            origin: Coordinate::new(standard_x, standard_y),
        })
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

    pub fn draw_board(&mut self, game_board: &GameBoard) -> error::Result<()> {
        self.draw_shape(&TitleBox::new(
            self.origin.x,
            self.origin.y,
            BOX_WIDTH,
            BOX_HEIGHT,
            "[ Board ]",
        ))?;
        self.draw_shape(game_board)?;

        Ok(())
    }

    pub fn get_board_origin(&self) -> Coordinate {
        Coordinate::new(self.origin.x + X_OFFSET, self.origin.y + Y_OFFSET)
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
}

impl Deref for TerminalHandler {
    type Target = terminal::Terminal;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for TerminalHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
