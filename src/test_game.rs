extern crate gl;

use super::resource_manager::ResourceManager;
use super::shader::Shader;
use gl::types::{GLfloat, GLint, GLuint};
use std::cell::RefCell;
use std::mem::size_of;
use std::ptr::null;
use std::rc::Rc;

pub struct TestGame {
    resource_manager: Rc<RefCell<ResourceManager>>,
    vao: GLuint,
}

impl TestGame {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>) -> Self {
        TestGame {
            resource_manager: resource_manager.clone(),
            vao: 0,
        }
    }

    pub fn init(&mut self) {
        let mut shader = self
            .resource_manager
            .borrow_mut()
            .load_shader(
                "/home/gopiandcode/Documents/Rust/gui-base/shaders/triangle.vs",
                "/home/gopiandcode/Documents/Rust/gui-base/shaders/triangle.frag",
                "triangle",
            )
            .expect("Could not load triangle shader");

        unsafe {
            {
                let mut _shader = shader.borrow_mut();
                _shader.enable();
            }
        }

        let vertices: Vec<GLfloat> = vec![
            0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        ];

        let mut vbo = 0;
        let mut vao = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            self.vao = vao;
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * size_of::<GLfloat>()) as gl::types::GLint,
                null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn processInput(&mut self, dt: f32) {}
    pub fn update(&mut self, dt: f32) {}
    pub fn render(&mut self) {
        if let Some(mut shader) = self.resource_manager.borrow().get_shader("triangle") {
            unsafe {
                let mut _shader = shader.borrow_mut();
                _shader.enable();
            }
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
    }
}
