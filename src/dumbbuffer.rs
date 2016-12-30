use ::ffi;
use ::Context;
use ::Buffer;
use ::result::Result;

use std::os::unix::io::{AsRawFd, RawFd};
use std::convert::AsMut;
use std::fs::File;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;
use std::marker::PhantomData;
use std::borrow::Borrow;

use libc::{mmap, munmap, c_void, PROT_READ, PROT_WRITE, MAP_SHARED};

impl<T> Context<T> where T: AsRawFd {
    pub fn create_dumbbuffer<'a>(&'a self, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer<'a>> {
        let fd = self.handle.borrow().as_raw_fd();
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(fd, width, height, bpp));
        let buffer = DumbBuffer {
            _phantom: PhantomData,
            fd: fd,
            size: (width, height),
            depth: 24,
            bpp: bpp,
            pitch: raw.raw.pitch,
            handle: raw.raw.handle,
            raw_size: raw.raw.size as usize,
        };

        Ok(buffer)
    }

}

/// A `DumbBuffer` is a simple buffer type provided by all major graphics
/// drivers. It can be mapped to main memory and provided direct access to the
/// pixel data to be displayed.
#[derive(Debug)]
pub struct DumbBuffer<'a> {
    _phantom: PhantomData<&'a ()>,
    fd: RawFd,
    size: (u32, u32),
    depth: u8,
    bpp: u8,
    pitch: u32,
    handle: u32,
    raw_size: usize,
}

impl<'a> Drop for DumbBuffer<'a> {
    fn drop(&mut self) {
        let _ = ffi::DrmModeDestroyDumbBuffer::new(self.fd, self.handle);
    }
}

impl<'a> DumbBuffer<'a> {
    pub fn map(&'a self) -> Result<DumbMapping<'a>> {
        let raw = try!(ffi::DrmModeMapDumbBuffer::new(self.fd, self.handle));
        let ptr = unsafe {
            mmap(null_mut(), self.raw_size, PROT_READ | PROT_WRITE, MAP_SHARED, self.fd, raw.raw.offset as i64)
        } as *mut u8;
        let map = unsafe {
            from_raw_parts_mut(ptr, self.raw_size)
        };
        let mapping = DumbMapping {
            map: map
        };
        Ok(mapping)
    }
}

impl<'a> Buffer for DumbBuffer<'a> {
    fn size(&self) -> (u32, u32) { self.size }
    fn depth(&self) -> u8 { self.depth }
    fn bpp(&self) -> u8 { self.bpp }
    fn pitch(&self) -> u32 { self.pitch }
    fn handle(&self) -> u32 { self.handle }
}

#[derive(Debug)]
pub struct DumbMapping<'a> {
    map: &'a mut [u8]
}

impl<'a> DumbMapping<'a> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.map
    }
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
/*
impl<C, T, F> DumbBuffer<C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    /// Attempts to create a `DumbBuffer` from the given size and bits per
    /// pixel.
    pub fn create(ctx: C, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer<C, T, F>> {
        let fd = ctx.borrow().as_raw_fd();
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(fd, width, height, bpp));
        let buffer = DumbBuffer {
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
    pub fn map(&self) -> Result<DumbMapping<C, T, F>> {
        let raw = try!(ffi::DrmModeMapDumbBuffer::new(self.ctx.borrow().as_raw_fd(), self.handle));
        let ptr = unsafe {
            mmap(null_mut(), self.raw_size, PROT_READ | PROT_WRITE, MAP_SHARED, self.ctx.borrow().as_raw_fd(), raw.raw.offset as i64)
        } as *mut u8;
        let map = unsafe {
            from_raw_parts_mut(ptr, self.raw_size)
        };
        let mapping = DumbMapping {
            _phantom: PhantomData,
            map: map
        };
        Ok(mapping)
    }
}

impl<C, T, F> Drop for DumbBuffer<C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    fn drop(&mut self) {
        ffi::DrmModeDestroyDumbBuffer::new(self.ctx.borrow().as_raw_fd(), self.handle).unwrap();
    }
}

/// A `DumbMapping` is the mapping of a `DumbBuffer`. You can read and write
/// directly into the map and it will be mapped to the `DumbBuffer`. It is
/// recommended to use `copy_from_slice` to write to the buffer, as this data
/// is copied to the graphics card on each write.
pub struct DumbMapping<'a, C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    _phantom: PhantomData<DumbBuffer<C, T, F>>,
    map: &'a mut [u8]
}

impl<'a, C, T, F> DumbMapping<'a, C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    fn map_buffer(&'a mut self) -> &'a mut [u8] {
        self.map
    }
}

impl<'a, C, T, F> Drop for DumbMapping<'a, C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    fn drop(&mut self) {
        let addr = self.map.as_mut_ptr();
        let size = self.map.len();
        unsafe {
            munmap(addr as *mut c_void, size);
        }
    }
}

impl<C, T, F> Buffer for DumbBuffer<C, T, F> where C: Borrow<Context<T, F>>, T: Borrow<F>, F: AsRawFd {
    fn size(&self) -> (u32, u32) { self.size }
    fn depth(&self) -> u8 { self.depth }
    fn bpp(&self) -> u8 { self.bpp }
    fn pitch(&self) -> u32 { self.pitch }
    fn handle(&self) -> u32 { self.handle }
}
*/
