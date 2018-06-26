extern crate gl;
extern crate nalgebra;

use super::game_object::{GameObject, GameObjectBuilder};
use utilities::texture::Texture;

use std::convert::{AsRef, AsMut};
use std::rc::Rc;
use std::cell::RefCell;

use gl::types::{GLfloat, GLuint};
use nalgebra::base::{Vector2};


pub const BALL_RADIUS : GLfloat = 12.5;
pub const BALL_VELOCITY_X : GLfloat = 100.0;
pub const BALL_VELOCITY_Y : GLfloat = 350.0;

pub struct BallObject {
    pub(super) object: GameObject,
    pub(super) radius: GLfloat,
    pub(super) is_stuck: bool
}


impl AsMut<GameObject> for BallObject {
    fn as_mut(&mut self) -> &mut GameObject {
        &mut self.object
    }
}


impl AsRef<GameObject> for BallObject {
    fn as_ref(&self) -> &GameObject {
            &self.object
    }
}

impl BallObject {
    pub fn new(position: Vector2<GLfloat>, radius: GLfloat, velocity: Vector2<GLfloat>, sprite: &Rc<RefCell<Texture>>) -> Self {
       let mut builder = GameObject::new(sprite); 
       builder.with_position(position)
              .with_velocity(velocity);
       let builder = builder.build();

       BallObject {
            object: builder,
            radius: radius,
            is_stuck: false
       }

    }

    pub fn update(&mut self, dt : f32, window_width: GLuint, window_height: GLuint) -> &Vector2<GLfloat> {
        let window_width : f32 = window_width as f32;
        let window_height : f32 = window_height as f32;

        if !self.is_stuck {
            self.object.position += self.object.velocity * dt;            

            if self.object.position.x <= 0.0 {
                self.object.velocity.x *= -1.0;
                self.object.position.x = 0.0;
            } else if self.object.position.x + self.object.size.x >= window_width {
                self.object.velocity.x *= -1.0;
                self.object.position.x = window_width - self.object.size.x;
            }
            if self.object.position.y >= window_height {
                self.object.velocity.y *= -1.0;
                self.object.position.x = window_height;
            }
        }
        &self.object.position
    }


    pub fn reset(&mut self, position: &Vector2<GLfloat>, velocity: &Vector2<GLfloat>) {
        self.object.position.x = position.x;
        self.object.position.y = position.y;

        self.object.velocity.x = velocity.x;
        self.object.velocity.y = velocity.y;

        self.is_stuck = true;
    }
}
