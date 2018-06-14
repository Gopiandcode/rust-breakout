extern crate gl;
extern crate nalgebra;

use super::resource_manager::ResourceManager;
use super::sprite_renderer::SpriteRenderer;
use super::texture::Texture;

use std::cell::RefCell;
use std::ptr::null;
use std::rc::Rc;

use gl::types::GLfloat;
use nalgebra::base::{Matrix4, Vector2, Vector3};

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
        }
    }
    pub fn init(&mut self) {
        let shader = self.resource_manager.borrow_mut().load_shader(
            "/home/gopiandcode/Documents/Rust/gui-base/shaders/sprite.vs",
            "/home/gopiandcode/Documents/Rust/gui-base/shaders/sprite.frag",
            "sprite",
        ).expect("sprite shader could not be loaded");

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
            // _shader.setInt("image", 0);
            _shader.setMatrix4("projection", &projection);
            }
            RENDERER = Some(SpriteRenderer::new(shader));
        }

        self.resource_manager
            .borrow_mut()
            .load_texture("/home/gopiandcode/Documents/Rust/gui-base/textures/awesomeface.png", true, "face");
    }
    pub fn processInput(&mut self, dt: f32) {}
    pub fn update(&mut self, dt: f32) {}
    pub fn render(&mut self) {
        unsafe {
            if let Some(ref mut screen) = RENDERER {
                let texture: Rc<RefCell<Texture>> =
                    self.resource_manager.borrow().get_texture("face").unwrap();
                screen.draw_sprite_transformed(
                    &texture.borrow(),
                    &Vector2::new(200.0, 200.0),
                    &Vector2::new(300.0, 400.0),
                    45.0,
                    &Vector3::new(0.0, 1.0, 0.0),
                );
            }
        }
    }
}
