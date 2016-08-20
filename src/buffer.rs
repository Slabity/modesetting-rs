use super::ffi;
use super::error::Result;

use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;

use libc::{mmap, munmap, PROT_READ, PROT_WRITE, MAP_SHARED};

/// An object that implements the `Buffer` trait allows it to be used as a part
/// of a `Framebuffer`.
pub trait Buffer {
    /// The width and height of the buffer.
    fn size(&self) -> (u32, u32);
    /// The depth size of the buffer.
    fn depth(&self) -> u8;
    /// The number of 'bits per pixel'
    fn bpp(&self) -> u8;
    /// The pitch of the buffer.
    fn pitch(&self) -> u32;
    /// A handle provided by your graphics driver that can be used to reference
    /// the buffer, such as a dumb buffer handle or a handle provided by mesa's
    /// libgbm.
    fn handle(&self) -> u32;
}

/// A `DumbBuffer` is a simple buffer type provided by all major graphics
/// drivers. It can be mapped to main memory and provided direct access to the
/// pixel data to be displayed.
pub struct DumbBuffer<'a> {
    device: &'a File,
    size: (u32, u32),
    depth: u8,
    bpp: u8,
    pitch: u32,
    handle: u32,
    raw_size: usize
}

impl<'a> DumbBuffer<'a> {
    /// Attempts to create a `DumbBuffer` from the given size and bits per
    /// pixel.
    pub fn create<T: 'a + AsRef<File>>(device: &'a T, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(device.as_ref().as_raw_fd(), width, height, bpp));
        let buffer = DumbBuffer {
            device: device.as_ref(),
            size: (width, height),
            depth: 24,
            bpp: bpp,
            pitch: raw.raw.pitch,
            handle: raw.raw.handle,
            raw_size: raw.raw.size as usize
        };
        Ok(buffer)
    }

    /// Attempts to map the buffer directly into main memory as represented by
    /// a mutable `&[u8]`. Because this data is copied to the graphics card on
    /// each write, it is recommended to draw into another buffer of identical
    /// size and then copy its contents using `copy_from_slice`.
    pub fn map(&self) -> Result<&mut [u8]> {
        let raw = try!(ffi::DrmModeMapDumbBuffer::new(self.device.as_raw_fd(), self.handle));
        let ptr = unsafe {
            mmap(null_mut(), self.raw_size, PROT_READ | PROT_WRITE, MAP_SHARED, self.device.as_raw_fd(), raw.raw.offset as i64)
        } as *mut u8;
        Ok(unsafe {
            from_raw_parts_mut(ptr, self.raw_size)
        })
    }
}

impl<'a> Drop for DumbBuffer<'a> {
    fn drop(&mut self) {
        ffi::DrmModeDestroyDumbBuffer::new(self.device.as_raw_fd(), self.handle).unwrap();
    }
}

impl<'a> Buffer for DumbBuffer<'a> {
    fn size(&self) -> (u32, u32) { self.size }
    fn depth(&self) -> u8 { self.depth }
    fn bpp(&self) -> u8 { self.bpp }
    fn pitch(&self) -> u32 { self.pitch }
    fn handle(&self) -> u32 { self.handle }
}


