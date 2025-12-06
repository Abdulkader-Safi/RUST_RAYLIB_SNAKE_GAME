mod core;
mod entities;
mod utils;

use raylib::prelude::*;

use core::{Game, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snake Game")
        .vsync()
        .build();

    let mut game = Game::new();

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        game.handle_input(&rl);
        game.update(delta_time);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        game.draw(&mut d);
        d.draw_fps(10, 10);
    }
}
