use std::string::ToString;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, ResetColor, SetStyle, Stylize},
};

use crate::error;

use super::{rect::*, shape::Shape, terminal::Terminal};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub enum TitleLocation {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct TitleBox {
    rect: Rect,
    title: String,
    title_location: TitleLocation,
}

impl TitleBox {
    pub fn new(x: u16, y: u16, width: u16, height: u16, title: impl ToString) -> Self {
        let mut title = title.to_string();
        title.truncate(width.saturating_sub(4) as usize);

        Self {
            rect: Rect::new(x, y, width, height),
            title,
            title_location: TitleLocation::default(),
        }
    }

    #[allow(dead_code)]
    pub fn alignment(mut self, title_location: TitleLocation) -> Self {
        self.title_location = title_location;
        self
    }
}

impl Stylize for TitleBox {
    type Styled = Self;

    fn stylize(self) -> Self::Styled {
        self
    }
}

impl Shape for TitleBox {
    fn draw(&self, terminal: &mut Terminal) -> error::Result<()> {
        let title_len = self.title.len() as u16;

        queue!(
            terminal.stdout,
            SetStyle(self.rect.style),
            MoveTo(self.rect.x, self.rect.y),
            Print(TOP_LEFT_CORNER),
            MoveTo(self.rect.x + self.rect.width, self.rect.y),
            Print(TOP_RIGHT_CORNER),
            MoveTo(self.rect.x, self.rect.y + self.rect.height),
            Print(BOTTOM_LEFT_CORNER),
            MoveTo(
                self.rect.x + self.rect.width,
                self.rect.y + self.rect.height
            ),
            Print(BOTTOM_RIGHT_CORNER),
        )?;

        match self.title_location {
            TitleLocation::TopLeft => {
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(self.rect.x + 1, self.rect.y),
                    Print(HORIZONTAL_LINE),
                    MoveTo(self.rect.x + 2, self.rect.y),
                    Print(&self.title)
                )?;
                for i in (2 + title_len)..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
            TitleLocation::TopCenter => {
                for i in 1..((self.rect.width - title_len) / 2) {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(
                        self.rect.x + ((self.rect.width - title_len) / 2),
                        self.rect.y
                    ),
                    Print(&self.title)
                )?;
                for i in ((self.rect.width + title_len) / 2)..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
            TitleLocation::TopRight => {
                for i in 1..(self.rect.width - 1 - title_len) {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(self.rect.x + self.rect.width - 1 - title_len, self.rect.y),
                    Print(&self.title),
                    MoveTo(self.rect.x + self.rect.width - 1, self.rect.y),
                    Print(HORIZONTAL_LINE),
                )?;
            }
            _ => {
                for i in 1..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
        }

        for i in 1..self.rect.height {
            queue!(
                terminal.stdout,
                SetStyle(self.rect.style),
                MoveTo(self.rect.x, self.rect.y + i),
                Print(VERTICAL_LINE),
                MoveTo(self.rect.x + self.rect.width, self.rect.y + i),
                Print(VERTICAL_LINE),
            )?;
        }

        match self.title_location {
            TitleLocation::BottomLeft => {
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(self.rect.x + 1, self.rect.y + self.rect.height),
                    Print(HORIZONTAL_LINE),
                    MoveTo(self.rect.x + 2, self.rect.y + self.rect.height),
                    Print(&self.title)
                )?;
                for i in (2 + title_len)..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y + self.rect.height),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
            TitleLocation::BottomCenter => {
                for i in 1..((self.rect.width - title_len) / 2) {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y + self.rect.height),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(
                        self.rect.x + ((self.rect.width - title_len) / 2),
                        self.rect.y + self.rect.height
                    ),
                    Print(&self.title)
                )?;
                for i in ((self.rect.width + title_len) / 2)..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y + self.rect.height),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
            TitleLocation::BottomRight => {
                for i in 1..(self.rect.width - 1 - title_len) {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y + self.rect.height),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
                queue!(
                    terminal.stdout,
                    SetStyle(self.rect.style),
                    MoveTo(
                        self.rect.x + self.rect.width - 1 - title_len,
                        self.rect.y + self.rect.height
                    ),
                    Print(&self.title),
                    MoveTo(
                        self.rect.x + self.rect.width - 1,
                        self.rect.y + self.rect.height
                    ),
                    Print(HORIZONTAL_LINE),
                )?;
            }
            _ => {
                for i in 1..self.rect.width {
                    queue!(
                        terminal.stdout,
                        SetStyle(self.rect.style),
                        MoveTo(self.rect.x + i, self.rect.y + self.rect.height),
                        Print(HORIZONTAL_LINE),
                    )?;
                }
            }
        }
        queue!(terminal.stdout, ResetColor)?;

        Ok(())
    }
}

impl AsRef<ContentStyle> for TitleBox {
    fn as_ref(&self) -> &ContentStyle {
        &self.rect.style
    }
}

impl AsMut<ContentStyle> for TitleBox {
    fn as_mut(&mut self) -> &mut ContentStyle {
        &mut self.rect.style
    }
}
