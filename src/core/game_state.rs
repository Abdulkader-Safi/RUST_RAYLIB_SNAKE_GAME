#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    MainMenu,
    Options,
    Playing,
    #[allow(dead_code)]
    Closed,
}
