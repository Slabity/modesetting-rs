#[cfg(feature = "use_bindgen")]
extern crate bindgen;
#[cfg(feature = "use_bindgen")]
use bindgen::Builder;
#[cfg(feature = "use_bindgen")]
use std::env::var;
#[cfg(feature = "use_bindgen")]
use std::path::PathBuf;

#[cfg(feature = "use_bindgen")]
// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let bindings = Builder::default()
        .no_unstable_rust()
        .header("src/c/wrapper.h")
        .clang_arg("-I/usr/include/drm")
        .link("drm")
        .ctypes_prefix("libc")
        .hide_type("max_align_t")
        .emit_builtins()
        .emit_clang_ast()
        .emit_ir()
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings.write_to_file(out_path).expect("Could not write bindings");
}

#[cfg(feature = "use_bindgen")]
pub fn main() {
    generate_shim_bindings();
}

#[cfg(not(feature = "use_bindgen"))]
pub fn main() {}
