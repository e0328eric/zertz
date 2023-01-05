mod board;
mod coordinate;
mod error;
mod game;
mod renderer;
mod union_find;
mod utility;

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 450;

use board::Marble;
use coordinate::Coordinate;

fn main() -> error::Result<()> {
    let mut zertz = game::Game::new();

    zertz.put_marble(Coordinate::new(0, 0), Coordinate::new(6, 6), Marble::White)?;
    zertz.put_marble(Coordinate::new(3, 0), Coordinate::new(6, 5), Marble::Black)?;
    zertz.put_marble(Coordinate::new(2, 2), Coordinate::new(2, 0), Marble::Gray)?;
    zertz.put_marble(Coordinate::new(1, 0), Coordinate::new(4, 1), Marble::Gray)?;
    zertz.put_marble(Coordinate::new(5, 6), Coordinate::new(5, 2), Marble::Gray)?;
    zertz.put_marble(Coordinate::new(5, 5), Coordinate::new(4, 2), Marble::White)?;
    zertz.put_marble(Coordinate::new(3, 6), Coordinate::new(3, 1), Marble::White)?;
    zertz.put_marble(Coordinate::new(6, 4), Coordinate::new(5, 3), Marble::White)?;
    zertz.put_marble(Coordinate::new(6, 3), Coordinate::new(4, 3), Marble::White)?;
    zertz.put_marble(Coordinate::new(0, 1), Coordinate::new(5, 4), Marble::Gray)?;

    println!("{:?}", zertz);
    println!("{:?}", zertz.components.components());

    Ok(())
}
