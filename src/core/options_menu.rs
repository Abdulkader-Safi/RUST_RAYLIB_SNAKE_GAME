use super::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::{Button, Checkbox};
use raylib::prelude::*;

pub struct OptionsMenu {
    back_button: Button,
    fps_checkbox: Checkbox,
}

impl OptionsMenu {
    pub fn new(show_fps: bool) -> Self {
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
            fps_checkbox: Checkbox::new(200.0, 250.0, 30.0, "Show FPS", show_fps),
        }
    }

    pub fn handle_input(&mut self, mouse_pos: Vector2, mouse_clicked: bool) -> OptionsMenuAction {
        if self.fps_checkbox.handle_click(mouse_pos, mouse_clicked) {
            return OptionsMenuAction::ToggleFPS;
        }

        if self.back_button.is_clicked(mouse_pos, mouse_clicked) {
            return OptionsMenuAction::Back;
        }

        OptionsMenuAction::None
    }

    pub fn update_fps_checkbox(&mut self, show_fps: bool) {
        self.fps_checkbox.set_checked(show_fps);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let title = "OPTIONS";
        let title_size = 60;
        let title_width = d.measure_text(title, title_size);
        let title_x = (SCREEN_WIDTH - title_width as f32) / 2.0;

        d.draw_text(title, title_x as i32, 80, title_size, Color::WHITE);

        self.fps_checkbox.draw(d, mouse_pos);
        self.back_button.draw(d, mouse_pos);
    }
}

pub enum OptionsMenuAction {
    None,
    Back,
    ToggleFPS,
}
