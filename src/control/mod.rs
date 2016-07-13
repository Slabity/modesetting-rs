use super::ffi;
use super::error::{Error, Result};

use libc::ioctl;

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;

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

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: file
        };
        Ok(dev)
    }

    pub fn resources(&self) {
        let res = ffi::drm_ioctl_mode_get_resources(self.as_raw_fd());

        println!("{:#?}", res);
    }
}

pub struct ConnectorId(u32);
pub struct EncoderId(u32);
pub struct CrtcId(u32);
pub struct FramebufferId(u32);

pub struct Resources {
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
    framebuffers: Vec<FramebufferId>
}

