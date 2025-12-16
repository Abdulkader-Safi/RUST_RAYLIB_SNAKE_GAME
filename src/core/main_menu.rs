use super::SCREEN_WIDTH;
use crate::ui::Button;
use raylib::prelude::*;

pub struct MainMenu {
    start_button: Button,
    options_button: Button,
    close_button: Button,
}

impl MainMenu {
    pub fn new() -> Self {
        let button_width = 250.0;
        let button_height = 60.0;
        let center_x = (SCREEN_WIDTH - button_width) / 2.0;
        let start_y = 200.0;
        let button_spacing = 80.0;

        MainMenu {
            start_button: Button::new(center_x, start_y, button_width, button_height, "START"),
            options_button: Button::new(
                center_x,
                start_y + button_spacing,
                button_width,
                button_height,
                "OPTIONS",
            ),
            close_button: Button::new(
                center_x,
                start_y + button_spacing * 2.0,
                button_width,
                button_height,
                "CLOSE",
            ),
        }
    }

    pub fn handle_input(&self, mouse_pos: Vector2, mouse_clicked: bool) -> MainMenuAction {
        if self.start_button.is_clicked(mouse_pos, mouse_clicked) {
            return MainMenuAction::Start;
        }
        if self.options_button.is_clicked(mouse_pos, mouse_clicked) {
            return MainMenuAction::Options;
        }
        if self.close_button.is_clicked(mouse_pos, mouse_clicked) {
            return MainMenuAction::Close;
        }
        MainMenuAction::None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let title = "SNAKE GAME";
        let title_size = 60;
        let title_width = d.measure_text(title, title_size);
        let title_x = (SCREEN_WIDTH - title_width as f32) / 2.0;

        d.draw_text(title, title_x as i32, 80, title_size, Color::GREEN);

        self.start_button.draw(d, mouse_pos);
        self.options_button.draw(d, mouse_pos);
        self.close_button.draw(d, mouse_pos);
    }
}

pub enum MainMenuAction {
    None,
    Start,
    Options,
    Close,
}
