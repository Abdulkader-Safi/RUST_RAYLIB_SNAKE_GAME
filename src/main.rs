mod core;
mod entities;
mod ui;
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

    while !rl.window_should_close() && !game.should_close() {
        let delta_time = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();
        let mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        game.handle_input(&rl, mouse_pos, mouse_clicked);
        game.update(delta_time);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        game.draw(&mut d, mouse_pos);

        if game.show_fps() {
            d.draw_fps(10, 10);
        }
    }

    #[cfg(target_family = "wasm")]
    {
        if game.should_close() {
            web_sys::window()
                .and_then(|win| win.close().ok())
                .unwrap_or(());
        }
    }
}
