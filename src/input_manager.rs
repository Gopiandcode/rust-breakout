extern crate gl;
extern crate nalgebra;
extern crate sdl2;

use super::resource_manager::ResourceManager;
use super::sprite_renderer::SpriteRenderer;
use super::texture::Texture;
use super::game_level::GameLevel;
use super::player::Player;

use std::cell::RefCell;
use std::ptr::null;
use std::rc::Rc;

use gl::types::GLfloat;
use nalgebra::base::{Matrix4, Vector2, Vector3};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[derive(Debug)]
pub enum Input {
   UP,
    DOWN,
    LEFT,
    RIGHT,
    ACTION0
}

pub fn parse_input(elements: &Vec<Event>) -> Vec<Input> {
    elements.iter().filter_map(|elem|
        if let &Event::KeyDown {
            keycode: Some(ref keycode),
            ..
        } = elem {
            println!("keycode: {:?}", keycode);
           match keycode {
               Keycode::D => Some(Input::RIGHT),
               Keycode::A => Some(Input::LEFT),
               Keycode::S => Some(Input::DOWN),
               Keycode::W => Some(Input::UP),
               Keycode::Right => Some(Input::RIGHT),
               Keycode::Left => Some(Input::LEFT),
               Keycode::Up   => Some(Input::UP),
               Keycode::Down => Some(Input::DOWN),
               Keycode::J => Some(Input::ACTION0),
               _ => None
           }
        } else {None}).collect()

}
