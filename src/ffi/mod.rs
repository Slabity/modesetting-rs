include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

use std::io::Error;
use ::result::Result;
use std::os::unix::io::RawFd;
use std::ptr::null;
use libc::{ioctl, c_void};

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(Error::last_os_error().into());
        }
    })
}

pub fn set_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, DRM_IOCTL_SET_MASTER, null() as *const c_void);
    Ok(())
}

pub fn drop_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, DRM_IOCTL_DROP_MASTER, null() as *const c_void);
    Ok(())
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
        ioctl!(fd, DRM_IOCTL_MODE_CREATE_DUMB, &raw);
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
        ioctl!(fd, DRM_IOCTL_MODE_MAP_DUMB, &raw);
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
        ioctl!(fd, DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let destroy = DrmModeDestroyDumbBuffer { raw: raw };
        Ok(destroy)
    }
}

