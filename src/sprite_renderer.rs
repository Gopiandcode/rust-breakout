extern crate gl;
extern crate nalgebra;

use super::texture::Texture;
use super::shader::{Shader, string_to_glchar};

use std::mem::size_of;
use std::rc::Rc;
use std::cell::RefCell;
use std::os::raw::c_void;
use std::ptr::null;
use std::ffi::CString;

use nalgebra::base::{Vector2,Vector3, Matrix4, Unit};
use nalgebra::geometry::{Transform3, Affine3, Rotation3, Translation3, Similarity3};

use gl::types::{GLuint, GLfloat, GLint};

pub struct SpriteRenderer {
    size: Vector2<f32>,
    rotation: GLfloat,
    color: Vector3<f32>,
    shader: Rc<RefCell<Shader>>,
    quadVAO: GLuint
}


impl SpriteRenderer {
    pub fn new(shader: Rc<RefCell<Shader>>) -> Self {
        SpriteRenderer {
            size: Vector2::new(10.0, 10.0),
            rotation: 0.0,
            color: Vector3::new(1.0, 1.0, 1.0),
            shader: shader.clone(),
            quadVAO: 0
        }
    }


    fn init_render_data(&mut self) {
       let mut vbo : GLuint  = 0;
        let mut vertices : Vec<GLfloat> = vec![
            // pos          // tex
            0.0, 1.0,      0.0, 1.0,
            1.0, 0.0,      1.0, 0.0,
            0.0, 0.0,      0.0, 0.0,

            0.0, 1.0,      0.0, 1.0,
            1.0, 1.0,      1.0, 1.0,
            1.0, 0.0,      1.0, 0.0
        ];


        unsafe {
            gl::GenVertexArrays(1, &mut self.quadVAO);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertices.len() as isize,
                           vertices.as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
            gl::BindVertexArray(self.quadVAO);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,                       // index
                4,                       // size
                gl::FLOAT,               // type
                gl::FALSE,               // normalized
                (4 * size_of::<GLfloat>()) as GLint,
                                         // stride
                null()                   // pointer
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }


    pub fn draw_sprite(&mut self, texture: &Texture, position: &Vector2<GLfloat>, size: &Vector2<GLfloat>, rotate: GLfloat, color: &Vector3<GLfloat>) {
        unsafe {
            self.shader.borrow().useShader();
        }


        // construct the positionining matrix for the texture
        let mut model =
            // move to correct location on size
            Translation3::from_vector(Vector3::new(position.x, position.y, 0.0)) *

            // rotate 2d texture aroud z-axis by specified angle, around the center of the screen
            Translation3::from_vector(Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0)) *
            Rotation3::from_axis_angle(&Vector3::z_axis(), rotate) *
            Translation3::from_vector(Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0)) *

            // scale locations to x and y
            Transform3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(size.x, size.y, 1.0)));


        let mut model = model.matrix();

        let matrix_text = string_to_glchar(CString::new("model").unwrap().to_bytes());
        let color_text = string_to_glchar(CString::new("spriteColor").unwrap().to_bytes());

        unsafe {
            // load the position matrix and color vector into the shader
            self.shader.borrow_mut().setMatrix4(&matrix_text, &model);
            self.shader.borrow_mut().setVector3f(&color_text, &color);

            // this function will be using texture_0 entry for storing textures
            gl::ActiveTexture(gl::TEXTURE0);

            // draw the sprite using the specified texture
            texture.bind();
            gl::BindVertexArray(self.quadVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);


            gl::BindVertexArray(0);
        }

    }

}