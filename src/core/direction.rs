#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        *self == other.opposite()
    }
}
