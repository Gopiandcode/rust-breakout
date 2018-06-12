extern crate gl;
extern crate nalgebra;

use std::collections::HashMap;
use std::ffi::{CString,CStr};
use std::ptr;
use std::ptr::null;

use nalgebra::base::{Matrix3, Matrix4, Vector2, Vector3};
use sdl2::event::Event;
use gl::types::{GLuint, GLint, GLfloat, GLboolean, GLchar, GLsizei};


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
                          vertexSource: &CStr,
                          fragmentSource: &CStr) {

        let sVertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(sVertex, 1, &vertexSource.as_ptr(), null());
        gl::CompileShader(sVertex);

        println!("Checking for vertex shader compiling errors");
        self.checkCompileErrors(sVertex, "VERTEX");
        println!("Finished checking for vertex shader errors");

        let sFragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(sFragment, 1, &fragmentSource.as_ptr(), null());
        gl::CompileShader(sFragment);

        println!("Checking for fragment shader compiling errors");
        self.checkCompileErrors(sFragment, "FRAGMENT");
        println!("Finished checking for fragment shader errors");


        self.id = gl::CreateProgram();
        gl::AttachShader(self.id, sVertex);
        gl::AttachShader(self.id, sFragment);

        gl::LinkProgram(self.id);
        let id = self.id;
        self.checkCompileErrors(id, "PROGRAM");

        gl::DetachShader(self.id, sVertex);
        gl::DetachShader(self.id, sFragment);

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
        let mut success: GLint = 1;

        if (object_t != "PROGRAM") {

            gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(object, gl::INFO_LOG_LENGTH, &mut len);

                let mut error = allocate_cstring_buffer(len as usize);


                gl::GetShaderInfoLog(object, len, ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);


                let string = error.to_string_lossy().into_owned();
                println!("| ERROR::SHADER: Link-time error: Type: {} \n{}\n -- ---------------------------------------------------- -- ", object_t, string);
            }
        } else {
            gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(object, gl::INFO_LOG_LENGTH, &mut len);

                let mut error = allocate_cstring_buffer(len as usize);


                gl::GetProgramInfoLog(object, len, ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);


                let string = error.to_string_lossy().into_owned();
                println!("| ERROR::SHADER: Link-time error: Type: {} \n{}\n -- ---------------------------------------------------- -- ", object_t, string);
            }
        }
    }
}

fn allocate_cstring_buffer(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}