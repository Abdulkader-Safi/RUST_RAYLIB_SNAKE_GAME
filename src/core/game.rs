use raylib::prelude::*;

use super::{Direction, MOVE_INTERVAL};
use crate::entities::{Food, Snake};

pub struct Game {
    snake: Snake,
    food: Food,
    move_timer: f32,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            snake: Snake::new(),
            food: Food::new(),
            move_timer: 0.0,
            game_over: false,
        }
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle) {
        if self.game_over {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.restart();
            }
            return;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || rl.is_key_pressed(KeyboardKey::KEY_D) {
            self.snake.set_direction(Direction::Right);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
            self.snake.set_direction(Direction::Down);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A) {
            self.snake.set_direction(Direction::Left);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
            self.snake.set_direction(Direction::Up);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
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

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.snake.draw(d);
        self.food.draw(d);

        if self.game_over {
            d.draw_text("GAME OVER!", 200, 250, 40, Color::WHITE);
            d.draw_text("Press SPACE to restart", 170, 310, 25, Color::LIGHTGRAY);
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
