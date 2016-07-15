mod drm_shim;

pub use self::drm_shim::*;
use super::error::{Error, Result};
use errno::errno;

use std::os::unix::io::RawFd;
use libc::ioctl;

// Wrap the ioctl function to return errno on failure.
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code, $obj) != 0 {
            return Err(Error::Ioctl(errno()));
        }
    })
}

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

pub fn drm_ioctl_mode_get_resources(fd: RawFd) -> Result<DrmModeCardRes> {
    // The first time we call the ioctl, the counts will be filled.
    let mut raw_res = drm_mode_card_res::default();
    ioctl!(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw_res);

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
    ioctl!(fd, FFI_DRM_IOCTL_MODE_GETRESOURCES, &raw_res);

    let res = DrmModeCardRes {
        connectors: conns,
        encoders: encs,
        crtcs: crtcs,
        framebuffers: fbs,
        min_width: raw_res.min_width,
        max_width: raw_res.max_width,
        min_height: raw_res.min_height,
        max_height: raw_res.max_height,
    };

    Ok(res)
}

#[derive(Debug)]
pub struct DrmModeGetConnector {
    pub encoders: Vec<u32>,
    pub modes: Vec<u32>,
    pub properties: Vec<u32>,
    pub prop_values: Vec<u32>,
    pub encoder: u32,
    pub connector: u32,
    pub conn_type: u32,
    pub conn_type_id: u32,
    pub connected: u32,
    pub size: (u32, u32),
    pub subpixel: u32
}

pub fn drm_ioctl_mode_get_connector(fd: RawFd, id: u32) -> Result<DrmModeGetConnector> {
    // The first time we call the ioctl, the counts will be filled
    let mut raw_con = drm_mode_get_connector::default();
    raw_con.connector_id = id;
    ioctl!(fd, FFI_DRM_IOCTL_MODE_GETCONNECTOR, &raw_con);

    // Let's use those counts to allocate the vectors
    let mut encs = vec![0u32; raw_con.count_encoders as usize];
    let mut modes = vec![0u32; raw_con.count_modes as usize];
    let mut props = vec![0u32; raw_con.count_props as usize];
    let mut prop_vals = vec![0u32; raw_con.count_props as usize];

    raw_con.encoders_ptr = encs.as_mut_slice().as_mut_ptr() as u64;
    raw_con.modes_ptr = modes.as_mut_slice().as_mut_ptr() as u64;
    raw_con.props_ptr = props.as_mut_slice().as_mut_ptr() as u64;
    raw_con.prop_values_ptr = prop_vals.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, FFI_DRM_IOCTL_MODE_GETCONNECTOR, &raw_con);

    let conn = DrmModeGetConnector {
        encoders: encs,
        modes: modes,
        properties: props,
        prop_values: prop_vals,
        encoder: raw_con.encoder_id,
        connector: raw_con.connector_id,
        conn_type: raw_con.connector_type,
        conn_type_id: raw_con.connector_type_id,
        connected: raw_con.connection,
        size: (raw_con.mm_width, raw_con.mm_height),
        subpixel: raw_con.subpixel
    };

    Ok(conn)
}


