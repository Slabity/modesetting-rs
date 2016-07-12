use std::os::unix::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;

use super::error::{Error, Result};

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
}

