use raylib::prelude::*;
use std::collections::VecDeque;

use crate::core::{Direction, MAP_SIZE, TILE_SIZE};

pub struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut body = VecDeque::new();

        body.push_back(Vector2::new(4.0, MAP_SIZE * 0.5));
        body.push_back(Vector2::new(3.0, MAP_SIZE * 0.5));
        body.push_back(Vector2::new(2.0, MAP_SIZE * 0.5));

        Snake {
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
    }

    pub fn update(&mut self) {
        self.body.pop_back();
        let mut new_head = self.body[0];

        if self.next_direction != self.direction {
            if !self.direction.is_opposite(&self.next_direction) {
                self.direction = self.next_direction;
            }
        }

        match self.direction {
            Direction::Right => {
                new_head.x += 1.0;
                if new_head.x >= MAP_SIZE {
                    new_head.x = 0.0;
                }
            }
            Direction::Down => {
                new_head.y += 1.0;
                if new_head.y >= MAP_SIZE {
                    new_head.y = 0.0;
                }
            }
            Direction::Left => {
                new_head.x -= 1.0;
                if new_head.x < 0.0 {
                    new_head.x = MAP_SIZE - 1.0;
                }
            }
            Direction::Up => {
                new_head.y -= 1.0;
                if new_head.y < 0.0 {
                    new_head.y = MAP_SIZE - 1.0;
                }
            }
        }

        self.body.push_front(new_head);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for (i, tile) in self.body.iter().enumerate() {
            let color = if i == 0 {
                Color::GREEN
            } else {
                Color::RAYWHITE
            };

            d.draw_rectangle_rec(
                Rectangle {
                    x: tile.x * TILE_SIZE,
                    y: tile.y * TILE_SIZE,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                },
                color,
            );
        }
    }

    pub fn head_position(&self) -> Vector2 {
        self.body[0]
    }

    pub fn check_self_collision(&self) -> bool {
        self.body.iter().skip(1).any(|tile| *tile == self.body[0])
    }

    pub fn grow(&mut self) {
        if let Some(tail) = self.body.back() {
            self.body.push_back(*tail);
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}
