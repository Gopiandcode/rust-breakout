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
            width: 1024,
            height: 640,
            resource_manager: resource_manager.clone(),
        }
    }
    pub fn init(&mut self) {
        let shader = self.resource_manager.borrow_mut().load_shader(
            "./shaders/sprite.vs",
            "./shaders/sprite.frag",
            "sprite",
        ).expect("sprite shader could not be loaded");

        let projection = Matrix4::new_orthographic(
            0.0,                    // left
            self.width as GLfloat,  // right
            0.0,                    // bottom
            self.height as GLfloat, // top
            -1.0,                   // znear
            1.0,                    // zfar
        );/*{
            let l = 0.0;
            let r = self.width as GLfloat;
            let b = self.height as GLfloat;
            let t = 0.0;
            let n = -1.0;
            let f = 1.0;

            let n2 = 2.0 * n;
            let r_l = r-l;
            let rpl = r+l;
            let f_n = f-n;
            let fpn = f+n;
            let t_b = t-b;
            let tpb = t + b;

            Matrix4::new(
//                n2/r_l, 0.0, 0.0, 0.0,
//                0.0, n2/t_b, 0.0, 0.0,
//                rpl/r_l, tpb/t_b, -fpn/f_n, -1.0,
//                0.0,  0.0, -2.0*(f*n)/f_n,  0.0,
                n2/r_l,   0.0,  rpl/r_l, 0.0,
                0.0,   n2/t_b,  tpb/t_b, 0.0,
                0.0,      0.0, -fpn/f_n, -2.0*(f*n)/f_n,
                0.0,      0.0,     -1.0, 0.0,
            )
        };*/

        println!("Perspective_matrix: {:?}", projection);
        // [0.001953125,      0.0,     0.0,    0.0,
        //  0.0,         0.003125,     0.0,    0.0,
        //  0.0,              0.0,    -1.0,    0.0,
        // -1.0,             -1.0,    -0.0,    1.0]

        unsafe {
            {
            let mut _shader = shader.borrow_mut();
            _shader.enable();
            _shader.setInt("image", 0);
            _shader.setMatrix4("projection", &projection);
            }
            RENDERER = Some(SpriteRenderer::new(shader));
        }

        self.resource_manager
            .borrow_mut()
            .load_texture("textures/awesomeface.png", true, "face");
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
