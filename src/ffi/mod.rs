#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

mod resources;
mod properties;

pub use self::resources::*;
pub use self::properties::*;

use std::mem;
use std::ptr::null;
use std::os::unix::io::RawFd;
use std::io::Error as IoError;
use libc::{ioctl, c_void};
use ::result::Result;

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
}

pub fn set_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, MACRO_DRM_IOCTL_SET_MASTER, null() as *const c_void);
    Ok(())
}

pub fn drop_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, MACRO_DRM_IOCTL_DROP_MASTER, null() as *const c_void);
    Ok(())
}

pub fn enable_atomic(fd: RawFd) -> Result<()> {
    let mut raw: drm_set_client_cap = unsafe { mem::zeroed() };
    raw.capability = DRM_CLIENT_CAP_UNIVERSAL_PLANES as u64;
    raw.value = 1;

    ioctl!(fd, MACRO_DRM_IOCTL_SET_CLIENT_CAP, &raw);
    Ok(())
}

pub fn enable_universal_planes(fd: RawFd) -> Result<()> {
    let mut raw: drm_set_client_cap = unsafe { mem::zeroed() };
    raw.capability = DRM_CLIENT_CAP_ATOMIC as u64;
    raw.value = 1;

    ioctl!(fd, MACRO_DRM_IOCTL_SET_CLIENT_CAP, &raw);
    Ok(())
}

#[derive(Debug)]
pub struct DrmModeCreateDumbBuffer {
    pub raw: drm_mode_create_dumb
}

impl DrmModeCreateDumbBuffer {
    pub fn new(fd: RawFd, width: u32, height: u32, bpp: u8) -> Result<DrmModeCreateDumbBuffer> {
        let mut raw: drm_mode_create_dumb = unsafe { mem::zeroed() };
        raw.width = width;
        raw.height = height;
        raw.bpp = bpp as u32;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_CREATE_DUMB, &raw);
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
        let mut raw: drm_mode_map_dumb = unsafe { mem::zeroed() };
        raw.handle = handle;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_MAP_DUMB, &raw);
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
        let mut raw: drm_mode_destroy_dumb = unsafe { mem::zeroed() };
        raw.handle = handle;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let destroy = DrmModeDestroyDumbBuffer { raw: raw };
        Ok(destroy)
    }
}

