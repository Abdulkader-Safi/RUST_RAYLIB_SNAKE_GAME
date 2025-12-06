use raylib::consts::KeyboardKey::*;
use raylib::ffi::GetRandomValue;
use raylib::prelude::*;

use std::collections::VecDeque;

fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { GetRandomValue(min, max) }
}

const TILE_SIZE: f32 = 32.0;
const MAP_SIZE: f32 = 20.0;

const SCREEN_WIDTH: f32 = MAP_SIZE * TILE_SIZE;
const SCREEN_HEIGHT: f32 = MAP_SIZE * TILE_SIZE;

const GAME_SPEED: u32 = 5;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let mut instance = Snake {
            body: VecDeque::new(),
            direction: Direction::RIGHT,
            next_direction: Direction::RIGHT,
        };

        let position = Vector2::new(4.0, MAP_SIZE * 0.5);
        instance.body.push_back(position);

        let position = Vector2::new(3.0, MAP_SIZE * 0.5);
        instance.body.push_back(position);

        let position = Vector2::new(2.0, MAP_SIZE * 0.5);
        instance.body.push_back(position);

        return instance;
    }

    pub fn update(&mut self) {
        let _ = self.body.pop_back();
        let mut tmp = self.body[0].clone();

        if self.next_direction != self.direction {
            match self.direction {
                Direction::RIGHT => {
                    if self.next_direction == Direction::LEFT {
                        self.next_direction = Direction::RIGHT;
                    }
                }
                Direction::DOWN => {
                    if self.next_direction == Direction::UP {
                        self.next_direction = Direction::DOWN;
                    }
                }
                Direction::LEFT => {
                    if self.next_direction == Direction::RIGHT {
                        self.next_direction = Direction::LEFT;
                    }
                }
                Direction::UP => {
                    if self.next_direction == Direction::DOWN {
                        self.next_direction = Direction::UP;
                    }
                }
            }

            self.direction = self.next_direction;
        }

        match self.direction {
            Direction::RIGHT => {
                tmp.x += 1.0;
                if tmp.x >= MAP_SIZE {
                    tmp.x = 0.0
                }
                self.body.push_front(tmp);
            }
            Direction::DOWN => {
                tmp.y += 1.0;
                if tmp.y >= MAP_SIZE {
                    tmp.y = 0.0
                }
                self.body.push_front(tmp);
            }
            Direction::LEFT => {
                tmp.x -= 1.0;
                if tmp.x < 0.0 {
                    tmp.x = MAP_SIZE - 1.0
                }
                self.body.push_front(tmp);
            }
            Direction::UP => {
                tmp.y -= 1.0;
                if tmp.y < 0.0 {
                    tmp.y = MAP_SIZE - 1.0
                }
                self.body.push_front(tmp);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let mut color = Color::GREEN;

        for tile in self.body.iter() {
            d.draw_rectangle_rec(
                Rectangle {
                    x: tile.x * TILE_SIZE,
                    y: tile.y * TILE_SIZE,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                },
                color,
            );

            if color == Color::GREEN {
                color = Color::RAYWHITE;
            }
        }
    }
}

struct Food {
    position: Vector2,
}

impl Food {
    pub fn new() -> Food {
        Food {
            position: Vector2::new(
                get_random_value(0, (MAP_SIZE as i32) - 1) as f32,
                get_random_value(0, (MAP_SIZE as i32) - 1) as f32,
            ),
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
        self.position = Vector2::new(
            get_random_value(0, (MAP_SIZE as i32) - 1) as f32,
            get_random_value(0, (MAP_SIZE as i32) - 1) as f32,
        )
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello, World!")
        .vsync()
        .build();

    rl.set_target_fps(30);
    let mut frame_count = 0;

    let mut snake = Snake::new();
    let mut food = Food::new();

    while !rl.window_should_close() {
        /*--- INPUT ---*/
        if rl.is_key_pressed(KEY_RIGHT) {
            snake.next_direction = Direction::RIGHT;
        }
        if rl.is_key_pressed(KEY_DOWN) {
            snake.next_direction = Direction::DOWN;
        }
        if rl.is_key_pressed(KEY_LEFT) {
            snake.next_direction = Direction::LEFT;
        }
        if rl.is_key_pressed(KEY_UP) {
            snake.next_direction = Direction::UP;
        }

        if frame_count % GAME_SPEED == 0 {
            snake.update();
        }

        for tile in snake.body.iter().skip(1) {
            if snake.body[0] == *tile {
                return;
            }
        }

        if snake.body[0] == food.position {
            food.respawn();
            let tmp = snake.body.back().unwrap().clone();
            snake.body.push_back(tmp);
        }

        /*--- DRAW ---*/
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        snake.draw(&mut d);
        food.draw(&mut d);

        frame_count += 1;

        d.draw_fps(10, 10);
    }
}
