mod drm_shim;

pub use self::drm_shim::*;
use std::io::Error;
use ::error::Result;
use std::os::unix::io::RawFd;
use std::ptr::null;
use libc::{ioctl, c_void};

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code, $obj) != 0 {
            return Err(Error::last_os_error().into());
        }
    })
}

pub fn set_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, FFI_DRM_IOCTL_SET_MASTER, null() as *const c_void);
    Ok(())
}

pub fn drop_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, FFI_DRM_IOCTL_DROP_MASTER, null() as *const c_void);
    Ok(())
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
        let mut prop_values: Vec<u64> =
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

#[derive(Debug)]
pub struct DrmModeGetEncoder {
    pub raw: drm_mode_get_encoder
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

#[derive(Debug)]
pub struct DrmModeGetCrtc {
    pub raw: drm_mode_crtc
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

#[derive(Debug)]
pub struct DrmModeSetCrtc {
    pub raw: drm_mode_crtc
}

impl DrmModeSetCrtc {
    pub fn new(fd: RawFd, id: u32, fb_id: u32, x: u32, y: u32, mut connectors: Vec<u32>, mode: drm_mode_modeinfo) -> Result<DrmModeSetCrtc> {
        let mut raw: drm_mode_crtc = Default::default();
        raw.crtc_id = id;
        raw.fb_id = fb_id;
        raw.x = x;
        raw.y = y;
        raw.mode = mode;
        raw.mode_valid = 1;
        raw.count_connectors = connectors.len() as u32;
        raw.set_connectors_ptr = connectors.as_mut_slice().as_mut_ptr() as u64;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_SETCRTC, &raw);
        let crtc = DrmModeSetCrtc { raw: raw };
        Ok(crtc)
    }
}

#[derive(Debug)]
pub struct DrmModeAddFb {
    pub raw: drm_mode_fb_cmd
}

impl DrmModeAddFb {
    pub fn new(fd: RawFd, width: u32, height: u32, depth: u8, bpp: u8,
               pitch: u32, handle: u32) -> Result<DrmModeAddFb> {
        let mut raw: drm_mode_fb_cmd = Default::default();
        raw.width = width;
        raw.height = height;
        raw.depth = depth as u32;
        raw.bpp = bpp as u32;
        raw.pitch = pitch;
        raw.handle = handle;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_ADDFB, &raw);
        let fb = DrmModeAddFb { raw: raw };
        Ok(fb)
    }
}

#[derive(Debug)]
pub struct DrmModeRmFb;

impl DrmModeRmFb {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeRmFb> {
        let raw = id;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_RMFB, &raw);
        let fb = DrmModeRmFb;
        Ok(fb)
    }
}

#[derive(Debug)]
pub struct DrmModeCreateDumbBuffer {
    pub raw: drm_mode_create_dumb
}

impl DrmModeCreateDumbBuffer {
    pub fn new(fd: RawFd, width: u32, height: u32, bpp: u8) -> Result<DrmModeCreateDumbBuffer> {
        let mut raw: drm_mode_create_dumb = Default::default();
        raw.width = width;
        raw.height = height;
        raw.bpp = bpp as u32;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_CREATE_DUMB, &raw);
        let buffer = DrmModeCreateDumbBuffer { raw: raw };
        Ok(buffer)
    }
}

#[derive(Debug)]
pub struct DrmModeMapDumbBuffer {
    pub raw: drm_mode_map_dumb
}

impl DrmModeMapDumbBuffer {
    pub fn new(fd: RawFd, handle: u32) -> Result<DrmModeMapDumbBuffer> {
        let mut raw: drm_mode_map_dumb = Default::default();
        raw.handle = handle;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let map = DrmModeMapDumbBuffer { raw: raw };
        Ok(map)
    }
}

#[derive(Debug)]
pub struct DrmModeDestroyDumbBuffer {
    pub raw: drm_mode_destroy_dumb
}

impl DrmModeDestroyDumbBuffer {
    pub fn new(fd: RawFd, handle: u32) -> Result<DrmModeDestroyDumbBuffer> {
        let mut raw: drm_mode_destroy_dumb = Default::default();
        raw.handle = handle;
        ioctl!(fd, FFI_DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let destroy = DrmModeDestroyDumbBuffer { raw: raw };
        Ok(destroy)
    }
}

