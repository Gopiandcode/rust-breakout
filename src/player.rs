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
    screen_width: GLfloat, 
    screen_height: GLfloat
}

static PLAYER_VELOCITY: GLfloat = 5.0;
static PLAYER_SIZE_X: GLfloat = 100.0;
static PLAYER_SIZE_Y: GLfloat = 20.0;

impl Player {

    pub fn new(position : &Vector2<GLfloat>, (screen_width, screen_height): (GLfloat, GLfloat), sprite: &Rc<RefCell<Texture>>) -> Player {
        let mut object = GameObject::new(sprite);
        object
            .with_position(position)
            .with_size(&mut Vector2::new(PLAYER_SIZE_X, PLAYER_SIZE_Y));
        Player {
            object:  object.build(),
            screen_width: screen_width,
            screen_height: screen_height
        }
    }

    pub fn move_left(&mut self, dt: GLfloat) {

        let velocity = PLAYER_VELOCITY * dt;
        let x = self.object.position().x; 
        println!("Moving left with position {}", x);

        if x >= 0.0 {
            self.object.position_mut().x -= velocity;
        }

        if x <= 0.0 {
            self.object.position_mut().x = 0.0;
        }

        println!("position is now {}", self.object.position().x);
    }

    pub fn move_right(&mut self, dt: GLfloat) {
        let velocity = PLAYER_VELOCITY * dt;
        let x = self.object.position().x; 
        println!("Moving right with position {}", x);
        let bound = self.screen_width - PLAYER_SIZE_X;


        if x <=  bound {
            self.object.position_mut().x += velocity;
        }
        if x >=  bound {
            self.object.position_mut().x = bound;
        }

        println!("position is now {}", self.object.position().x);
    }

}

impl AsMut<GameObject> for Player {
    fn as_mut(&mut self) -> &mut GameObject {
        &mut self.object
    }
}
