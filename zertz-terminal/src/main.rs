mod controlflow;
mod error;
mod renderer;
mod terminal;

use std::thread;
use std::time;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use zertz_core::board::BoardKind;
use zertz_core::server::Server;

use renderer::Renderer;
use terminal::Terminal;

fn main() -> error::Result<()> {
    let terminal = Terminal::new()?;
    let mut renderer = Renderer::new(terminal);
    let zertz = Server::new(BoardKind::Rings61);

    let ten_millis = time::Duration::from_millis(10);
    let delta = time::Duration::from_millis(10);

    // Entering the raw mode
    renderer.enable_raw_mode()?;

    terminal_loop! {
        if unwrap!(renderer.poll(delta)) {
            match renderer.read() {
                Ok(event) => match event {
                    Event::Key(KeyEvent {
                        code, modifiers,
                        ..
                    }) => match (code, modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::NONE) => jmp!("break"),
                        _ => {},
                    }
                    _ => {},
                }
                Err(_) => jmp!("break"),
            }
        }


        unwrap!(renderer.clear())

        unwrap!(renderer.draw_board(&zertz))

        unwrap!(renderer.flush())

        thread::sleep(ten_millis)
    }

    // Leave out from the raw mode
    renderer.disable_raw_mode()?;

    Ok(())
}
