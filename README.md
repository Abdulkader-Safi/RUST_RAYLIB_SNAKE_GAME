# Snake Game

A classic Snake game built with Rust and Raylib.

## Features

* Classic snake gameplay with wrap-around edges
* Simple controls using arrow keys
* Score increases by eating food (red squares)
* Game over on self-collision

## Requirements

* Rust (1.70+)
* Raylib dependencies (automatically handled by `raylib-rs`)

## Clone and Run

```bash
# Clone the repository
git clone https://github.com/Abdulkader-Safi/RUST_RAYLIB_SNAKE_GAME.git
cd RUST_RAYLIB_SNAKE_GAME

# Build and run
cargo run --release
```

## Controls

| Key         | Action     |
| ----------- | ---------- |
| Arrow Up    | Move up    |
| Arrow Down  | Move down  |
| Arrow Left  | Move left  |
| Arrow Right | Move right |

## Configuration

You can modify these constants in `src/main.rs`:

```rust
const TILE_SIZE: f32 = 32.0;   // Size of each grid cell
const MAP_SIZE: f32 = 20.0;    // Grid dimensions (20x20)
const GAME_SPEED: u32 = 5;     // Lower = faster
```
