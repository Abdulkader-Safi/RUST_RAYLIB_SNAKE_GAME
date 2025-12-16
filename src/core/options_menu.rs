use super::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::Button;
use raylib::prelude::*;

pub struct OptionsMenu {
    back_button: Button,
}

impl OptionsMenu {
    pub fn new() -> Self {
        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = (SCREEN_WIDTH - button_width) / 2.0;

        OptionsMenu {
            back_button: Button::new(
                center_x,
                SCREEN_HEIGHT - 100.0,
                button_width,
                button_height,
                "BACK",
            ),
        }
    }

    pub fn handle_input(&self, mouse_pos: Vector2, mouse_clicked: bool) -> bool {
        self.back_button.is_clicked(mouse_pos, mouse_clicked)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let title = "OPTIONS";
        let title_size = 60;
        let title_width = d.measure_text(title, title_size);
        let title_x = (SCREEN_WIDTH - title_width as f32) / 2.0;

        d.draw_text(title, title_x as i32, 150, title_size, Color::WHITE);

        self.back_button.draw(d, mouse_pos);
    }
}
