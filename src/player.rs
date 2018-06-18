extern crate gl;
extern crate nalgebra;

use super::game_object::{GameObject, GameObjectBuilder};
use super::texture::Texture;
use super::shader::Shader;

use std::convert::{AsRef,AsMut};
use std::rc::Rc;
use std::cell::RefCell;


use nalgebra::base::{Vector2};
use gl::types::{GLfloat};

pub struct Player {
   object: GameObject,
}

impl Player {

    pub fn new(position : &Vector2<GLfloat>, sprite: &Rc<RefCell<Texture>>) -> Player {
        let mut object = GameObject::new(sprite);
        object
            .with_position(position)
            .with_size(&mut Vector2::new(100.0, 20.0));
        Player {
            object:  object.build()
        }
    }

}

impl AsMut<GameObject> for Player {
    fn as_mut(&mut self) -> &mut GameObject {
        &mut self.object
    }
}
