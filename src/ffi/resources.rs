use std::mem;
use std::os::unix::io::RawFd;
use std::io::Error as IoError;
use libc::ioctl;
use ::result::Result;

use super::*;

macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
}

#[derive(Debug)]
pub struct DrmModeCardRes {
    pub raw: drm_mode_card_res,
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub framebuffers: Vec<u32>
}

impl DrmModeCardRes {
    pub fn new(fd: RawFd) -> Result<DrmModeCardRes> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_card_res = unsafe { mem::zeroed() };
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &raw);

        // Create buffers for each array
        let mut connectors: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_connectors as usize];
        let mut encoders: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_encoders as usize];
        let mut crtcs: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_crtcs as usize];
        let mut framebuffers: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_fbs as usize];

        // Pass a handle to the buffers to the raw struct
        raw.connector_id_ptr = connectors.as_mut_slice().as_mut_ptr() as u64;
        raw.encoder_id_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.crtc_id_ptr = crtcs.as_mut_slice().as_mut_ptr() as u64;
        raw.fb_id_ptr = framebuffers.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &raw);

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

#[derive(Debug)]
pub struct DrmModeGetConnector {
    pub raw: drm_mode_get_connector,
    pub encoders: Vec<u32>,
    pub modes: Vec<drm_mode_modeinfo>,
    pub properties: Vec<u32>,
    pub prop_values: Vec<u64>
}

impl DrmModeGetConnector {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetConnector> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_get_connector = unsafe { mem::zeroed() };
        raw.connector_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

        // Create buffers for each array
        let mut encoders: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_encoders as usize];
        let mut modes: Vec<drm_mode_modeinfo> =
            vec![unsafe { mem::zeroed() }; raw.count_modes as usize];
        let mut properties: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];
        let mut prop_values: Vec<u64> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];

        // Pass a handle to the buffers to the raw struct
        raw.encoders_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.modes_ptr = modes.as_mut_slice().as_mut_ptr() as u64;
        raw.props_ptr = properties.as_mut_slice().as_mut_ptr() as u64;
        raw.prop_values_ptr = prop_values.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

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

#[derive(Debug)]
pub struct DrmModeGetEncoder {
    pub raw: drm_mode_get_encoder
}

impl DrmModeGetEncoder {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetEncoder> {
        let mut raw: drm_mode_get_encoder = unsafe { mem::zeroed() };
        raw.encoder_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETENCODER, &raw);
        let enc = DrmModeGetEncoder { raw: raw };
        Ok(enc)
    }
}

#[derive(Debug)]
pub struct DrmModeGetCrtc {
    pub raw: drm_mode_crtc
}

impl DrmModeGetCrtc {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetCrtc> {
        let mut raw: drm_mode_crtc = unsafe { mem::zeroed() };
        raw.crtc_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCRTC, &raw);
        let crtc = DrmModeGetCrtc { raw: raw };
        Ok(crtc)
    }
}

#[derive(Debug)]
pub struct DrmModePlaneRes {
    pub raw: drm_mode_get_plane_res,
    pub planes: Vec<u32>,
}

impl DrmModePlaneRes {
    pub fn new(fd: RawFd) -> Result<DrmModePlaneRes> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_get_plane_res = unsafe { mem::zeroed() };
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &raw);

        // Create buffers for each array
        let mut planes: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_planes as usize];

        // Pass a handle to the buffers to the raw struct
        raw.plane_id_ptr = planes.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &raw);

        let res = DrmModePlaneRes{
            raw: raw,
            planes: planes,
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct DrmModeGetPlane {
    pub raw: drm_mode_get_plane
}

impl DrmModeGetPlane {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetPlane> {
        let mut raw: drm_mode_get_plane = unsafe { mem::zeroed() };
        raw.plane_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANE, &raw);
        let plane = DrmModeGetPlane { raw: raw };
        Ok(plane)
    }
}
