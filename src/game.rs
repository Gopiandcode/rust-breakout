use super::resource_manager::ResourceManager;
use std::cell::RefCell;
use std::rc::Rc;

pub enum GameState {
    GAME_ACTIVE,
    GAME_MENU,
    GAME_WIN
}

pub struct Game {
   state : GameState,
    keys : [bool; 1024],
    width: u32,
    height: u32,
    resource_manager: Rc<RefCell<ResourceManager>>
}

impl Game {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>) -> Self {
        Game {
            state: GameState::GAME_ACTIVE,
            keys: [true; 1024],
            width: 1024,
            height: 640,
            resource_manager: resource_manager.clone()
        }
    }
    pub fn init(&mut self) { }
    pub fn processInput(&mut self, dt : f32) {}
    pub fn update(&mut self, dt: f32) {}
    pub fn render(&mut self) {}

}
