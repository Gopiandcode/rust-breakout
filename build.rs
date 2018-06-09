extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("{}", &out_dir);
    println!("cargo:rustc-link-search=native=lib/SOIL");
    println!("cargo:rustc-link-lib=static=SOIL");


    let bindings = bindgen::Builder::default()
        .header("lib/SOIL/SOIL.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir.clone());
    bindings.write_to_file(out_path.join("SOIL_bindings.rs"))
        .expect("Couldn't write bindings!");

    panic!("Anay");


}