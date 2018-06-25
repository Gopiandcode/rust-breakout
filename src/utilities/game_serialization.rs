extern crate gl;
extern crate nalgebra;

use utilities::sprite_renderer::SpriteRenderer;
use objects::game_object::{GameObject,GameObjectBuilder};
use systems::resource_manager::ResourceManager;

use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::{Cursor, Read, BufRead, BufReader};

use gl::types::{GLuint, GLfloat};
use nalgebra::base::{Matrix4, Vector2, Vector3};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_correct_format() {
        read_from_string("15:30\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4,\n 4, 1, 4, 1, 4, 0, 0, 1, 0, 0, 4, 1, 4, 1, 4,\n 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3,\n 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3,\n 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,\n").expect("Should work");
    }

    #[test]
    #[should_panic]
    fn rejects_incorrect_column_widths() {
        read_from_string("15:30\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4,\n 4, 1, 4, 1, 4, 0, 0, 1, 0, 0, 4, 1, 4, 1, 4,\n 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3,\n 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3,\n 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,\n").expect("Should work");
    }

    #[test]
    #[should_panic]
    fn rejects_incorrectly_formatted_headers() {
        read_from_string("1530\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,\n 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4,\n 4, 1, 4, 1, 4, 0, 0, 1, 0, 0, 4, 1, 4, 1, 4,\n 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3,\n 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3,\n 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,\n").expect("Should work");
    }
}


pub fn read_from_file<T : AsRef<Path>>(filename: &T) -> Result<(Vec<u32>, (GLuint, GLuint)), String> {
    let mut file = File::open(filename).map_err(|e| e.to_string())?;
    read_from_reader(&mut BufReader::new(file))

}

pub fn read_from_string(string : &str) -> Result<(Vec<u32>, (GLuint, GLuint)), String>  {
    read_from_reader(&mut BufReader::new(string.as_bytes()))
}


pub fn read_from_reader<R : BufRead>(reader : &mut R) -> Result<(Vec<u32>, (GLuint, GLuint)), String> {
    let mut line = String::new();
    let mut height = 0;
    let mut width = 0;

    reader.read_line(&mut line).map_err(|e| e.to_string())?;

    let dim = line.split(":")
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<GLuint>().ok())
        .collect::<Vec<GLuint>>();


    line.clear();

    if dim.len() != 2 {
        return Err(format!("Invalid file format - incorrect header structure - {:?}", dim));
    }

    width = *dim.get(0).ok_or("Invalid file format - incorrect header format")?;
    height = *dim.get(1).ok_or("Invalid file format - incorrect header format")?;

    let mut level = Vec::with_capacity((height * width) as usize);

    let mut i = 0;
    while i < height {
        if let Ok(len) = reader.read_line(&mut line) {
            if len == 0 {
                break;
            }
            let count = line.split(",")
                .map(|s| s.trim())
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            if count.len() != width as usize {
                println!("Line is {}", line);
                println!("Incorrect as {} /= {}", count.len(), width);
                return Err(format!("Invalid file format - at line {}", i))
            }

            for entry in count {
                level.push(entry);
            }

            line.clear();
        } else {
            break;
       }

        i = i + 1;
    }

    while i < height {
        for j in 0..width {
                level.push(0);
        }
        i = i + 1;
    }

    Ok((level,(width, height)))
}

