use super::terminal::Terminal;
use crate::error;

pub trait Shape {
    fn draw(&self, terminal: &mut Terminal) -> error::Result<()>;
}
