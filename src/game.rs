extern crate gl;
extern crate nalgebra;

use super::resource_manager::ResourceManager;
use super::sprite_renderer::SpriteRenderer;
use super::texture::Texture;
use super::game_level::GameLevel;

use std::cell::RefCell;
use std::ptr::null;
use std::rc::Rc;

use gl::types::GLfloat;
use nalgebra::base::{Matrix4, Vector2, Vector3};

#[derive(Clone)]
pub enum GameState {
    GAME_ACTIVE,
    GAME_MENU,
    GAME_WIN,
}

pub struct Game {
    state: GameState,
    keys: [bool; 1024],
    width: u32,
    height: u32,
    resource_manager: Rc<RefCell<ResourceManager>>,
    levels: Vec<GameLevel>,
    current_level: Option<usize>,
    renderer: Option<SpriteRenderer>
}

static mut RENDERER: Option<SpriteRenderer> = None;

impl Game {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>) -> Self {
        Game {
            state: GameState::GAME_ACTIVE,
            keys: [true; 1024],
            width: 700,
            height: 900,
            resource_manager: resource_manager.clone(),
            levels: Vec::new(),
            current_level: None,
            renderer: None
        }
    }
    pub fn init(&mut self) { // Loading resources
        let shader = self.resource_manager.borrow_mut().load_shader(
            "/home/gopiandcode/Documents/Rust/gui-base/shaders/sprite.vs",
            "/home/gopiandcode/Documents/Rust/gui-base/shaders/sprite.frag",
            "sprite",
        ).expect("sprite shader could not be loaded");
        {
            let mut manager = self.resource_manager.borrow_mut();
            for (filename, is_alpha, reference) in &[
                ("textures/background.jpg", false, "background"),
                ("textures/block.png", false, "block"),
                ("textures/block_solid.png", false,  "block_solid"),
                ("/home/gopiandcode/Documents/Rust/gui-base/textures/awesomeface.png", true, "face"),
            ] {

                manager.load_texture(filename, *is_alpha, reference)
                    .expect(&format!("Could not find file {}", filename));
            }
        }

        let width = self.width;
        let height = self.height;
        for file_name in &["levels/one.lvl", "levels/two.lvl", "levels/three.lvl", "levels/four.lvl"] {
            let mut level = GameLevel::from_file(&self.resource_manager, file_name, width, height)
                .expect(&format!("Could not open file {}", file_name));

            self.levels.push(level);
        }
        self.current_level = Some(0);
        self.renderer = Some(SpriteRenderer::new(&shader));


        let projection = Matrix4::new_orthographic(
            0.0,                    // left
            self.width as GLfloat,  // right
            0.0,                    // bottom
            self.height as GLfloat, // top
            -1.0,                   // znear
            1.0,                    // zfar
        );


        unsafe {
            {
            let mut _shader = shader.borrow_mut();
            _shader.enable();
            _shader.setInt("image", 0);
            _shader.setMatrix4("projection", &projection);
            }
        }


    }
    pub fn processInput(&mut self, dt: f32) {}
    pub fn update(&mut self, dt: f32) {}
    pub fn render(&mut self) {
        let mut screen : &mut SpriteRenderer = self.renderer.as_mut().expect("Game error - render called before init");
        let state = self.state.clone();

        match state {
         GAME_ACTIVE => {
             let texture: Rc<RefCell<Texture>> =
                    self.resource_manager.borrow().get_texture("background")
                        .expect("Game error - could not load background image");

             let height = self.height;
             let width = self.width;
             let index = self.current_level.expect("Game error - No Current Level");
             let mut level =  &mut self.levels[index];
             screen.draw_sprite_transformed(
                 &texture.borrow(),
                 &Vector2::new(0.0, 0.0),
                 &Vector2::new(width as f32, height as f32),
                 0.0,
                 &Vector3::new(1.0, 1.0, 1.0),
                );

             level.draw(&mut screen);

         },
           _ => return
        }

    }
}
