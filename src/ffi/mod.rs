mod drm_shim;

pub use self::drm_shim::*;

use std::os::unix::io::{RawFd, AsRawFd};
use libc::ioctl;

#[derive(Debug)]
pub struct DrmModeCardRes {
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub framebuffers: Vec<u32>,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

pub fn drm_ioctl_mode_get_resources(fd: RawFd) -> DrmModeCardRes {
    // The first time we call the ioctl, the counts will be filled.
    let mut raw_res = drm_mode_card_res::default();
    unsafe { ioctl(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw_res) };

    // Let's use those counts to allocate some vectors of those sizes.
    let mut conns = vec![0u32; raw_res.count_connectors as usize];
    let mut encs = vec![0u32; raw_res.count_encoders as usize];
    let mut crtcs = vec![0u32; raw_res.count_crtcs as usize];
    let mut fbs = vec![0u32; raw_res.count_fbs as usize];

    // Let's use the above vectors as buffers for the resources
    raw_res.connector_id_ptr = conns.as_mut_slice().as_mut_ptr() as u64;
    raw_res.encoder_id_ptr = encs.as_mut_slice().as_mut_ptr() as u64;
    raw_res.crtc_id_ptr = crtcs.as_mut_slice().as_mut_ptr() as u64;
    raw_res.fb_id_ptr = fbs.as_mut_slice().as_mut_ptr() as u64;

    // The second time we call it, all buffers will be filled
    unsafe { ioctl(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw_res) };

    DrmModeCardRes {
        connectors: conns,
        encoders: encs,
        crtcs: crtcs,
        framebuffers: fbs,
        min_width: raw_res.min_width,
        max_width: raw_res.max_width,
        min_height: raw_res.min_height,
        max_height: raw_res.max_height,
    }
}
