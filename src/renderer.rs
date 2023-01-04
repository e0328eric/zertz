use raylib::prelude::*;

use crate::board::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub trait ZertzRenderer {
    fn render_board(&mut self, board: &Board);
}

// Raylib Renderer for zertz game
#[repr(transparent)]
pub struct RaylibRenderer<'r> {
    draw_handle: RaylibDrawHandle<'r>,
}

impl<'r> RaylibRenderer<'r> {
    pub fn new(rl: &'r mut RaylibHandle, thread: &'r RaylibThread) -> Self {
        Self {
            draw_handle: rl.begin_drawing(thread),
        }
    }
}

fn coord_to_vec2(x: usize, y: usize) -> Vector2 {
    let (x, y) = (x as f32, y as f32);

    Vector2 {
        x: x * 60.0 - y * 30.0 + WINDOW_WIDTH as f32 / 2.7,
        y: -3.0f32.sqrt() * 30.0 * y + WINDOW_HEIGHT as f32 * 0.85,
    }
}

impl RaylibDraw for RaylibRenderer<'_> {}

impl ZertzRenderer for RaylibRenderer<'_> {
    fn render_board(&mut self, board: &Board) {
        for x in 0..7 {
            for y in 0..7 {
                let ring = board[(x, y)];

                match ring {
                    Ring::None => {}
                    Ring::Vacant => {
                        self.draw_circle_v(coord_to_vec2(x, y), 30., Color::WHITE);
                        self.draw_circle_v(coord_to_vec2(x, y), 20., Color::BLACK);
                    }
                    Ring::Occupied(Marble::White) => {
                        self.draw_circle_v(coord_to_vec2(x, y), 30., Color::WHITE);
                        self.draw_circle_v(coord_to_vec2(x, y), 20., Color::BLACK);
                        self.draw_circle_v(coord_to_vec2(x, y), 10., Color::WHITE);
                    }
                    Ring::Occupied(Marble::Grey) => {
                        self.draw_circle_v(coord_to_vec2(x, y), 30., Color::WHITE);
                        self.draw_circle_v(coord_to_vec2(x, y), 20., Color::BLACK);
                        self.draw_circle_v(coord_to_vec2(x, y), 10., Color::GRAY);
                    }
                    Ring::Occupied(Marble::Black) => {
                        self.draw_circle_v(coord_to_vec2(x, y), 30., Color::WHITE);
                        self.draw_circle_v(coord_to_vec2(x, y), 20., Color::BLACK);
                        self.draw_circle_v(coord_to_vec2(x, y), 10., Color::DARKGRAY);
                    }
                }
            }
        }
    }
}
