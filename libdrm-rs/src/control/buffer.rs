use drm_sys::*;
use super::super::util::*;
use super::super::result::*;

use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct BufferId(pub ResourceId);

#[derive(Debug)]
pub struct DumbBuffer<'a, T> where T: 'a + AsRawFd + Sized {
    device: &'a T,
    id: BufferId,
    size: (u32, u32),
    bpp: u8,
    pitch: u32,
    length: u64
}

impl<'a, T> DumbBuffer<'a, T> where T: AsRawFd {
    pub fn new(device: &'a T, size: (u32, u32), bpp: u8) -> Result<DumbBuffer<'a, T>> {
        let mut raw: drm_mode_create_dumb = Default::default();
        raw.width = size.0;
        raw.height = size.1;
        raw.bpp = bpp as u32;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_CREATE_DUMB, &mut raw);

        let db = DumbBuffer {
            device: device,
            id: BufferId(raw.handle),
            size: (raw.width, raw.height),
            bpp: raw.bpp as u8,
            pitch: raw.pitch,
            length: raw.size
        };

        Ok(db)
    }

    pub fn remove(mut self) -> Result<()> {
        self.drop_with_result()?;
        mem::forget(self);
        Ok(())
    }

    fn drop_with_result(&mut self) -> Result<()> {
        let mut raw: drm_mode_destroy_dumb = Default::default();
        raw.handle = self.id.0;
        ioctl!(self.device, MACRO_DRM_IOCTL_MODE_DESTROY_DUMB, &mut raw);
        Ok(())
    }
}

#[cfg(not(debug_assertions))]
impl<'a, T> Drop for DumbBuffer<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.drop_with_result();
    }
}

#[cfg(debug_assertions)]
impl<'a, T> Drop for DumbBuffer<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.drop_with_result().unwrap();
    }
}

pub trait Buffer {
    fn size(&self) -> (u32, u32);
    fn pitch(&self) -> u32;
    fn bpp(&self) -> u8;
    fn depth(&self) -> u32;
    fn handle(&self) -> BufferId;
}

impl<'a, T> Buffer for DumbBuffer<'a, T> where T: AsRawFd {
    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn pitch(&self) -> u32 {
        self.pitch
    }

    fn bpp(&self) -> u8 {
        self.bpp
    }

    fn depth(&self) -> u32 {
        32
    }

    fn handle(&self) -> BufferId {
        self.id
    }
}
