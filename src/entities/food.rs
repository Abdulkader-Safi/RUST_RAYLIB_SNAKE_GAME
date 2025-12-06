use raylib::prelude::*;

use crate::core::{MAP_SIZE, TILE_SIZE};
use crate::utils::get_random_value;

pub struct Food {
    position: Vector2,
}

impl Food {
    pub fn new() -> Self {
        Food {
            position: Self::random_position(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(
            Rectangle {
                x: self.position.x * TILE_SIZE,
                y: self.position.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE,
            },
            Color::RED,
        );
    }

    pub fn respawn(&mut self) {
        self.position = Self::random_position();
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    fn random_position() -> Vector2 {
        Vector2::new(
            get_random_value(0, MAP_SIZE as i32 - 1) as f32,
            get_random_value(0, MAP_SIZE as i32 - 1) as f32,
        )
    }
}

impl Default for Food {
    fn default() -> Self {
        Self::new()
    }
}
