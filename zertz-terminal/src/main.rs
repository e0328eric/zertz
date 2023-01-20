mod coordinate;
mod error;
mod play_handler;
mod terminal_handler;

use zertz_core::app::App;
use zertz_core::board::BoardKind;

use play_handler::PlayHandler;
#[allow(unused_imports)]
use terminal_handler::{
    titlebox::{TitleBox, TitleLocation},
    TerminalHandler,
};

fn main() -> error::Result<()> {
    let terminal_handler = TerminalHandler::new()?;
    let zertz = App::new(BoardKind::Rings61);
    let mut play_handler = PlayHandler::new(zertz, terminal_handler);

    // Entering the raw mode
    play_handler.enable_raw_mode()?;

    loop {
        match play_handler.run_game() {
            Ok(false) => {}
            Ok(true) => break,
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    // Leave out from the raw mode
    play_handler.disable_raw_mode()?;

    Ok(())
}
