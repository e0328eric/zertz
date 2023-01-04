#![allow(unused)]

mod board;
mod coordinate;
mod error;
mod game;
mod renderer;
mod union_find;
mod utility;

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 450;

use coordinate::{Coordinate, Direction};

fn main() -> error::Result<()> {
    let mut zertz = game::Game::new();

    println!("{:?}", zertz);
    println!("{:?}", zertz.components.components());

    zertz.remove_ring(Coordinate::new(1, 0))?;
    zertz.remove_ring(Coordinate::new(2, 0))?;
    zertz.remove_ring(Coordinate::new(2, 1))?;
    zertz.remove_ring(Coordinate::new(1, 1))?;
    zertz.remove_ring(Coordinate::new(0, 1))?;
    zertz.remove_ring(Coordinate::new(6, 6))?;

    println!("{:?}", zertz);
    println!("{:?}", zertz.components.components());

    assert_eq!(
        zertz.components.find(&Coordinate::new(3, 0)),
        zertz.components.find(&Coordinate::new(0, 2)),
    );

    Ok(())
}
