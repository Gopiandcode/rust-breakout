extern crate gl;
use gl::types::{GLint, GLuint};
use std::os::raw::c_void;

/// Struct used to represent texture
pub struct Texture {
    id: GLuint,              // id of texture object
    width: GLuint,           // width of loaded texture
    height: GLuint,          // height of loaded texture
    internal_format: GLuint, // format of texture object
    image_format: GLuint,    // format of loaded image
    wrap_S: GLuint,          // wrapping on S axis
    wrap_T: GLuint,          // wrapping on T axis
    filter_min: GLuint,      // filtering mode if pixels < screen pixels
    filter_max: GLuint,      // filtering mode if pixels > screen pixels
}

pub struct TextureBuilder {
    width: GLuint,
    height: GLuint,
    data: *const c_void,
    internal_format: Option<GLuint>, // format of texture object
    image_format: Option<GLuint>,    // format of loaded image
    wrap_S: Option<GLuint>,          // wrapping on S axis
    wrap_T: Option<GLuint>,          // wrapping on T axis
    filter_min: Option<GLuint>,      // filtering mode if pixels < screen pixels
    filter_max: Option<GLuint>,      // filtering mode if pixels > screen pixels
}

impl TextureBuilder {
    fn new(width: GLuint, height: GLuint, data: *const c_void) -> Self {
        TextureBuilder {
            width: width,
            height: height,
            data: data,
            internal_format: None,
            image_format: None,
            wrap_S: None,
            wrap_T: None,
            filter_min: None,
            filter_max: None,
        }
    }

    pub fn with_internal_format(&mut self, internal_format: GLuint) -> &mut Self {
        self.internal_format = Some(internal_format);
        self
    }

    pub fn with_image_format(&mut self, image_format: GLuint) -> &mut Self {
        self.image_format = Some(image_format);
        self
    }

    pub fn with_wrap_S(&mut self, wrap_S: GLuint) -> &mut Self {
        self.wrap_S = Some(wrap_S);
        self
    }

    pub fn with_wrap_T(&mut self, wrap_T: GLuint) -> &mut Self {
        self.wrap_T = Some(wrap_T);
        self
    }

    pub fn with_filter_min(&mut self, filter_min: GLuint) -> &mut Self {
        self.filter_min = Some(filter_min);
        self
    }

    pub fn with_filter_max(&mut self, filter_max: GLuint) -> &mut Self {
        self.filter_max = Some(filter_max);
        self
    }

    pub fn with_alpha(&mut self, alpha: bool) -> &mut Self {
        if alpha {
            self.internal_format = Some(gl::RGBA);
            self.image_format = Some(gl::RGBA);
        } else {
            self.internal_format = Some(gl::RGB);
            self.image_format = Some(gl::RGB);
        }
        self
    }

    pub fn build(&self) -> Result<Texture, String> {
        let internal_format = self.internal_format.unwrap_or(gl::RGB);
        let image_format = self.image_format.unwrap_or(gl::RGB);
        let wrap_S = self.wrap_S.unwrap_or(gl::REPEAT);
        let wrap_T = self.wrap_T.unwrap_or(gl::REPEAT);
        let filter_min = self.filter_min.unwrap_or(gl::LINEAR);
        let filter_max = self.filter_max.unwrap_or(gl::LINEAR);

        Texture::construct(
            self.width,
            self.height,
            self.data,
            internal_format,
            image_format,
            wrap_S,
            wrap_T,
            filter_min,
            filter_max,
        )
    }
}

impl Texture {
    pub fn new(width: GLuint, height: GLuint, data: *const c_void) -> TextureBuilder {
        TextureBuilder::new(width, height, data)
    }
    fn construct(
        width: GLuint,
        height: GLuint,
        data: *const c_void,
        internal_format: GLuint,
        image_format: GLuint,
        wrap_S: GLuint,
        wrap_T: GLuint,
        filter_min: GLuint,
        filter_max: GLuint,
    ) -> Result<Self, String> {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            internal_format as GLint,
            width as GLint,
            height as GLint,
            0,
            image_format,
            gl::UNSIGNED_BYTE,
            data,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_S as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_T as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_max as GLint);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        Ok(Texture {
            id: id,
            width: width,
            height: height,
            internal_format: internal_format,
            image_format: image_format,
            wrap_S: wrap_S,
            wrap_T: wrap_T,
            filter_min: filter_min,
            filter_max: filter_max,
        })
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
