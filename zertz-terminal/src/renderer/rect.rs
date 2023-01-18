use std::fmt::{self, Display};

use crossterm::style::Stylize;

const HORIZONTAL_LINE: char = '\u{2501}';
const VERTICAL_LINE: char = '\u{2503}';
const LEFT_TOP_CORNER: char = '\u{250F}';
const LEFT_BOTTOM_CORNER: char = '\u{2517}';
const RIGHT_TOP_CORNER: char = '\u{2513}';
const RIGHT_BOTTOM_CORNER: char = '\u{251B}';

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", LEFT_TOP_CORNER)?;
        for _ in 1..(self.width - 1) {
            write!(f, "{}", HORIZONTAL_LINE)?;
        }
        write!(f, "{}\r\n", RIGHT_TOP_CORNER)?;
        for _ in 1..(self.height - 1) {
            write!(f, "{}", VERTICAL_LINE)?;
            for _ in 1..(self.width - 1) {
                write!(f, " ")?;
            }
            write!(f, "{}\r\n", VERTICAL_LINE)?;
        }
        write!(f, "{}", LEFT_BOTTOM_CORNER)?;
        for _ in 1..(self.width - 1) {
            write!(f, "{}", HORIZONTAL_LINE)?;
        }
        write!(f, "{}\r\n", RIGHT_BOTTOM_CORNER)?;

        Ok(())
    }
}

impl Stylize for Rect {
    type Styled = <String as Stylize>::Styled;

    fn stylize(self) -> Self::Styled {
        format!("{}", self).stylize()
    }
}
