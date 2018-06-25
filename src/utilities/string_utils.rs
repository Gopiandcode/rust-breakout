extern crate gl;
extern crate nalgebra;

use std::collections::HashMap;
use std::ffi::CString;

use gl::types::GLchar;

static mut glchar_cache: Option<HashMap<String, CString>> = None;

pub fn allocate_cstring_buffer(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn str_to_glchar(string: &str) -> *const GLchar {
    unsafe {
        if let Some(ref mut map) = glchar_cache {
            if let Some(ref converted) = map.get(string) {
                return converted.as_ptr() as *const GLchar;
            }

            let name = CString::new(string)
                .expect("| ERROR::TEXT: Non-convertible string used in codebase");

            map.insert(string.to_string(), name);

            if let Some(ref converted) = map.get(string) {
                converted.as_ptr() as *const GLchar
            } else {
                str_to_glchar(string)
            }
        } else {
            glchar_cache = Some(HashMap::new());
            str_to_glchar(string)
        }
    }
}
