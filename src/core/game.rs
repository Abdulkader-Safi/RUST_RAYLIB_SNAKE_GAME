use raylib::prelude::*;

use super::{Direction, GameState, MOVE_INTERVAL, MainMenu, MainMenuAction, OptionsMenu};
use crate::entities::{Food, Snake};

pub struct Game {
    snake: Snake,
    food: Food,
    move_timer: f32,
    game_over: bool,
    state: GameState,
    main_menu: MainMenu,
    options_menu: OptionsMenu,
    should_close: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            snake: Snake::new(),
            food: Food::new(),
            move_timer: 0.0,
            game_over: false,
            state: GameState::MainMenu,
            main_menu: MainMenu::new(),
            options_menu: OptionsMenu::new(),
            should_close: false,
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle, mouse_pos: Vector2, mouse_clicked: bool) {
        match self.state {
            GameState::MainMenu => match self.main_menu.handle_input(mouse_pos, mouse_clicked) {
                MainMenuAction::Start => {
                    self.state = GameState::Playing;
                    self.restart();
                }
                MainMenuAction::Options => {
                    self.state = GameState::Options;
                }
                MainMenuAction::Close => {
                    #[cfg(target_family = "wasm")]
                    {
                        self.state = GameState::Closed;
                    }
                    #[cfg(not(target_family = "wasm"))]
                    {
                        self.should_close = true;
                    }
                }
                MainMenuAction::None => {}
            },
            GameState::Options => {
                if self.options_menu.handle_input(mouse_pos, mouse_clicked) {
                    self.state = GameState::MainMenu;
                }
            }
            GameState::Closed => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.state = GameState::MainMenu;
                }
            }
            GameState::Playing => {
                if self.game_over {
                    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                        self.restart();
                    }
                    if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                        self.state = GameState::MainMenu;
                        self.game_over = false;
                    }
                    return;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.state = GameState::MainMenu;
                    return;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
                    || rl.is_key_pressed(KeyboardKey::KEY_D)
                {
                    self.snake.set_direction(Direction::Right);
                }
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S)
                {
                    self.snake.set_direction(Direction::Down);
                }
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A)
                {
                    self.snake.set_direction(Direction::Left);
                }
                if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
                    self.snake.set_direction(Direction::Up);
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state != GameState::Playing {
            return;
        }

        if self.game_over {
            return;
        }

        self.move_timer += delta_time;

        if self.move_timer >= MOVE_INTERVAL {
            self.move_timer -= MOVE_INTERVAL;
            self.snake.update();

            if self.snake.check_self_collision() {
                self.game_over = true;
                return;
            }

            if self.snake.head_position() == self.food.position() {
                self.food.respawn();
                self.snake.grow();
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        match self.state {
            GameState::MainMenu => {
                self.main_menu.draw(d, mouse_pos);
            }
            GameState::Options => {
                self.options_menu.draw(d, mouse_pos);
            }
            GameState::Playing => {
                self.snake.draw(d);
                self.food.draw(d);

                if self.game_over {
                    d.draw_text("GAME OVER!", 200, 250, 40, Color::WHITE);
                    d.draw_text("Press SPACE to restart", 170, 310, 25, Color::LIGHTGRAY);
                    d.draw_text("Press ESC for main menu", 170, 350, 25, Color::LIGHTGRAY);
                }
            }
            GameState::Closed => {
                let title = "Thanks for playing!";
                let title_size = 50;
                let title_width = d.measure_text(title, title_size);
                let title_x = (super::SCREEN_WIDTH - title_width as f32) / 2.0;
                let title_y = super::SCREEN_HEIGHT / 2.0 - 80.0;

                d.draw_text(
                    title,
                    title_x as i32,
                    title_y as i32,
                    title_size,
                    Color::GREEN,
                );

                let subtitle = "You can close this tab now";
                let subtitle_size = 20;
                let subtitle_width = d.measure_text(subtitle, subtitle_size);
                let subtitle_x = (super::SCREEN_WIDTH - subtitle_width as f32) / 2.0;
                let subtitle_y = title_y + 70.0;

                d.draw_text(
                    subtitle,
                    subtitle_x as i32,
                    subtitle_y as i32,
                    subtitle_size,
                    Color::LIGHTGRAY,
                );

                let hint = "Press ESC to return to main menu";
                let hint_size = 18;
                let hint_width = d.measure_text(hint, hint_size);
                let hint_x = (super::SCREEN_WIDTH - hint_width as f32) / 2.0;
                let hint_y = subtitle_y + 40.0;

                d.draw_text(
                    hint,
                    hint_x as i32,
                    hint_y as i32,
                    hint_size,
                    Color::DARKGRAY,
                );
            }
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
        self.move_timer = 0.0;
        self.game_over = false;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
