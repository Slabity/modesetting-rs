use drm_sys::*;
use super::super::util::*;
use super::super::result::*;

use std::os::unix::io::AsRawFd;
use std::slice::from_raw_parts_mut;
use libc;

#[derive(Debug, Clone, Copy)]
pub struct BufferId(pub RawId);

#[derive(Debug)]
pub struct DumbBuffer<'a, T> where T: 'a + AsRawFd + Sized {
    device: &'a T,
    id: BufferId,
    size: (u32, u32),
    bpp: u8,
    pitch: u32,
    length: u64
}

#[derive(Debug)]
pub struct DumbMapping<'a, T> where T: 'a + AsRawFd + Sized {
    buffer: &'a DumbBuffer<'a, T>,
    slice: &'a mut [u8]
}

impl<'a, T> DumbBuffer<'a, T> where T: AsRawFd {
    pub fn new(device: &'a T, size: (u16, u16), bpp: u8) -> Result<DumbBuffer<'a, T>> {
        let mut raw: drm_mode_create_dumb = Default::default();
        raw.width = size.0 as u32;
        raw.height = size.1 as u32;
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

    pub fn map(&'a self) -> Result<DumbMapping<'a, T>> {
        let mut raw: drm_mode_map_dumb = Default::default();
        raw.handle = self.id.0;
        ioctl!(self.device, MACRO_DRM_IOCTL_MODE_MAP_DUMB, &mut raw);

        let ptr = unsafe {
            let base = 0 as *mut _;
            let length = self.length as usize;
            let prot = libc::PROT_READ | libc::PROT_WRITE;
            let flags = libc::MAP_SHARED;
            let fd = self.device.as_raw_fd();
            libc::mmap(base, length, prot, flags, fd, raw.offset as i64)
        } as *mut _;

        let slice = unsafe { from_raw_parts_mut(ptr, self.length as usize) };

        let map = DumbMapping {
            buffer: self,
            slice: slice
        };

        Ok(map)
    }

    pub fn remove(&mut self) -> Result<()> {
        let mut raw: drm_mode_destroy_dumb = Default::default();
        raw.handle = self.id.0;
        ioctl!(self.device, MACRO_DRM_IOCTL_MODE_DESTROY_DUMB, &mut raw);
        Ok(())
    }
}

#[cfg(not(debug_assertions))]
impl<'a, T> Drop for DumbBuffer<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.remove();
    }
}

#[cfg(debug_assertions)]
impl<'a, T> Drop for DumbBuffer<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.remove().unwrap();
    }
}

impl<'a, T> DumbMapping<'a, T> where T: 'a + AsRawFd + Sized {
    pub fn as_slice<'b>(&'b mut self) -> &'b mut [u8] {
        self.slice
    }

    pub fn remove(&mut self) -> Result<()> {
        let res = unsafe {
            libc::munmap(self.slice.as_mut_ptr() as *mut _, self.slice.len())
        };

        match res {
            0 => Ok(()),
            _ => Err(IoctlError::last_os_error().into())
        }
    }
}

#[cfg(not(debug_assertions))]
impl<'a, T> Drop for DumbMapping<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.remove();
    }
}

#[cfg(debug_assertions)]
impl<'a, T> Drop for DumbMapping<'a, T> where T: AsRawFd {
    fn drop(&mut self) {
        self.remove().unwrap();
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
        24
    }

    fn handle(&self) -> BufferId {
        self.id
    }
}
