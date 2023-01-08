mod board;
mod coordinate;
mod error;
mod game;
mod macros;
mod union_find;
mod utility;

use board::Marble;
use coordinate::Coordinate;

macro_rules! test_zertz {
    ($zertz: ident put at $put_coord: expr, $marble: expr, remove at $remove_coord: expr) => {
        $zertz.put_marble($put_coord, $remove_coord, $marble)?;
        println!("{:?}", $zertz);
    };
}

fn main() -> error::Result<()> {
    let mut zertz = game::Game::new();

    test_zertz!(zertz put at Coordinate::new(0, 0), Marble::White, remove at Coordinate::new(6, 6));
    test_zertz!(zertz put at Coordinate::new(3, 0), Marble::Black, remove at Coordinate::new(6, 5));
    test_zertz!(zertz put at Coordinate::new(3, 2), Marble::Gray, remove at Coordinate::new(2, 0));
    test_zertz!(zertz put at Coordinate::new(1, 0), Marble::Gray, remove at Coordinate::new(4, 1));
    test_zertz!(zertz put at Coordinate::new(5, 6), Marble::Gray, remove at Coordinate::new(5, 2));
    test_zertz!(zertz put at Coordinate::new(5, 5), Marble::White, remove at Coordinate::new(4, 2));
    test_zertz!(zertz put at Coordinate::new(3, 6), Marble::White, remove at Coordinate::new(3, 1));
    test_zertz!(zertz put at Coordinate::new(6, 4), Marble::White, remove at Coordinate::new(5, 3));
    test_zertz!(zertz put at Coordinate::new(0, 1), Marble::White, remove at Coordinate::new(4, 3));
    test_zertz!(zertz put at Coordinate::new(1, 1), Marble::Gray, remove at Coordinate::new(5, 4));
    test_zertz!(zertz put at Coordinate::new(6, 3), Marble::Black, remove at Coordinate::new(0, 3));

    println!("{:?}", zertz.list_catchable(Coordinate::new(0, 0)));

    Ok(())
}
