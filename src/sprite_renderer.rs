extern crate gl;
extern crate nalgebra;

use super::shader::{ Shader};
use super::texture::Texture;

use std::cell::RefCell;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::rc::Rc;

use nalgebra::base::{Matrix4, Unit, Vector2, Vector3};
use nalgebra::geometry::{Affine3, Rotation3, Similarity3, Transform3, Translation3};

use gl::types::{GLfloat, GLint, GLsizeiptr, GLuint};

pub unsafe fn buffer_data<T>(vbo: GLuint, vertices: &[T]) {
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * size_of::<T>()) as GLsizeiptr,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
}

pub struct SpriteRenderer {
    size: Vector2<f32>,
    rotation: GLfloat,
    color: Vector3<f32>,
    shader: Rc<RefCell<Shader>>,
    quadVAO: GLuint,
}

impl SpriteRenderer {
    pub fn new(shader: Rc<RefCell<Shader>>) -> Self {
        let mut vertices: Vec<GLfloat> = vec![
            // pos          // tex
            0.0, 1.0, 0.0, 1.0, 
            1.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 1.0, 
            1.0, 1.0, 1.0, 1.0, 
            1.0, 0.0, 1.0, 0.0, ]; 

        SpriteRenderer::new_with_custom_quad(shader, &vertices)
    }
    pub fn new_with_custom_quad(shader: Rc<RefCell<Shader>>, quad: &[GLfloat]) -> Self {
        let mut vbo: GLuint = 0;
        let mut vao: GLuint = 0;

        // allocate space
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        // load over data onto GPU
        unsafe {
            buffer_data(vbo, quad);
        }

        // bind to the VAO for reuse
        unsafe {

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,         // index
                4,         // size
                gl::FLOAT, // type
                gl::FALSE, // normalized
                (4 * size_of::<GLfloat>()) as GLint,
                // stride
                null(), // pointer
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

        }

        SpriteRenderer {
            size: Vector2::new(10.0, 10.0),
            rotation: 0.0,
            color: Vector3::new(1.0, 1.0, 1.0),
            shader: shader.clone(),
            quadVAO: 0,
        }
    }

    pub fn draw_sprite(&mut self, texture: &Texture, position: &Vector2<GLfloat>){
        self.draw_sprite_transformed_internal(texture, position, None, None, None);
    }

    pub fn draw_sprite_transformed(
        &mut self,
        texture: &Texture,
        position: &Vector2<GLfloat>,
        size: &Vector2<GLfloat>,
        rotate: GLfloat,
        color: &Vector3<GLfloat>
    ) {
        self.draw_sprite_transformed_internal(texture, position, Some(size), Some(rotate), Some(color));
    }

    fn draw_sprite_transformed_internal(
        &mut self,
        texture: &Texture,
        position: &Vector2<GLfloat>,
        size: Option<&Vector2<GLfloat>>,
        rotate: Option<GLfloat>,
        color: Option<&Vector3<GLfloat>>,
    ) {
        let rotate = rotate.unwrap_or(self.rotation);
        let color = color.unwrap_or(&self.color);
        let size = size.unwrap_or(&self.size);


        unsafe {
            self.shader.borrow().enable();
        }

        // construct the positionining matrix for the texture

        let position_vector = Translation3::from_vector(Vector3::new(position.x, position.y, 0.0));
        let center_prime = Translation3::from_vector(Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0));
        let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), rotate);
        let center = Translation3::from_vector(Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0));
        let scaling_matrix = Transform3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(
            &Vector3::new(size.x, size.y, 1.0),
        ));

        let mut model =
            // move to correct location on size
            position_vector *

            // rotate 
            center_prime *
                rotation *
                center *
            // scale quad to correct size 
                scaling_matrix;

        let mut model = model.matrix();

        unsafe {
            // load the position matrix and color vector into the shader
            self.shader.borrow_mut().setMatrix4("model", &model);
            self.shader.borrow_mut().setVector3f("spriteColor", &color);

            // this function will be using texture_0 entry for storing textures
            gl::ActiveTexture(gl::TEXTURE0);

            // draw the sprite using the specified texture
            texture.bind();
            gl::BindVertexArray(self.quadVAO);
            gl::EnableVertexAttribArray(0);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindVertexArray(0);
        }
    }
}
