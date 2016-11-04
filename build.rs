extern crate gcc;
extern crate bindgen;

// Generate rust bindings to access drm structs
fn generate_shim_bindings() {
    let mut builder = bindgen::Builder::default();
    builder = builder.header("/usr/include/libdrm/drm_mode.h");
    builder = builder.link("drm");

    // Needed DRM constants
    builder = builder.whitelisted_type("DRM_MODE_ATOMIC_FLAGS");

    // Needed DRM structs
    builder = builder.whitelisted_type("drm_mode_atomic");
    builder = builder.whitelisted_type("drm_mode_modeinfo");
    builder = builder.whitelisted_type("drm_mode_create_dumb");
    builder = builder.whitelisted_type("drm_mode_map_dumb");
    builder = builder.whitelisted_type("drm_mode_destroy_dumb");

    // Needed DRM ioctl numbers
    builder = builder.whitelisted_var("DRM_IOCTL_SET_MASTER");
    builder = builder.whitelisted_var("DRM_IOCTL_DROP_MASTER");
    builder = builder.whitelisted_var("DRM_IOCTL_MODE_CREATE_DUMB");
    builder = builder.whitelisted_var("DRM_IOCTL_MODE_DESTROY_DUMB");
    builder = builder.whitelisted_var("DRM_IOCTL_MODE_MAP_DUMB");

    match builder.generate() {
        Ok(b) => b.write_to_file(concat!(env!("OUT_DIR"), "/ffi.rs")).unwrap(),
        Err(e) => panic!(e)
    };
}

pub fn main() {
    generate_shim_bindings();
}
