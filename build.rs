extern crate gcc;
extern crate bindgen;

// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let mut builder = bindgen::Builder::default();
    builder = builder.header("src/ffi/c/ioctls.h");
    builder = builder.link("drm");

    builder = builder.hide_type("drm_wait_vblank");
    builder = builder.hide_type("drm_wait_vblank_t");

    match builder.generate() {
        Ok(b) => b.write_to_file(concat!(env!("OUT_DIR"), "/ffi.rs")).unwrap(),
        Err(e) => panic!(e)
    };
}

pub fn main() {
    generate_shim_bindings();
}
