
use systems::resource_manager::ResourceManager;
use utilities::sprite_renderer::SpriteRenderer;
use utilities::texture::Texture;
use utilities::game_level::GameLevel;
use objects::player::Player;
use objects::ball::BallObject;
use systems::input_manager::{Input, parse_input};

use std::cell::RefCell;
use std::ptr::null;
use std::rc::Rc;

use gl::types::GLfloat;
use nalgebra::base::{Matrix4, Vector2, Vector3};
use sdl2::event::Event;


#[derive(Clone)]
pub enum GameState {
    GAME_ACTIVE,
    GAME_MENU,
    GAME_WIN,
}

pub struct Game {
    state: GameState,
    width: u32,
    height: u32,
    resource_manager: Rc<RefCell<ResourceManager>>,
    levels: Vec<GameLevel>,
    current_level: Option<usize>,
    renderer: Option<SpriteRenderer>,
    player: Option<Player>,
    ball: Option<BallObject>
}

static mut RENDERER: Option<SpriteRenderer> = None;

impl Game {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>) -> Self {
        Game {
            state: GameState::GAME_ACTIVE,
            width: 700,
            height: 900,
            resource_manager: resource_manager.clone(),
            levels: Vec::new(),
            current_level: None,
            renderer: None,
            player: None,
            ball: None
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
                ("textures/block_solid.png", false, "block_solid"),
                ("/home/gopiandcode/Documents/Rust/gui-base/textures/awesomeface.png", true, "face"),
                ("textures/paddle.png", true, "paddle")
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


        // setup player
        {
            let _resource_manager = self.resource_manager.borrow();
            let texture = _resource_manager.get_texture("paddle").expect("Could not load paddle texture");
            self.player = Some(Player::new(Vector2::new((self.width as f32/2.0), 10.0), (width as f32, height as f32), &texture));
        }

        {
            let _resource_manager = self.resource_manager.borrow();
            let texture = _resource_manager.get_texture("face").expect("Could not load face texture");
            let position = Vector2::new(
                (self.width as f32/2.0) +  ::objects::player::PLAYER_SIZE_X / 2.0 - ::objects::ball::BALL_RADIUS, 
                ::objects::ball::BALL_RADIUS * 2.0);
            let radius = ::objects::ball::BALL_RADIUS;
            let velocity = Vector2::new(::objects::ball::BALL_VELOCITY_X, ::objects::ball::BALL_VELOCITY_Y);

            
            self.ball = Some(BallObject::new(position, radius, velocity, &texture));
        }


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


    pub fn processInput(&mut self, dt: f32, events: &Vec<Event>) {
        let mut player : &mut Player = self.player.as_mut().expect("Game error -render called before player initialized");
        match &self.state {
            ref GAME_ACTIVE => {
                for event in events { println!("{:?}", event)}

               for input in parse_input(events) {
                   println!("Parsed: {:?}", input);

                   match input {
                       Input::LEFT => {
                            // pass input to player
                            player.move_left(dt);
                       }
                       Input::RIGHT => {

                           // pass input to player
                            player.move_right(dt);
                       }
                       Input::DOWN => {

                           // pass input to player
                       }
                       Input::UP => {

                           // pass input to player
                       }
                       _ => ()

                   }
               }
           }
            _ => return
        }
    }

    pub fn update(&mut self, dt: f32) {

        let mut player : &mut Player = self.player.as_mut().expect("Game error render called before player initialized");
        let mut ball : &mut BallObject = self.ball.as_mut().expect("Game error render called before ball initialized");
        println!("ball_pos: {:?}", ball.update(dt, self.width, self.height));

        // update player state.
    }


    pub fn render(&mut self) {
        let mut screen: &mut SpriteRenderer = self.renderer.as_mut().expect("Game error - render called before init");
        let mut player : &mut Player = self.player.as_mut().expect("Game error -render called before player initialized");
        let mut ball : &mut BallObject = self.ball.as_mut().expect("Game error render called before ball initialized");
        let state = self.state.clone();

        match state {
            GAME_ACTIVE => {
                let texture: Rc<RefCell<Texture>> =
                    self.resource_manager.borrow().get_texture("background")
                        .expect("Game error - could not load background image");

                let height = self.height;
                let width = self.width;
                let index = self.current_level.expect("Game error - No Current Level");
                let mut level = &mut self.levels[index];
                screen.draw_sprite_transformed(
                    &texture.borrow(),
                    &Vector2::new(0.0, 0.0),
                    &Vector2::new(width as f32, height as f32),
                    0.0,
                    &Vector3::new(1.0, 1.0, 1.0),
                );

                level.draw(&mut screen);
                player.as_mut().draw(&mut screen);
                ball.as_mut().draw(&mut screen);
            }
            _ => return
        }
    }

}
