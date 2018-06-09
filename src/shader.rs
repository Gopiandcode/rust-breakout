extern crate gl;
use sdl2::event::Event;
use std::ptr;
use gl::types::{GLuint, GLint, GLfloat, GLboolean, GLchar, GLsizei};

pub struct Shader {
    id : GLuint,
    useShader: bool
}

pub fn glchar_to_string(bytes: &[GLchar]) -> String {
    let mut vector : Vec<u8> = Vec::new();

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
            useShader: false
        }
    }

    pub fn setUseShader(&mut self, useShader: bool) {
       self.useShader = useShader;
    }

    pub fn setActive(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    unsafe fn useShader(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn compile(&mut self,
                   vertexSource : &[GLchar],
                   fragmentSource : &[GLchar]) {

        let sVertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(sVertex, 1 as GLsizei, &vertexSource.as_ptr(), &1);
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

    pub unsafe fn setFloat(&mut self, name : &[GLchar], value : GLfloat) {
        if self.useShader {
                self.useShader();
        }

        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn setInt(&mut self, name : &[GLchar], value : GLint) {
        if self.useShader {
            self.useShader();
        }

        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn setVector2f(&mut self, name : &[GLchar], value : &[GLfloat; 2]) {
        if self.useShader {
            self.useShader();
        }

        gl::Uniform2fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }

    pub unsafe fn setVector3f(&mut self, name : &[GLchar], value : &[GLfloat; 3]) {
        if self.useShader {
            self.useShader();
        }

        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }

    pub unsafe fn setMatrix4(&mut self, name : &[GLchar], value : &[GLfloat; 16]) {
        if self.useShader {
            self.useShader();
        }

        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, value.as_ptr());
    }


    unsafe fn checkCompileErrors(&mut self, object: GLuint, object_t: &str) {
        let mut success : GLint = 0;
        let mut infoLog : [GLchar; 1024] = [0; 1024];

        if(object_t != "PROGRAM") {
            gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);
            if success != 0 {
                gl::GetShaderInfoLog(object, 1024, ptr::null_mut(), infoLog.as_mut_ptr());
                let string = glchar_to_string(&infoLog);
                println!("| ERROR::SHADER: Compile-time error: Type: {} \n{}\n -- ---------------------------------------------------- -- ", object_t, string);
            }
        } else {
            gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);

            if success != 0 {
                gl::GetProgramInfoLog(object, 1024, ptr::null_mut(), infoLog.as_mut_ptr());
            }
            let string = glchar_to_string(&infoLog);
            println!("| ERROR::SHADER: Link-time error: Type: {} \n{}\n -- ---------------------------------------------------- -- ", object_t, string);
        }

    }
}

