mod drm_shim;

pub use self::drm_shim::*;
use super::error::{Error, Result};
use errno::errno;

use std::os::unix::io::RawFd;
use libc::ioctl;

// This macro simple wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code, $obj) != 0 {
            return Err(Error::Ioctl(errno()));
        }
    })
}

// A large number of the ioctl calls used need to be called twice. This is
// because the system does not allocate memory for buffers. Instead, it stores
// the number of elements that a buffer needs to have and leaves it to the
// program to allocate and deallocate the buffers. Then the ioctl call is made
// again and the system fills the buffers up. Manual allocation in rust is a
// pain though. Instead, we create a Vec<T> buffer to store the elements and
// let the compiler deallocate it when the struct itself is deallocated.
//
// This macro takes care of it for us by creating a new type that stores both
// the raw C struct and all the buffers used.
macro_rules! buffered_ioctl_struct {
    (
        Create $new_ty:ident from $raw_ty:ty;
        Ioctl $ioctl:ident;
        $(
            Set $raw_var:ident to $pass_var:ident with type $pass_ty:ty;
        )*
        $(
            Buffer $new_val:ident from ($raw_count:ident, $raw_val:ident) with type $buf_ty:ty;
        )* ) => (

        // Create a new struct named $new_ty
        pub struct $new_ty {
            pub raw: $raw_ty,

            // Create a new field for each buffer.
            $(
                pub $new_val: Vec<$buf_ty>,
            )*
        }

        impl $new_ty {
            pub fn new(fd: RawFd$(, $pass_var: $pass_ty)*) -> Result<$new_ty> {
                // Create the C struct and set the default value
                let mut raw: $raw_ty = Default::default();

                // Set whatever variables in the struct we need
                $(
                raw.$raw_var = $pass_var;
                )*

                // First call fills in the buffer sizes
                ioctl!(fd, $ioctl, &raw);

                // Create each buffer with each size and type
                $(
                let mut $new_val: Vec<$buf_ty> = vec![Default::default(); raw.$raw_count as usize];
                raw.$raw_val = $new_val.as_mut_slice().as_mut_ptr() as u64;
                )*

                // Second call fills up the buffers
                ioctl!(fd, $ioctl, &raw);

                let new = $new_ty {
                    raw: raw,
                    $(
                        $new_val: $new_val,
                    )*
                };

                Ok(new)
            }
        }
    )
}

buffered_ioctl_struct!(
    Create DrmModeCardRes from drm_mode_card_res;
    Ioctl FFI_DRM_IOCTL_MODE_GETRESOURCES;
    Buffer connectors from (count_connectors, connector_id_ptr) with type u32;
    Buffer encoders from (count_encoders, encoder_id_ptr) with type u32;
    Buffer crtcs from (count_crtcs, crtc_id_ptr) with type u32;
    Buffer framebuffers from (count_fbs, fb_id_ptr) with type u32;
    );

buffered_ioctl_struct!(
    Create DrmModeGetConnector from drm_mode_get_connector;
    Ioctl FFI_DRM_IOCTL_MODE_GETCONNECTOR;
    Set connector_id to id with type u32;
    Buffer encoders from (count_encoders, encoders_ptr) with type u32;
    Buffer modes from (count_modes, modes_ptr) with type drm_mode_modeinfo;
    Buffer properties from (count_props, props_ptr) with type u32;
    Buffer prop_values from (count_props, prop_values_ptr) with type u32;
    );

// Note that this one doesn't have any buffers. But by pure luck we can use
// the macro for it just as well.
buffered_ioctl_struct!(
    Create DrmModeGetEncoder from drm_mode_get_encoder;
    Ioctl FFI_DRM_IOCTL_MODE_GETENCODER;
    Set encoder_id to id with type u32;
    );

buffered_ioctl_struct!(
    Create DrmModeGetCrtc from drm_mode_crtc;
    Ioctl FFI_DRM_IOCTL_MODE_GETCRTC;
    Set crtc_id to id with type u32;
    );

buffered_ioctl_struct!(
    Create DrmModeGetFb from drm_mode_fb_cmd;
    Ioctl FFI_DRM_IOCTL_MODE_GETFB;
    Set fb_id to id with type u32;
    );

buffered_ioctl_struct!(
    Create DrmModeGetProperty from drm_mode_get_property;
    Ioctl FFI_DRM_IOCTL_MODE_GETPROPERTY;
    Set prop_id to id with type u32;
    Buffer values from (count_values, values_ptr) with type u32;
    Buffer enum_blobs from (count_enum_blobs, enum_blob_ptr) with type drm_mode_property_enum;
    );

buffered_ioctl_struct!(
    Create DrmModeGetBlob from drm_mode_get_blob;
    Ioctl FFI_DRM_IOCTL_MODE_GETPROPBLOB;
    Set blob_id to id with type u32;
    Buffer data from (length, data) with type u8;
    );

pub fn DrmModeAddFb2(fd: RawFd, width: u32, height: u32, pixels: u32, handles: [u32; 4], pitches: [u32; 4], offsets: [u32; 4]) -> Result<u32> {
    let mut fb_cmd = drm_mode_fb_cmd2::default();

    fb_cmd.width = width;
    fb_cmd.height = height;
    fb_cmd.pixel_format = pixels;

    fb_cmd.handles = handles;
    fb_cmd.pitches = pitches;
    fb_cmd.offsets = offsets;

    ioctl!(fd, FFI_DRM_IOCTL_MODE_ADDFB2, &fb_cmd);

    Ok(fb_cmd.fb_id)
}

