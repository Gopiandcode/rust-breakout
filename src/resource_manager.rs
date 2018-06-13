extern crate gl;

use gl::types::{GLchar, GLint, GLuint};

use std::collections::hash_map::HashMap;

use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::mem::swap;
use std::slice::from_raw_parts;

use std::cell::RefCell;
use std::rc::Rc;

use std::os::raw::{c_char, c_int, c_uchar, c_void};
use std::ptr::null_mut;

use super::shader::{Shader};
use super::texture::Texture;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/SOIL_bindings.rs"));

pub struct ResourceManager {
    shaders: HashMap<String, Rc<RefCell<Shader>>>,
    textures: HashMap<String, Rc<RefCell<Texture>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        vertex_file: &str,
        fragment_file: &str,
        name: &str,
    ) -> Result<Rc<RefCell<Shader>>,String> {
        let shader = ResourceManager::load_shader_from_file( vertex_file, fragment_file )?;
        self.shaders.insert(
            name.to_string(),
            Rc::new(RefCell::new(shader))
        );
        Ok(self.shaders[name].clone())
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<RefCell<Shader>>> {
        self.shaders.get(name).map(|shdr| shdr.clone())
    }

    pub fn load_texture(
        &mut self,
        vertex_file: &str,
        is_alpha: bool,
        name: &str,
    ) -> Result<Rc<RefCell<Texture>>,String> {
        let texture = ResourceManager::load_texture_from_file( vertex_file, is_alpha)?;
        self.textures.insert(
            name.to_string(),
            Rc::new(RefCell::new(texture)),
        );
        Ok(self.textures[name].clone())
    }

    pub fn get_texture(&self, name: &str) -> Option<Rc<RefCell<Texture>>> {
        self.textures.get(name).map(|shdr| shdr.clone())
    }

    pub fn clear(&mut self) {
        {
            let mut new_shaders = HashMap::new();
            let mut new_textures = HashMap::new();

            swap(&mut new_shaders, &mut self.shaders);
            swap(&mut new_textures, &mut self.textures);
        }
    }

    fn load_shader_from_file(vertex_file: &str, fragment_file: &str) -> Result<Shader, String> {
        let mut vertex_string = String::new();
        let mut fragment_string = String::new();

        {
            let mut vertex = try!(File::open(vertex_file).map_err(|e| e.to_string()));
            let mut fragment = try!(File::open(fragment_file).map_err(|e| e.to_string()));
            vertex
                .read_to_string(&mut vertex_string)
                .map_err(|e| e.to_string())?;
            fragment
                .read_to_string(&mut fragment_string)
                .map_err(|e| e.to_string())?;
        }

        let mut vertex_source =
            CString::new(vertex_string.into_bytes()).map_err(|e| e.to_string())?;
        let mut fragment_source =
            CString::new(fragment_string.into_bytes()).map_err(|e| e.to_string())?;

        Shader::new(&vertex_source, &fragment_source)
    }

    fn load_texture_from_file(path: &str, alpha: bool) -> Result<Texture,String> {

        let mut file = CString::new(path)
            .expect("| ERROR::RESOURCE_MANAGER: Invalid file string for texture.");
        let mut file = file.as_ptr();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut force_channels = SOIL_LOAD_AUTO;


        unsafe {
            let mut channels = 0;
            let mut image: *mut c_uchar = SOIL_load_image(
                file,
                &mut width,
                &mut height,
                &mut channels,
                force_channels as i32,
            );
            if image.is_null() {
                return Err( format!( "Unable to load image at location {}", path));
            }

            let mut texture = Texture::new(width as GLuint, height as GLuint, image as *const c_void).build();

            SOIL_free_image_data(image);

            texture
        }
    }
}
