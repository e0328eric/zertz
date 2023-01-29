mod coordinate;
mod error;
mod play_handler;
mod renderer;

use std::mem::MaybeUninit;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time;

use crossterm::event::Event;
use zertz_core::app::App;
use zertz_core::board::BoardKind;

use play_handler::PlayHandler;
use renderer::{RenderData, Renderer};

fn main() -> error::Result<()> {
    // TODO: make an user input with changing the board.
    let mut center = MaybeUninit::uninit();
    let mut origin = MaybeUninit::uninit();
    let mut renderer = Renderer::new(&mut center, &mut origin)?;

    // SAFETY: While this code is executed, since Renderer::new is successfully runs,
    // center and origin are safetly initialized. So we can unwrap MaybeUninits.
    let (play_handler, init_render_data) = unsafe {
        PlayHandler::new(
            App::new(BoardKind::Rings61),
            center.assume_init(),
            origin.assume_init(),
        )
    };

    let (render_data_sender, render_data_receiver) = channel::<Option<RenderData>>();
    let (event_sender, event_receiver) = channel::<Event>();

    let delta = time::Duration::from_millis(10);

    // Renderer thread
    let render_thread = thread::spawn(move || -> error::Result<()> {
        // Entering the raw mode
        renderer.enable_raw_mode()?;

        // Power on the renderer
        let mut prev_render_data = Some(init_render_data.clone());
        let event = renderer.render(&init_render_data)?;
        event_sender
            .send(event)
            .expect("send failed. (event_sender)");

        loop {
            let render_data = match render_data_receiver.recv_timeout(delta) {
                Ok(render_data) => {
                    prev_render_data = render_data.clone();
                    &prev_render_data
                }
                Err(RecvTimeoutError::Timeout) => &prev_render_data,
                Err(err) => {
                    return Err(error::ZertzTerminalError::UnexpectedPanic(err.to_string()))
                }
            };
            if let Some(render_data) = render_data {
                let event = renderer.render(render_data)?;
                if event_sender.send(event).is_err() {
                    break;
                }
            } else {
                break;
            }
        }

        // Leave out from the raw mode
        renderer.disable_raw_mode()?;

        Ok(())
    });

    // Play Handler thread
    let play_handler_thread = thread::spawn(move || -> error::Result<()> {
        let mut exit_game;
        let mut play_handler = play_handler;

        loop {
            let event = event_receiver
                .recv()
                .expect("receive failed (event_receiver)");
            let render_data = play_handler.run_game(event)?;
            exit_game = render_data.is_none() || play_handler.is_game_end();
            render_data_sender
                .send(render_data)
                .expect("send failed. (render_data_sender)");

            if exit_game {
                break;
            }
        }

        Ok(())
    });

    play_handler_thread.join().unwrap()?;
    render_thread.join().unwrap()?;

    Ok(())
}
