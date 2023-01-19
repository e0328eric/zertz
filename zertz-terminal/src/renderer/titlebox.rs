use std::string::ToString;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, SetStyle, Stylize},
};

use crate::{error, renderer::rect::Rect};

use super::{shape::Shape, terminal::Terminal};

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

        self.rect.draw(terminal)?;
        match self.title_location {
            TitleLocation::TopLeft => queue!(
                terminal.stdout,
                MoveTo(self.rect.x + 2, self.rect.y),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
            TitleLocation::TopCenter => queue!(
                terminal.stdout,
                MoveTo(self.rect.x + (self.rect.width - title_len) / 2, self.rect.y),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
            TitleLocation::TopRight => queue!(
                terminal.stdout,
                MoveTo(self.rect.x + self.rect.width - 2, self.rect.y),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
            TitleLocation::BottomLeft => queue!(
                terminal.stdout,
                MoveTo(self.rect.x + 2, self.rect.y + self.rect.height),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
            TitleLocation::BottomCenter => queue!(
                terminal.stdout,
                MoveTo(
                    self.rect.x + (self.rect.width - title_len) / 2,
                    self.rect.y + self.rect.height
                ),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
            TitleLocation::BottomRight => queue!(
                terminal.stdout,
                MoveTo(
                    self.rect.x + self.rect.width - 2,
                    self.rect.y + self.rect.height
                ),
                SetStyle(self.rect.style),
                Print(&self.title)
            )?,
        }

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
