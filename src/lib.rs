/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block device and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a Device and using four types of resources:

  - Connectors: The physical interfaces on your GPU, such as HDMI, VGA, and
  LVDS ports.
  - Encoders: These modify and deliver the pixel data to the connectors.
  - CRTCs: Points to a scanout buffer in video memory and reads it based on the
  mode it is set to.
  - Framebuffer: Pixel data that can be used by a CRTC

  The standard procedure to do this is to first open the device. Then choose
  the connectors you wish to use. For each connector, get your desired mode and
  choose an available CRTC to use (in most situations, attaching a CRTC to a
  connector will automatically choose the preferred encoder). Once you have a
  suitable Connector, CRTC, and Mode, you can create a framebuffer for scanout.

  For more information, see the `drm-kms` man page.
  */

extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
pub mod resource;
pub mod mode;

use error::{Result, Error};
use self::resource::*;

use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Mutex;
use std::marker::PhantomData;

pub type ResourceId = u32;

#[derive(Debug)]
pub struct Device {
    file: File
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: File::from_raw_fd(fd)
        }
    }
}

impl IntoRawFd for Device {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

impl From<File> for Device {
    fn from(file: File) -> Device {
        Device {
            file: file
        }
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: file,
        };
        Ok(dev)
    }

    pub fn dumb_buffer(&self, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        DumbBuffer::create(self, width, height, bpp)
    }

    fn add_framebuffer(&self, width: u32, height: u32, pitch: u32, bpp: u32, depth: u32, handle: u32)
        -> Result<ffi::DrmModeAddFb> {
        ffi::DrmModeAddFb::new(self.file.as_raw_fd(), width, height, pitch, bpp, depth, handle)
    }
}

#[derive(Debug)]
pub struct MasterDevice<'a> {
    handle: RawFd,
//    connectors: Mutex<Vec<ConnectorId>>,
//    encoders: Mutex<Vec<EncoderId>>,
//    crtcs: Mutex<Vec<CrtcId>>,
    device: PhantomData<&'a Device>
}

impl<'a> AsRawFd for MasterDevice<'a> {
    fn as_raw_fd(&self) -> RawFd {
        self.handle
    }
}

impl<'a> FromRawFd for MasterDevice<'a> {
    unsafe fn from_raw_fd(fd: RawFd) -> MasterDevice<'a> {
        panic!("Not implemented")
    }
}

impl<'a> IntoRawFd for MasterDevice<'a> {
    fn into_raw_fd(self) -> RawFd {
        self.handle
    }
}

pub trait Buffer {
    fn size(&self) -> (u32, u32);
    fn depth(&self) -> u8;
    fn bpp(&self) -> u8;
    fn pitch(&self) -> u32;
    fn handle(&self) -> u32;
}

pub struct DumbBuffer<'a> {
    device: &'a AsRawFd,
    size: (u32, u32),
    depth: u8,
    bpp: u8,
    pitch: u32,
    handle: u32
}

impl<'a> DumbBuffer<'a> {
    fn create(device: &'a AsRawFd, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(device.as_raw_fd(), width, height, bpp));
        let buffer = DumbBuffer {
            device: device,
            size: (width, height),
            depth: 24,
            bpp: bpp,
            pitch: raw.raw.pitch,
            handle: raw.raw.handle
        };
        Ok(buffer)
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
