use super::ffi;
use super::error::{Error, Result};

use libc::ioctl;

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;
use std::mem::transmute;

#[derive(Debug)]
pub struct Device {
    file: Arc<File>
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}
impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: Arc::new(File::from_raw_fd(fd))
        }
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: Arc::new(file)
        };
        Ok(dev)
    }

    pub fn resources(&self) -> Result<Resources> {
        let res = try!(ffi::drm_ioctl_mode_get_resources(self.as_raw_fd()));

        unsafe {
            Ok(Resources {
                device: self.file.clone(),
                connectors: transmute(res.connectors),
                encoders: transmute(res.encoders),
                crtcs: transmute(res.crtcs),
                framebuffers: transmute(res.framebuffers)
            })
        }
    }
}

#[derive(Debug)]
pub struct ConnectorId(u32);
#[derive(Debug)]
pub struct EncoderId(u32);
#[derive(Debug)]
pub struct CrtcId(u32);
#[derive(Debug)]
pub struct FramebufferId(u32);

#[derive(Debug)]
pub struct Resources {
    device: Arc<File>,
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
    framebuffers: Vec<FramebufferId>
}

