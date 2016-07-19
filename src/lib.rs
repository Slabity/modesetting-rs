extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
pub mod resources;
pub mod mode;

use error::Result;
pub use self::resources::*;

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Device {
    file: Arc<File>,
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: Arc::new(File::from_raw_fd(fd)),
        }
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: Arc::new(file),
        };
        Ok(dev)
    }

    pub fn resources(&self) -> Result<Resources> {
        let raw = try!(ffi::DrmModeCardRes::new(self.as_raw_fd()));
        Ok(Resources::from((self, &raw)))
    }

    fn connector(&self, id: ConnectorId) -> Result<Connector> {
        let raw = try!(ffi::DrmModeGetConnector::new(self.as_raw_fd(), id));
        Ok(Connector::from((self, &raw)))
    }

    fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        let raw = try!(ffi::DrmModeGetEncoder::new(self.as_raw_fd(), id));
        Ok(Encoder::from((self, &raw)))
    }

    fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        let raw = try!(ffi::DrmModeGetCrtc::new(self.as_raw_fd(), id));
        Ok(Crtc::from((self, &raw)))
    }
}


