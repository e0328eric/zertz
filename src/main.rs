#![allow(unused)]

mod board;
mod renderer;
mod union_find;
mod utility;

use raylib::prelude::*;

use crate::renderer::{RaylibRenderer, ZertzRenderer};

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, rl_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("zertz")
        .build();

    let zertz_board = board::Board::new();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = RaylibRenderer::new(&mut rl, &rl_thread);

        d.clear_background(Color::BLACK);
        d.render_board(&zertz_board);
    }
}
