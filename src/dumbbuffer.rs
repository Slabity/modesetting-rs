use ::ffi;
use ::Buffer;
use ::result::Result;

use std::os::unix::io::AsRawFd;
use std::fs::File;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;
use std::marker::PhantomData;

use libc::{mmap, munmap, c_void, PROT_READ, PROT_WRITE, MAP_SHARED};

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
    pub fn map(&self) -> Result<DumbMapping> {
        let raw = try!(ffi::DrmModeMapDumbBuffer::new(self.device.as_raw_fd(), self.handle));
        let ptr = unsafe {
            mmap(null_mut(), self.raw_size, PROT_READ | PROT_WRITE, MAP_SHARED, self.device.as_raw_fd(), raw.raw.offset as i64)
        } as *mut u8;
        let map = unsafe {
            from_raw_parts_mut(ptr, self.raw_size)
        };
        let mapping = DumbMapping {
            buffer: PhantomData,
            map: map
        };
        Ok(mapping)
    }
}

impl<'a> Drop for DumbBuffer<'a> {
    fn drop(&mut self) {
        ffi::DrmModeDestroyDumbBuffer::new(self.device.as_raw_fd(), self.handle).unwrap();
    }
}

/// A `DumbMapping` is the mapping of a `DumbBuffer`. You can read and write
/// directly into the map and it will be mapped to the `DumbBuffer`. It is
/// recommended to use `copy_from_slice` to write to the buffer, as this data
/// is copied to the graphics card on each write.
pub struct DumbMapping<'a> {
    pub map: &'a mut [u8],
    buffer: PhantomData<DumbBuffer<'a>>
}

impl<'a> Drop for DumbMapping<'a> {
    fn drop(&mut self) {
        let addr = self.map.as_mut_ptr();
        let size = self.map.len();
        unsafe {
            munmap(addr as *mut c_void, size);
        }
    }
}

impl<'a> Buffer for DumbBuffer<'a> {
    fn size(&self) -> (u32, u32) { self.size }
    fn depth(&self) -> u8 { self.depth }
    fn bpp(&self) -> u8 { self.bpp }
    fn pitch(&self) -> u32 { self.pitch }
    fn handle(&self) -> u32 { self.handle }
}

