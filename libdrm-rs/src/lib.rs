#![feature(concat_idents)]
#![feature(type_ascription)]

extern crate drm_sys;
extern crate libc;
extern crate smallvec;
#[macro_use]
extern crate error_chain;

#[macro_use]
mod util;
use util::*;

pub mod result;
pub mod control;

use std::os::unix::io::{RawFd, AsRawFd};
use libc::{ioctl, c_void, c_char};
use drm_sys::*;
use smallvec::SmallVec;
use std::io::Error as IoError;
use self::result::{Result, ErrorKind};

#[derive(Debug)]
pub struct DriverInfo {
    ver: (i32, i32, i32),
    name: Array<c_char>,
    date: Array<c_char>,
    desc: Array<c_char>
}

#[derive(Debug)]
/// A token unique to the process that determines who opened the device.
///
/// This token can be sent to another process that acts as the DRM Master and
/// then authenticated to give extra privileges.
pub struct AuthToken(u32);

#[derive(Debug)]
/// Capabilities that the process understands.
///
/// These can be used to tell the DRM device what capabilities the process can
/// use.
pub enum ClientCapability {
    Stereo3D = DRM_CLIENT_CAP_STEREO_3D as isize,
    UniversalPlanes = DRM_CLIENT_CAP_UNIVERSAL_PLANES as isize,
    Atomic = DRM_CLIENT_CAP_ATOMIC as isize
}

/// A trait for all DRM devices.
pub trait DRMDevice : AsRawFd {
    /// Generates and returns a magic token unique to the current process. This
    /// token can be used to authenticate with the DRM Master.
    fn magic(&self) -> Result<AuthToken> {
        let mut raw: drm_auth_t = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_GET_MAGIC, &raw);
        Ok(AuthToken(raw.magic))
    }

    // Returns stat information about the device.
    // TODO Finish this when we get data from stats.
    fn stats(&self) -> Result<()> {
        unimplemented!()
    }

    /// Tells the DRM device whether we understand or do not understand a
    /// particular capability. Some features, such as atomic modesetting,
    /// require informing the device that the process can use such features
    /// before will expose them.
    fn set_client_cap(&self, cap: ClientCapability, set: bool) -> Result<()> {
        let mut raw: drm_set_client_cap = Default::default();
        raw.capability = cap as u64;
        raw.value = set as u64;
        ioctl!(self, MACRO_DRM_IOCTL_SET_CLIENT_CAP, &raw);
        Ok(())
    }

    // Waits for the next VBlank to appear.
    // TODO Implement with EINTR
    fn wait_vblan(&self) -> () {
        unimplemented!()
    }
}

/// A DRM device providing an unprivileged DRM functionality.
#[derive(Debug)]
pub struct Device<T>(T) where T: AsRawFd;

impl<T> AsRawFd for Device<T> where T: AsRawFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl<T> From<T> for Device<T> where T: AsRawFd {
    fn from(file: T) -> Self {
        Device(file)
    }
}

// A DRM device providing DRM Master functionality.
pub struct Master<T>(T) where T: AsRawFd;

impl<T> AsRawFd for Master<T> where T: AsRawFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl<T> From<T> for Master<T> where T: AsRawFd {
    fn from(file: T) -> Self {
        Master(file)
    }
}

impl<T> DRMDevice for Device<T> where T: AsRawFd {}
impl<T> DRMDevice for Master<T> where T: AsRawFd {}

impl<T> control::Control for Device<T> where T: AsRawFd {}
impl<T> control::Control for Master<T> where T: AsRawFd {}
