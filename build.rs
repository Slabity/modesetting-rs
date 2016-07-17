extern crate gcc;
extern crate bindgen;

// Compile and link to the drm-shim
fn compile_drm_shim() {
    gcc::Config::new()
        .file("src/ffi/cc/drm_shim.c")
        .debug(false)
        .compile("libffi.a");
}

// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let mut builder = bindgen::Builder::new();
    builder.header("src/ffi/cc/drm_shim.c");
    builder.remove_prefix("");
    builder.link("kms", bindgen::LinkType::Dynamic);
    match builder.generate() {
        Ok(b) => b.write_to_file("src/ffi/drm_shim.rs").unwrap(),
        Err(e) => panic!(e)
    };
}

pub fn main() {
    compile_drm_shim();
    generate_shim_bindings();
}
