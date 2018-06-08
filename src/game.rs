enum GameState {
    GAME_ACTIVE,
    GAME_MENU,
    GAME_WIN
}

struct Game {
   state : GameState,
    keys : [bool; 1024],
    width: u32,
    height: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::GAME_ACTIVE,
            keys: [true; 1024],
            width: 1024,
            height: 640
        }
    }
    pub fn init() { }
    pub fn processInput(dt : f32) {}
    pub fn update(dt: f32) {}
    pub fn render() {}

}
