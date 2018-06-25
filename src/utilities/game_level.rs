extern crate gl;
extern crate nalgebra;

use utilities::sprite_renderer::SpriteRenderer;
use objects::game_object::{GameObject, GameObjectBuilder};
use systems::resource_manager::ResourceManager;
use utilities::game_serialization::{read_from_file, read_from_string};

use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::{Cursor, Read, BufRead, BufReader,};

use gl::types::{GLuint, GLfloat};
use nalgebra::base::{Matrix4, Vector2, Vector3};


pub struct GameLevel {
    bricks: Vec<GameObject>,
    resource_manager: Rc<RefCell<ResourceManager>>,
    level_height: GLuint,
    level_width: GLuint,
}


fn create_objects(resource_manager : &ResourceManager,
                  level: &Vec<u32>,
                  level_width: GLuint, level_height: GLuint,
                  screen_width: GLuint, screen_height: GLuint) -> Result<Vec<GameObject>, String> {

    let unit_width = (screen_width as f32) / (level_width as f32);
    let unit_height = (screen_height as f32) / (level_height as f32);

    let base_color = Vector3::new(0.8, 0.8, 0.7);
    let size_vector = Vector2::new(unit_width, unit_height);
    let mut bricks = Vec::new();
    let mut base_pos = Vector2::new(0.0, 0.0);

    for j in 0..level_height {
        for i in 0..level_width {
            let index = j * level_width + i;
            let value = *level.get(index as usize).ok_or(format!("invalid Game level input array"))?;
                 {
                    base_pos.x = unit_width * i as GLfloat;
                    base_pos.y = (screen_height as f32) - unit_height * j as GLfloat
                }
                let pos = &base_pos;
                let size = &size_vector;
            if value == 1 {
                let texture = resource_manager.get_texture("block_solid").ok_or("Could not load block_solid texture.")?;
                let mut obj = GameObject::new(&texture);
                obj.with_position(pos)
                    .with_size(size)
                    .with_color(&base_color)
                    .with_is_solid(true);
                let obj = obj.build();
                bricks.push(obj);
            } else if value > 1 {

                let texture = resource_manager.get_texture("block").ok_or("Could not load block texture")?;
                let mut color = Vector3::new(1.0, 1.0, 1.0);

                if value == 2 {
                    color.x = 0.2;
                    color.y = 0.6;
                    color.y = 1.0;
                } else if value == 3 {
                    color.x = 0.0;
                    color.y = 0.7;
                    color.y = 0.0;
                } else if value == 4 {
                    color.x = 0.8;
                    color.y = 0.8;
                    color.y = 0.4;
                } else if value == 5 {
                    color.x = 1.0;
                    color.y = 0.5;
                    color.y = 0.0;
                } else {
                    color.x = {
                        let tmp = 0.2 * (value as GLfloat);

                        if tmp > 1.0 {
                            1.0
                        } else {
                            tmp
                        }
                    };
                    color.y = {
                        let tmp = 0.6 / (value as GLfloat);

                        if tmp > 1.0 {
                            1.0
                        } else {
                            tmp
                        }
                    };
                    color.y = {
                        let tmp = 1.0 * (value as GLfloat);

                        if tmp > 1.0 {
                            1.0
                        } else {
                            tmp
                        }
                    };
                }

                let mut obj = GameObject::new(&texture);
                obj.with_position(pos)
                    .with_size(size)
                    .with_color(&color);
                let obj = obj.build();
                bricks.push(obj);
            }
        }
    }

    Ok(bricks)
}

fn update_scaling(objects: &mut Vec<GameObject>,
                  level_width: GLuint, level_height: GLuint,
                  screen_width: GLuint, screen_height: GLuint) {
    let unit_width = (screen_width as f32) / (level_width as f32);
    let unit_height = (screen_height as f32) / (level_height as f32);

    let base_color = Vector3::new(0.8, 0.8, 0.7);
    let size_vector = Vector2::new(unit_width, unit_height);
    let mut base_pos = Vector2::new(0.0, 0.0);

    for j in 0..level_height {
        for i in 0..level_width {
            let index = j * level_width + i;

            {
                base_pos.x = (unit_width * i as GLfloat);
                base_pos.y = (unit_height * j as GLfloat);
            }

            let mut obj : &mut GameObject = objects.get_mut(index as usize).expect("invalid Game level input array");
            let pos = &base_pos;
            let size = &size_vector;
            let rotation = obj.get_rotation();

            obj.update_transform(pos, size, rotation);

        }
    }
}


impl GameLevel {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>,
               elements: &Vec<u32>,
               level_width: GLuint,
               level_height: GLuint,
               screen_width: GLuint,
               screen_height: GLuint) -> Result<Self, String> {
        let objects = create_objects(
           &resource_manager.borrow(),
            elements,
            level_width,
            level_height,
            screen_width,
            screen_height )?;
        Ok(GameLevel {
            bricks: objects,
            resource_manager: resource_manager.clone(),
            level_height: level_height,
            level_width: level_width,
        })
    }

    pub fn from_file<T : AsRef<Path>>(resource_manager: &Rc<RefCell<ResourceManager>>,
                                      file : &T,
                                      screen_width: GLuint,
                                      screen_height: GLuint) -> Result<GameLevel, String> {
        let (level,(level_width, level_height)) = read_from_file(file)?;
        GameLevel::new(
            resource_manager,
            &level,
            level_width,
            level_height,
            screen_width,
            screen_height
        )
    }

    pub fn from_string(resource_manager: &Rc<RefCell<ResourceManager>>,
                       level : &str,
                       screen_width: GLuint,
                       screen_height: GLuint) -> Result<GameLevel, String> {
        let(level,(level_width, level_height)) = read_from_string(level)?;
         GameLevel::new(
            resource_manager,
            &level,
            level_width,
            level_height,
            screen_width,
            screen_height
        )
    }

    pub fn update_screen_size(&mut self, screen_width: GLuint, screen_height: GLuint) {

        let level_width = self.level_width;
        let level_height = self.level_height;

        update_scaling(&mut self.bricks, level_width, level_height, screen_width, screen_height);
    }


    pub fn draw(&mut self, renderer: &mut SpriteRenderer) {
        for brick in self.bricks.iter_mut() {
            brick.draw(renderer);
        }
    }

    pub fn is_completed(&self) -> bool {
        for brick in self.bricks.iter() {
            if !brick.is_completed() {
                return false;
            }
        }
        true
    }
}
