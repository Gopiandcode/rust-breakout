extern crate gl;

use gl::types::{GLuint, GLint};

use std::collections::hash_map::HashMap;

use std::fs::File;
use std::io::Read;
use std::mem::swap;

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_void};
use std::ptr::null_mut;

use super::shader::{Shader,string_to_glchar};
use super::texture::Texture;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/SOIL_bindings.rs"));

pub struct ResourceManager {
    shaders: HashMap<String, Shader>,
    textures: HashMap<String, Texture>,
}


impl ResourceManager {
    pub fn load_shader(&mut self, vertex_file: &str, fragment_file: &str, name: &str) -> &Shader {
        self.shaders.insert(name.to_string(), ResourceManager::load_shader_from_file(vertex_file, fragment_file));
        return &self.shaders[name];
    }

    pub fn get_shader(&mut self, name: &str) -> &Shader {
        return &self.shaders[name];
    }

    pub fn load_texture(&mut self, vertex_file: &str, is_alpha: bool, name: &str) -> &Texture {
        self.textures.insert(name.to_string(),  ResourceManager::load_texture_from_file(vertex_file, is_alpha));
        &self.textures[name]
    }

    pub fn get_texture(&self, name: &str) -> &Texture {
        &self.textures[name]
    }

    pub fn clear(&mut self) {
        {
            let mut new_shaders = HashMap::new();
            swap(&mut new_shaders, &mut self.shaders);

            for (_, mut shader) in new_shaders.iter_mut() {
                unsafe {
                    shader.delete();
                }
            }
        }

        {
            let mut new_textures = HashMap::new();
            swap(&mut new_textures, &mut self.textures);

            for (_, mut texture) in new_textures.iter_mut() {
                unsafe {
                    texture.delete();
                }
            }
        }
    }


    pub fn load_shader_from_file(vertex_file: &str, fragment_file: &str) -> Shader {
        let mut vertex_string = String::new();
        let mut fragment_string = String::new();

        {
            let mut vertex = File::open(vertex_file).expect("| ERROR::RESOURCE_MANAGER: Vertex shader not found");
            let mut fragment = File::open(fragment_file).expect("| ERROR::RESOURCE_MANAGER: Fragment shader not found");
            vertex.read_to_string(&mut vertex_string);
            fragment.read_to_string(&mut fragment_string);
        }

        let mut vertex_source = string_to_glchar(vertex_string.as_bytes());
        let mut fragment_source = string_to_glchar(fragment_string.as_bytes());

        let mut shader = Shader::new();

        unsafe {
            shader.compile(&vertex_source, &fragment_source);
        }

        shader
    }

    pub fn load_texture_from_file(file: &str, alpha: bool) -> Texture {
        let mut texture = Texture::new();

        let mut file = CString::new(file).expect("| ERROR::RESOURCE_MANAGER: Invalid file string for texture.");
        let mut file = file.as_ptr();
        let mut width : c_int = 0;
        let mut height : c_int = 0;
        let mut force_channels = if texture.is_alpha() {
                SOIL_LOAD_RGBA
        } else {
                SOIL_LOAD_RGB
        };

        texture.set_alpha(alpha);

        unsafe {
           let mut image : *mut c_uchar = SOIL_load_image(file,
                           &mut width,
                           &mut height,
                           null_mut(),
                            force_channels as i32);
            texture.generate(width as u32, height as u32, image as *const c_void);

            SOIL_free_image_data(image);

            texture
        }


    }
}
