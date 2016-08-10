mod drm_shim;

pub use self::drm_shim::*;
use super::error::{Error, Result};
use errno::errno;
use std::os::unix::io::RawFd;
use libc::ioctl;

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code, $obj) != 0 {
            return Err(Error::Ioctl(errno()));
        }
    })
}

pub struct DrmModeCardRes {
    pub raw: drm_mode_card_res,
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub framebuffers: Vec<u32>,
}

impl DrmModeCardRes {
    pub fn new(fd: RawFd) -> Result<DrmModeCardRes> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_card_res = Default::default();
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw);

        // Create buffers for each array
        let mut connectors: Vec<u32> =
            vec![Default::default(); raw.count_connectors as usize];
        let mut encoders: Vec<u32> =
            vec![Default::default(); raw.count_encoders as usize];
        let mut crtcs: Vec<u32> =
            vec![Default::default(); raw.count_crtcs as usize];
        let mut framebuffers: Vec<u32> =
            vec![Default::default(); raw.count_fbs as usize];

        // Pass a handle to the buffers to the raw struct
        raw.connector_id_ptr = connectors.as_mut_slice().as_mut_ptr() as u64;
        raw.encoder_id_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.crtc_id_ptr = crtcs.as_mut_slice().as_mut_ptr() as u64;
        raw.fb_id_ptr = framebuffers.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw);

        let res = DrmModeCardRes{
            raw: raw,
            connectors: connectors,
            encoders: encoders,
            crtcs: crtcs,
            framebuffers: framebuffers
        };

        Ok(res)
    }
}

pub struct DrmModeGetConnector {
    pub raw: drm_mode_get_connector,
    pub encoders: Vec<u32>,
    pub modes: Vec<drm_mode_modeinfo>,
    pub properties: Vec<u32>,
    pub prop_values: Vec<u32>,
}

impl DrmModeGetConnector {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetConnector> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_get_connector = Default::default();
        raw.connector_id = id;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

        // Create buffers for each array
        let mut encoders: Vec<u32> =
            vec![Default::default(); raw.count_encoders as usize];
        let mut modes: Vec<drm_mode_modeinfo> =
            vec![Default::default(); raw.count_modes as usize];
        let mut properties: Vec<u32> =
            vec![Default::default(); raw.count_props as usize];
        let mut prop_values: Vec<u32> =
            vec![Default::default(); raw.count_props as usize];

        // Pass a handle to the buffers to the raw struct
        raw.encoders_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.modes_ptr = modes.as_mut_slice().as_mut_ptr() as u64;
        raw.props_ptr = properties.as_mut_slice().as_mut_ptr() as u64;
        raw.prop_values_ptr = prop_values.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

        let conn = DrmModeGetConnector{
            raw: raw,
            encoders: encoders,
            modes: modes,
            properties: properties,
            prop_values: prop_values
        };

        Ok(conn)
    }
}

pub struct DrmModeGetEncoder {
    pub raw: drm_mode_get_encoder,
}

impl DrmModeGetEncoder {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetEncoder> {
        let mut raw: drm_mode_get_encoder = Default::default();
        raw.encoder_id = id;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETENCODER, &raw);
        let enc = DrmModeGetEncoder { raw: raw };
        Ok(enc)
    }
}

pub struct DrmModeGetCrtc {
    pub raw: drm_mode_crtc,
}

impl DrmModeGetCrtc {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetCrtc> {
        let mut raw: drm_mode_crtc = Default::default();
        raw.crtc_id = id;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_GETCRTC, &raw);
        let crtc = DrmModeGetCrtc { raw: raw };
        Ok(crtc)
    }
}

pub struct DrmModeAddFb {
    pub raw: drm_mode_fb_cmd,
}

impl DrmModeAddFb {
    pub fn new(fd: RawFd, width: u32, height: u32, pitch: u32, bpp: u32,
               depth: u32, handle: u32) -> Result<DrmModeAddFb> {
        let mut raw: drm_mode_fb_cmd = Default::default();
        raw.width = width;
        raw.height = height;
        raw.pitch = pitch;
        raw.bpp = bpp;
        raw.depth = depth;
        raw.handle = handle;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_ADDFB, &raw);
        let new = DrmModeAddFb { raw: raw };
        Ok(new)
    }
}

