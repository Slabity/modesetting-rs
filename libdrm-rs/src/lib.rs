#![feature(concat_idents)]
#![feature(type_ascription)]

extern crate drm_sys;
extern crate libc;
#[macro_use]
extern crate error_chain;

#[macro_use]
mod util;
use util::*;

mod result;
pub mod auth;
pub mod mode;

use std::os::unix::io::{RawFd, AsRawFd};
use libc::{ioctl, c_void, c_char};
use drm_sys::*;
use std::io::Error as IoError;
use self::result::{Result, ErrorKind};

pub struct Card<T>(T) where T: AsRawFd;

impl<T> Card<T> where T: AsRawFd {

    pub fn set_atomic(&self, enable: bool) -> Result<()> {
        self.set_capability(DRM_CLIENT_CAP_ATOMIC as u64, enable as u64)
    }

    pub fn set_universal_planes(&self, enable: bool) -> Result<()> {
        self.set_capability(DRM_CLIENT_CAP_ATOMIC as u64, enable as u64)
    }

    pub fn version_info(&self) -> Result<DriverInfo> {
        let mut raw: drm_version_t = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_VERSION, &mut raw);
        ptr_buffers! {
            name = (&mut raw.name, raw.name_len as usize + 1, SM_SIZE, c_char);
            date = (&mut raw.date, raw.date_len as usize + 1, SM_SIZE, c_char);
            desc = (&mut raw.desc, raw.desc_len as usize + 1, SM_SIZE, c_char);
        };
        ioctl!(self, MACRO_DRM_IOCTL_VERSION, &mut raw);

        let version = DriverInfo {
            ver: (raw.version_major, raw.version_minor, raw.version_patchlevel),
            name: name,
            date: date,
            desc: desc
        };

        Ok(version)
    }


    fn set_capability(&self, capability: u64, value: u64) -> Result<()> {
        let mut raw: drm_set_client_cap = Default::default();
        raw.capability = capability;
        raw.value = value;
        ioctl!(self, MACRO_DRM_IOCTL_SET_CLIENT_CAP, &raw);
        Ok(())
    }
}

impl<T> AsRawFd for Card<T> where T: AsRawFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl<T> From<T> for Card<T> where T: AsRawFd {
    fn from(file: T) -> Self {
        Card(file)
    }
}

#[derive(Debug)]
pub struct DriverInfo {
    ver: (i32, i32, i32),
    name: Buffer<c_char>,
    date: Buffer<c_char>,
    desc: Buffer<c_char>
}


