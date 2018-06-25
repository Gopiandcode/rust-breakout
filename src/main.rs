extern crate gl;
extern crate nalgebra;
extern crate sdl2;

mod objects;
mod systems;
mod utilities;
mod game;

use game::Game;
use systems::resource_manager::ResourceManager;
use utilities::timer::Timer;

use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::base::Matrix4;
use sdl2::event::Event;

fn main() {
    // configure SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl
        .video()
        .expect("| ERROR:INITIALIZATION: Could not initialize SDL video subsystem.");
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .expect("| ERROR::INITIALIZATION: Could not start window.");

    let gl_context = window.gl_create_context().unwrap();
    let gl_ =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();

    let mut resource_manager = Rc::new(RefCell::new(ResourceManager::new()));
    let mut game = /* TestGame::new(&resource_manager);*/  Game::new(&resource_manager); 
    let mut timer = Timer::new();

    // configure OpenGL
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    game.init();

    let mut delta_time = 0.0f32;
    let mut last_frame = 0.0f32;

    'main: loop {
        let current_frame = timer.get_time();
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        let mut event_iterator = event_pump .poll_iter();

        let events : Vec<_> = event_iterator.collect();

        if events
            .iter()
            .any(|event| if let Event::Quit { .. } = event { true } else { false }) {
            break 'main;
        }


        game.processInput(delta_time, &events);

        game.update(delta_time);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        game.render();

        window.gl_swap_window();
    }

    resource_manager.borrow_mut().clear();
}
