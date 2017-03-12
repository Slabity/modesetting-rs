extern crate bindgen;

use std::env::var;
use std::path::PathBuf;
use bindgen::Builder;

// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let bindings = Builder::default()
        .no_unstable_rust()
        .header("src/ffi/c/ioctls.h")
        .clang_arg("-I/usr/include/drm")
        .link("drm")
        .hide_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");
/*
    let mut builder = libbindgen::Builder::default();
    builder = builder.clang_arg("-I/usr/include/drm");
    builder = builder.header("src/ffi/c/ioctls.h");
    builder = builder.link("drm");

    builder = builder.hide_type("drm_wait_vblank");
    builder = builder.hide_type("drm_wait_vblank_t");

    let (_, out_dir) = vars().find(| &(ref n, _) | n == "OUT_DIR").unwrap();
    let binding_path = out_dir + "/ffi.rs";*/

    let out_path = PathBuf::from(var("OUT_DIR").unwrap()).join("ffi.rs");

    bindings.write_to_file(out_path).expect("Could not write bindings");
    /*match builder.generate() {
        Ok(b) => b.write_to_file(out_path).unwrap(),
        Err(e) => panic!(e)
    };*/
}

pub fn main() {
    generate_shim_bindings();
}
