mod error;
mod renderer;

use std::thread;
use std::time;

use crossterm::event::MouseEvent;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
};

use zertz_core::app::App;
use zertz_core::board::BoardKind;

use renderer::Renderer;

use crate::renderer::titlebox::TitleBox;

fn main() -> error::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut zertz = App::new(BoardKind::Rings61);

    // Entering the raw mode
    renderer.enable_raw_mode()?;

    loop {
        match main_loop(&mut renderer, &mut zertz) {
            Ok(false) => {}
            Ok(true) => break,
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    // Leave out from the raw mode
    renderer.disable_raw_mode()?;

    Ok(())
}

fn main_loop(renderer: &mut Renderer, zertz: &mut App) -> error::Result<bool> {
    static mut GET_KEY_EVENT: Option<KeyEvent> = None;
    static mut GET_MOUSE_EVENT: Option<MouseEvent> = None;

    let ten_millis = time::Duration::from_millis(10);
    let delta = time::Duration::from_millis(10);

    if renderer.poll(delta)? {
        match renderer.read() {
            Ok(event) => match event {
                Event::Key(
                    key_event @ KeyEvent {
                        code, modifiers, ..
                    },
                ) => match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => return Ok(true),
                    _ => unsafe { GET_KEY_EVENT = Some(key_event) },
                },
                Event::Mouse(mouse_event) => unsafe { GET_MOUSE_EVENT = Some(mouse_event) },
                _ => {}
            },
            Err(_) => return Ok(true),
        }
    }

    renderer.clear()?;
    renderer.draw_shape(TitleBox::new(0, 0, 98, 8, "[ Debug Block ]").bold().blue())?;
    renderer.draw_shape(TitleBox::new(1, 1, 96, 3, "[ Keyboard ]").bold().red())?;
    renderer.draw_shape(TitleBox::new(1, 4, 96, 3, "[ Mouse ]").bold().green())?;
    renderer.draw_object(
        format!("{:<94}", format!("{:?}", unsafe { GET_KEY_EVENT })).reset(),
        2,
        2,
    )?;
    renderer.draw_object(
        format!("{:<94}", format!("{:?}", unsafe { GET_MOUSE_EVENT })).reset(),
        2,
        5,
    )?;
    renderer.draw_board(zertz)?;
    renderer.flush()?;

    thread::sleep(ten_millis);

    Ok(false)
}
