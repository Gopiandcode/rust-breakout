extern crate gl;
extern crate libc;
use gl::types::{GLuint, GLint};
use std::collections::hash_map::HashMap;
use std::fs::File;
use super::shader::{Shader,string_to_glchar};
use super::texture::Texture;

#![allow(non_camel_case_types)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/SOIL_bindings.rs"));

pub struct ResourceManager {
    shaders: HashMap<String, Shader>,
    textures: HashMap<String, Texture>,
}


impl ResourceManager {
    pub fn load_shader(&mut self, vertex_file: String, fragment_file: String, name: String) -> &Shader {
        self.shaders[name] = ResourceManager::load_shader_from_file(vertex_file, fragment_file);
        return &self.shaders[name];
    }

    pub fn get_shader(&mut self, name: String) -> &Shader {
        return &self.shaders[name];
    }

    pub fn load_texture() -> Texture {

    }

    pub fn get_texture(name: String) -> Texture {

    }

    pub fn clear() {

    }


    pub fn load_shader_from_file(vertex_file: String, fragment_file: String) -> Shader {
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

    pub fn load_texture_from_file(file: String, alpha: bool) -> Texture {
        let mut texture = Texture::new();

        texture.set_alpha(alpha);
           pub fn SOIL_load_image(file,
                           width: *mut ::std::os::raw::c_int,
                           height: *mut ::std::os::raw::c_int,
                           channels: *mut ::std::os::raw::c_int,
                           force_channels: ::std::os::raw::c_int)



    }
}
