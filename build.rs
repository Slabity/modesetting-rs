
use std::path::PathBuf;
use std::env;

// Compile and link to the drm-shim
fn compile_drm_shim() {
    cc::Build::new()
        .file("src/ffi/cc/drm_shim.c")
        .debug(false)
        .compile("libffi.a");
}

// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let bindings = bindgen::Builder::default()
        .header("src/ffi/cc/drm_shim.c")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

//    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//    bindings
//        .write_to_file(out_path.join("drm_shim.rs"))
//        .expect("Couldn't write bindings!");
    bindings
        .write_to_file("src/ffi/drm_shim.rs")
        .expect("Couldn't write bindings!");
}

pub fn main() {
    compile_drm_shim();
    generate_shim_bindings();
}
