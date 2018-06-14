extern crate gl;
extern crate nalgebra;

use super::string_utils::{allocate_cstring_buffer, str_to_glchar};

use std::ffi::CStr;
use std::ptr;
use std::ptr::null;

use gl::types::{GLfloat, GLint, GLsizei, GLuint};
use nalgebra::base::{Matrix4, Vector2, Vector3};

pub struct Shader {
    id: GLuint,
    use_shader: bool,
}

unsafe fn check_program_compile_errors(object: GLuint) -> Option<String> {
    let mut success: GLint = 1;
    gl::GetProgramiv(object, gl::LINK_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        gl::GetProgramiv(object, gl::INFO_LOG_LENGTH, &mut len);

        let mut error = allocate_cstring_buffer(len as usize);

        gl::GetProgramInfoLog(
            object,
            len,
            ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );

        Some(error.to_string_lossy().into_owned())
    } else {
        None
    }
}

unsafe fn check_shader_compile_errors(object: GLuint) -> Option<String> {
    let mut success: GLint = 1;

    gl::GetShaderiv(object, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(object, gl::INFO_LOG_LENGTH, &mut len);

        let mut error = allocate_cstring_buffer(len as usize);

        gl::GetShaderInfoLog(
            object,
            len,
            ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );

        Some(error.to_string_lossy().into_owned())
    } else {
        None
    }
}

fn compile_shader(source: &CStr, shader_type: GLuint) -> Result<GLuint, String> {
    unsafe {
        let id = gl::CreateShader(shader_type);
        gl::ShaderSource(id, 1, &source.as_ptr(), null());
        gl::CompileShader(id);
        if let Some(err) = check_shader_compile_errors(id) {
            Err(err)
        } else {
            Ok(id)
        }
    }
}

fn compile_program(vertex_id: GLuint, fragment_id: GLuint) -> Result<GLuint, String> {
    unsafe {
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex_id);
        gl::AttachShader(id, fragment_id);

        gl::LinkProgram(id);

        let result = if let Some(err) = check_program_compile_errors(id) {
            Err(err)
        } else {
            Ok(id)
        };

        gl::DetachShader(id, vertex_id);
        gl::DetachShader(id, fragment_id);

        result
    }
}

impl Shader {
    /// Constructs a new Shader Program given a vertex source file, and a fragment source file.
    ///
    /// OpenGL compiles the shaders at runtime, so if there are any errors during compilation,
    /// the result will contain the error string.
    pub fn new(vertexSource: &CStr, fragmentSource: &CStr) -> Result<Self, String> {
        let mut id = 0;

        let sVertex = try!(compile_shader(vertexSource, gl::VERTEX_SHADER));
        let sFragment = try!(compile_shader(fragmentSource, gl::FRAGMENT_SHADER));

        id = try!(compile_program(sVertex, sFragment));

        // cleanup
        unsafe {
            gl::DeleteShader(sVertex);
            gl::DeleteShader(sFragment);
        }

        Ok(Shader {
            id: id,
            use_shader: false,
        })
    }

    pub fn set_use_shader(&mut self, use_shader: bool) {
        self.use_shader = use_shader;
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub unsafe fn setFloat(&mut self, name: &str, value: GLfloat) {
        if self.use_shader {
            self.enable();
        }

        let name = str_to_glchar(name);

        gl::Uniform1f(gl::GetUniformLocation(self.id, name), value);
    }

    pub unsafe fn setInt(&mut self, name: &str, value: GLint) {
        if self.use_shader {
            self.enable();
        }
        let name = str_to_glchar(name);

        gl::Uniform1i(gl::GetUniformLocation(self.id, name), value);
    }

    pub unsafe fn setVector2f(&mut self, name: &str, value: &Vector2<GLfloat>) {
        if self.use_shader {
            self.enable();
        }

        let name = str_to_glchar(name);

        gl::Uniform2fv(
            gl::GetUniformLocation(self.id, name),
            1,
            value.as_slice().as_ptr(),
        );
    }

    pub unsafe fn setVector3f(&mut self, name: &str, value: &Vector3<GLfloat>) {
        if self.use_shader {
            self.enable();
        }

        let name = str_to_glchar(name);

        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name),
            1,
            value.as_slice().as_ptr(),
        );
    }

    pub unsafe fn setMatrix4(&mut self, name: &str, value: &Matrix4<GLfloat>) {
        if self.use_shader {
            self.enable();
        }
        let name = str_to_glchar(name);

        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name),
            1,
            gl::FALSE,
            value.as_slice().as_ptr(),
        );
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
