extern crate gl;
use gl::types::{GLuint, GLint};
use std::os::raw::c_void;


/// Struct used to represent texture
pub struct Texture {
    id : GLuint,              // id of texture object
    width: GLuint,            // width of loaded texture
    height: GLuint,           // height of loaded texture
    internal_format: GLuint,  // format of texture object
    image_format: GLuint,     // format of loaded image
    wrap_S: GLuint,           // wrapping on S axis
    wrap_T: GLuint,           // wrapping on T axis
    filter_min: GLuint,       // filtering mode if pixels < screen pixels
    filter_max: GLuint,       // filtering mode if pixels > screen pixels
}


impl Texture {

    pub fn new() -> Self {
            let mut id = 0;
            unsafe {
            gl::GenTextures(1 ,&mut id);
        }

        Texture{
            id: id,
            width: 0,
            height: 0,
            internal_format: gl::RGB,
            image_format: gl::RGB,
            wrap_S: gl::REPEAT,
            wrap_T: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR
        }
    }

    pub fn set_alpha(&mut self, alpha: bool) {
        if alpha {
            self.internal_format = gl::RGBA;
            self.image_format = gl::RGBA;
        } else {
            self.internal_format = gl::RGB;
            self.image_format =  gl::RGB;
        }
    }

    pub fn is_alpha(&self) -> bool {
        self.image_format == gl::RGBA
    }

    pub unsafe fn generate(&mut self, width: GLuint, height: GLuint, data: *const c_void) {

        self.width = width;
        self.height = height;
        gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       self.internal_format as GLint,
                       width as GLint, height as GLint,
                       0,
                       self.image_format,
                       gl::UNSIGNED_BYTE,
                       data);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_S as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_T as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_max as GLint);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }


    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn delete(&mut self) {
        gl::DeleteTextures(1, &self.id);
        self.id = 0;
        self.width = 0;
        self.height = 0;
    }
}

