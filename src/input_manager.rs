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
use sdl2::event::Event;

pub enum Input {
   UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn parse_input(elements: &Vec<Event>) -> Vec<Input> {
    elements.iter().filter_map(|elem|
        if let &Event::KeyDown {
            keycode: Some(ref keycode),
            ..
        } = elem {
           match keycode {
               A => Some(Input::LEFT),
               D => Some(Input::RIGHT),
               S => Some(Input::DOWN),
               W => Some(Input::UP),
               _ => None
           }
        } else {None}).collect()

}
