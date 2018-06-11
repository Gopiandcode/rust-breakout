extern crate gl;
extern crate nalgebra;

use std::ffi::CString;
use std::ptr;

use nalgebra::base::{Matrix3, Matrix4, Vector2, Vector3};
use sdl2::event::Event;
use gl::types::{GLuint, GLint, GLfloat, GLboolean, GLchar, GLsizei};
use std::collections::HashMap;

static mut glchar_cache: Option<HashMap<String, Vec<GLchar>>> = None;


fn str_to_glchar(string: &str) -> *const GLchar {

    unsafe {
        if let Some(ref mut map) = glchar_cache {
            if let Some(ref converted) = map.get(string) {
                return converted.as_ptr();
            }

            let name: Vec<GLchar> = string_to_glchar(CString::new(string).unwrap().to_bytes());

            map.insert(string.to_string(), name);

            if let Some(ref converted) = map.get(string) {
                converted.as_ptr()
            } else {
                str_to_glchar(string)
            }
        } else {
            glchar_cache = Some(HashMap::new());
            str_to_glchar(string)
        }
    }
}

pub struct Shader {
    id: GLuint,
    use_shader: bool,
}

pub fn glchar_to_string(bytes: &[GLchar]) -> String {
    let mut vector: Vec<u8> = Vec::new();

    for &i in bytes.iter() {
        vector.push(i as u8);
    }

    String::from_utf8(vector).unwrap()
}

pub fn string_to_glchar(bytes: &[u8]) -> Vec<GLchar> {

    let mut vector: Vec<GLchar> = Vec::new();

    for &i in bytes.iter() {
        vector.push(i as GLchar);
    }

    vector
}

impl Shader {
    pub fn new() -> Self {
        Shader {
            id: 0,
            use_shader: false,
        }
    }

    pub fn setUseShader(&mut self, useShader: bool) {
        self.use_shader = useShader;
    }

    pub fn setActive(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub unsafe fn delete(&mut self) {
        gl::DeleteProgram(self.id);
        self.id = 0;
        self.use_shader = false;
    }

    pub unsafe fn useShader(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn compile(&mut self,
                          vertexSource: &[GLchar],
                          fragmentSource: &[GLchar]) {
        let sVertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(sVertex, 1 as GLsizei, &(vertexSource.as_ptr()), &1);
        gl::CompileShader(sVertex);
        self.checkCompileErrors(sVertex, "VERTEX");

        let sFragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(sFragment, 1 as GLsizei, &fragmentSource.as_ptr(), &1);
        gl::CompileShader(sFragment);
        self.checkCompileErrors(sFragment, "FRAGMENT");


        self.id = gl::CreateProgram();
        gl::AttachShader(self.id, sVertex);
        gl::AttachShader(self.id, sFragment);

        gl::LinkProgram(self.id);
        let id = self.id;
        self.checkCompileErrors(id, "PROGRAM");

        gl::DeleteShader(sVertex);
        gl::DeleteShader(sFragment);
    }

    pub unsafe fn setFloat(&mut self, name: &str, value: GLfloat) {
        if self.use_shader {
            self.useShader();
        }

        let name = str_to_glchar(name);

        gl::Uniform1f(gl::GetUniformLocation(self.id, name), value);
    }

    pub unsafe fn setInt(&mut self, name: &str, value: GLint) {
        if self.use_shader {
            self.useShader();
        }
        let name = str_to_glchar(name);

        gl::Uniform1i(gl::GetUniformLocation(self.id, name), value);
    }

    pub unsafe fn setVector2f(&mut self, name: &str, value: &Vector2<GLfloat>) {
        if self.use_shader {
            self.useShader();
        }

        let name = str_to_glchar(name);

        gl::Uniform2fv(gl::GetUniformLocation(self.id, name), 1, value.as_slice().as_ptr());
    }

    pub unsafe fn setVector3f(&mut self, name: &str, value: &Vector3<GLfloat>) {
        if self.use_shader {
            self.useShader();
        }

        let name = str_to_glchar(name);

        gl::Uniform3fv(gl::GetUniformLocation(self.id, name), 1, value.as_slice().as_ptr());
    }

    pub unsafe fn setMatrix4(&mut self, name: &str, value: &Matrix4<GLfloat>) {
        if self.use_shader {
            self.useShader();
        }
        let name = str_to_glchar(name);

        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name), 1, gl::FALSE, value.as_slice().as_ptr());
    }


    unsafe fn checkCompileErrors(&mut self, object: GLuint, object_t: &str) {
        let mut success: GLint = 0;
        let mut infoLog: [GLchar; 1024] = [0; 1024];

        if (object_t != "PROGRAM") {
            gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);
            if success != 0 {
                gl::GetShaderInfoLog(object, 1024, ptr::null_mut(), infoLog.as_mut_ptr());
                let string = glchar_to_string(&infoLog);
            }
        } else {
            gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);

            if success != 0 {
                let mut length = 0;
                gl::GetProgramInfoLog(object, 1024, &mut length, infoLog.as_mut_ptr());
                let string = glchar_to_string(&infoLog[0 .. length as usize]);
                println!("| ERROR::SHADER: Link-time error: Type: {} \n{}\n -- ---------------------------------------------------- -- ", object_t, string);
            }
        }
    }
}

