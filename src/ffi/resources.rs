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
pub struct CardResources {
    pub raw: drm_mode_card_res,
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub framebuffers: Vec<u32>
}

pub fn get_card_resources(fd: RawFd) -> Result<CardResources> {
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

    let res = CardResources {
        raw: raw,
        connectors: connectors,
        encoders: encoders,
        crtcs: crtcs,
        framebuffers: framebuffers
    };

    Ok(res)
}

#[derive(Debug)]
pub struct PlaneResources {
    pub raw: drm_mode_get_plane_res,
    pub planes: Vec<u32>,
}

pub fn get_plane_resources(fd: RawFd) -> Result<PlaneResources> {
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

    let res = PlaneResources {
        raw: raw,
        planes: planes,
    };

    Ok(res)
}

#[derive(Debug)]
pub struct Connector {
    pub raw: drm_mode_get_connector,
    pub encoders: Vec<u32>,
    pub modes: Vec<drm_mode_modeinfo>,
    pub properties: Vec<u32>,
    pub prop_values: Vec<u64>
}

pub fn get_connector(fd: RawFd, id: u32) -> Result<Connector> {
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

    let conn = Connector {
        raw: raw,
        encoders: encoders,
        modes: modes,
        properties: properties,
        prop_values: prop_values
    };

    Ok(conn)
}

#[derive(Debug)]
pub struct Encoder {
    pub raw: drm_mode_get_encoder
}

pub fn get_encoder(fd: RawFd, id: u32) -> Result<Encoder> {
    let mut raw: drm_mode_get_encoder = unsafe { mem::zeroed() };
    raw.encoder_id = id;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETENCODER, &raw);
    let enc = Encoder { raw: raw };
    Ok(enc)
}

#[derive(Debug)]
pub struct Crtc {
    pub raw: drm_mode_crtc
}

pub fn get_crtc(fd: RawFd, id: u32) -> Result<Crtc> {
    let mut raw: drm_mode_crtc = unsafe { mem::zeroed() };
    raw.crtc_id = id;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCRTC, &raw);
    let crtc = Crtc { raw: raw };
    Ok(crtc)
}

#[derive(Debug)]
pub struct Framebuffer {
    pub raw: drm_mode_fb_cmd
}

pub fn get_framebuffer(fd: RawFd, id: u32) -> Result<Framebuffer> {
    let mut raw: drm_mode_fb_cmd = unsafe { mem::zeroed() };
    raw.fb_id = id;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETFB, &raw);
    let fb = Framebuffer { raw: raw };
    Ok(fb)
}

pub fn create_framebuffer(fd: RawFd, width: u32, height: u32, pitch: u32,
                          bpp: u32, depth: u32, handle: u32) -> Result<Framebuffer> {

    let mut raw: drm_mode_fb_cmd = unsafe { mem::zeroed() };
    raw.width = width;
    raw.height = height;
    raw.pitch = pitch;
    raw.bpp = bpp;
    raw.depth = depth;
    raw.handle = handle;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_ADDFB, &raw);

    let fb = Framebuffer { raw: raw };
    Ok(fb)
}

#[derive(Debug)]
pub struct Plane {
    pub raw: drm_mode_get_plane
}

pub fn get_plane(fd: RawFd, id: u32) -> Result<Plane> {
    let mut raw: drm_mode_get_plane = unsafe { mem::zeroed() };
    raw.plane_id = id;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANE, &raw);
    let plane = Plane { raw: raw };
    Ok(plane)
}

#[derive(Debug)]
pub struct AtomicRequest {
    pub raw: drm_mode_atomic
}
