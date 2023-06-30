extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib/");
    println!("cargo:rustc-link-lib=qatzip");

    let bindings = bindgen::Builder::default()
        .header("/usr/local/include/qatzip.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
