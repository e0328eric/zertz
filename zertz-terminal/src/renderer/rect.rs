use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, ResetColor, SetStyle, Stylize},
};

use super::{shape::Shape, terminal::Terminal};
use crate::error;

pub(super) const HORIZONTAL_LINE: char = '\u{2501}';
pub(super) const VERTICAL_LINE: char = '\u{2503}';
pub(super) const TOP_LEFT_CORNER: char = '\u{250F}';
pub(super) const BOTTOM_LEFT_CORNER: char = '\u{2517}';
pub(super) const TOP_RIGHT_CORNER: char = '\u{2513}';
pub(super) const BOTTOM_RIGHT_CORNER: char = '\u{251B}';

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub(super) style: ContentStyle,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width: width.saturating_sub(1),
            height: height.saturating_sub(1),
            style: ContentStyle::new(),
        }
    }
}

impl Stylize for Rect {
    type Styled = Self;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl Shape for Rect {
    fn draw(&self, terminal: &mut Terminal) -> error::Result<()> {
        queue!(
            terminal.stdout,
            SetStyle(self.style),
            MoveTo(self.x, self.y),
            Print(TOP_LEFT_CORNER),
            MoveTo(self.x + self.width, self.y),
            Print(TOP_RIGHT_CORNER),
            MoveTo(self.x, self.y + self.height),
            Print(BOTTOM_LEFT_CORNER),
            MoveTo(self.x + self.width, self.y + self.height),
            Print(BOTTOM_RIGHT_CORNER),
        )?;

        for i in 1..self.width {
            queue!(
                terminal.stdout,
                SetStyle(self.style),
                MoveTo(self.x + i, self.y),
                Print(HORIZONTAL_LINE),
                MoveTo(self.x + i, self.y + self.height),
                Print(HORIZONTAL_LINE),
            )?;
        }
        for i in 1..self.height {
            queue!(
                terminal.stdout,
                SetStyle(self.style),
                MoveTo(self.x, self.y + i),
                Print(VERTICAL_LINE),
                MoveTo(self.x + self.width, self.y + i),
                Print(VERTICAL_LINE),
            )?;
        }
        queue!(terminal.stdout, ResetColor)?;

        Ok(())
    }
}

impl AsRef<ContentStyle> for Rect {
    fn as_ref(&self) -> &ContentStyle {
        &self.style
    }
}

impl AsMut<ContentStyle> for Rect {
    fn as_mut(&mut self) -> &mut ContentStyle {
        &mut self.style
    }
}
