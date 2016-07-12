extern crate gcc;

// Compile and link to the drm-shim
fn compile_drm_shim() {
    gcc::Config::new()
        .file("src/ffi/drm-ioctl.c")
        .debug(false)
        .compile("libffi.a");
}

pub fn main() {
    compile_drm_shim();
}
