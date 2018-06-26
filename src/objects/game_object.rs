
use utilities::sprite_renderer::SpriteRenderer;
use utilities::texture::Texture;

use std::cell::RefCell;
use std::rc::Rc;

use gl::types::GLfloat;
use nalgebra::base::{Matrix4, Unit, Vector2, Vector3, Vector4};

pub struct GameObject {
    pub(super) position: Vector2<GLfloat>,
    pub(super) size: Vector2<GLfloat>,
    pub(super) velocity: Vector2<GLfloat>,
    pub(super) rotation: GLfloat,
    pub(super) is_solid: bool,
    pub(super) is_destroyed: bool,
    pub(super) sprite: Rc<RefCell<Texture>>,
    pub(super) color: Vector3<GLfloat>,
}

pub struct GameObjectBuilder {
    position: Option<Vector2<GLfloat>>,
    size: Option<Vector2<GLfloat>>,
    velocity: Option<Vector2<GLfloat>>,
    rotation: Option<GLfloat>,
    is_solid: Option<bool>,
    is_destroyed: Option<bool>,
    sprite: Rc<RefCell<Texture>>,
    color: Option<Vector3<GLfloat>>,
}

impl GameObjectBuilder {
    fn new(sprite: &Rc<RefCell<Texture>>) -> Self {
        GameObjectBuilder {
            position: None,
            size: None,
            velocity: None,
            rotation: None,
            is_solid: None,
            is_destroyed: None,
            color: None,
            sprite: sprite.clone(),
        }
    }

    pub fn with_position(&mut self, position: Vector2<GLfloat>) -> &mut Self {
        self.position = Some(position);
        self
    }

    pub fn with_size(&mut self, size: Vector2<GLfloat>) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn with_velocity(&mut self, velocity: Vector2<GLfloat>) -> &mut Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn with_is_solid(&mut self, is_solid: bool) -> &mut Self {
        self.is_solid = Some(is_solid);
        self
    }
    pub fn with_is_destroyed(&mut self, is_destroyed: bool) -> &mut Self {
        self.is_destroyed = Some(is_destroyed);
        self
    }
    pub fn with_color(&mut self, color: Vector3<GLfloat>) -> &mut Self {
        self.color = Some(color);
        self
    }


    pub fn with_rotation(&mut self, rotation: GLfloat) -> &mut Self {
        self.rotation = Some(rotation);
        self
    }


    pub fn build(self) -> GameObject {
        let mut position = self.position.unwrap_or(Vector2::new(0.0, 0.0));
        let mut size = self.size.unwrap_or(Vector2::new(1.0, 1.0));
        let mut velocity = self.velocity.unwrap_or(Vector2::new(0.0, 0.0));
        let mut rotation = self.rotation.unwrap_or(0.0);
        let mut is_solid = self.is_solid.unwrap_or(false);
        let mut is_destroyed = self.is_destroyed.unwrap_or(false);
        let mut sprite = self.sprite;
        let mut color = self.color.unwrap_or(Vector3::new(1.0, 1.0, 1.0));

        GameObject::construct(position, size, velocity, rotation, is_solid, is_destroyed, sprite, color)
    }
}

impl GameObject {
    pub fn new(sprite: &Rc<RefCell<Texture>>) -> GameObjectBuilder {
        GameObjectBuilder::new(sprite)
    }

    fn construct(
        position: Vector2<GLfloat>,
        size: Vector2<GLfloat>,
        velocity: Vector2<GLfloat>,
        rotation: GLfloat,
        is_solid: bool,
        is_destroyed: bool,
        sprite: Rc<RefCell<Texture>>,
        color: Vector3<GLfloat>,
    ) -> Self {
        GameObject {
            position: position,
            size: size,
            velocity: velocity,
            rotation: rotation,
            is_solid: is_solid,
            is_destroyed: is_destroyed,
            sprite: sprite,
            color: color,
        }
    }

    pub fn get_rotation(&self) -> GLfloat {
        self.rotation
    }


    pub fn update_transform(&mut self, position: &Vector2<GLfloat>, size: &Vector2<GLfloat>, rotation : GLfloat) {
        self.position.x = position.x;
        self.position.y = position.y;
        self.size.x = size.x;
        self.size.y = size.y;
        self.rotation = rotation;
    }


    pub fn is_completed(&self) -> bool {
        self.is_solid || self.is_destroyed
    }

    pub fn position(&self) -> &Vector2<GLfloat> {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vector2<GLfloat> {
        &mut self.position
    }


    pub fn draw(&mut self, renderer: &mut SpriteRenderer) {
        if !self.is_destroyed  {
            renderer.draw_sprite_transformed(&self.sprite.borrow(), &self.position, &self.size, self.rotation, &self.color);
        }
    }
}
