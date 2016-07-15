use super::ffi;
use super::error::{Error, Result};

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;
use std::mem::transmute;
use std::iter::Iterator;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
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
                device: (*self).clone(),
                connectors: transmute(res.connectors),
                encoders: transmute(res.encoders),
                crtcs: transmute(res.crtcs),
                framebuffers: transmute(res.framebuffers)
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectorId(u32);
#[derive(Debug, Clone)]
pub struct EncoderId(u32);
#[derive(Debug, Clone)]
pub struct CrtcId(u32);
#[derive(Debug, Clone)]
pub struct FramebufferId(u32);

pub struct ConnectorIterator {
    device: Device,
    connectors: IntoIter<ConnectorId>
}

pub struct EncoderIterator {
    device: Device,
    encoders: IntoIter<EncoderId>
}

pub struct CrtcIterator {
    device: Device,
    crtcs: IntoIter<CrtcId>
}

pub struct FramebufferIterator {
    device: Device,
    framebuffers: IntoIter<FramebufferId>
}

impl Iterator for ConnectorIterator {
    type Item = Result<Connector>;
    fn next(&mut self) -> Option<Result<Connector>> {
        // Get the raw id of the connector
        let raw_id = match self.connectors.next() {
            Some(id) => id.0,
            None => return None
        };

        // Get the raw results
        let con_res = ffi::drm_ioctl_mode_get_connector(self.device.as_raw_fd(), raw_id);
        let ffi_con = match con_res {
            Err(e) => return Some(Err(e)),
            Ok(c) => c
        };

        unsafe {
            Some(Ok(Connector {
                encoders: transmute(ffi_con.encoders),
                size: ffi_con.size
            }))
        }
    }
}

impl Iterator for EncoderIterator {
    type Item = EncoderId;
    fn next(&mut self) -> Option<EncoderId> {
        self.encoders.next()
    }
}

impl Iterator for CrtcIterator {
    type Item = CrtcId;
    fn next(&mut self) -> Option<CrtcId> {
        self.crtcs.next()
    }
}

impl Iterator for FramebufferIterator {
    type Item = FramebufferId;
    fn next(&mut self) -> Option<FramebufferId> {
        self.framebuffers.next()
    }
}

#[derive(Debug)]
pub struct Resources {
    device: Device,
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
    framebuffers: Vec<FramebufferId>
}

impl Resources {
    pub fn connectors(&self) -> ConnectorIterator {
        ConnectorIterator {
            device: self.device.clone(),
            connectors: self.connectors.clone().into_iter()
        }
    }

    pub fn encoders(&self) -> EncoderIterator {
        EncoderIterator {
            device: self.device.clone(),
            encoders: self.encoders.clone().into_iter()
        }
    }

    pub fn crtcs(&self) -> CrtcIterator {
        CrtcIterator {
            device: self.device.clone(),
            crtcs: self.crtcs.clone().into_iter()
        }
    }

    pub fn framebuffers(&self) -> FramebufferIterator {
        FramebufferIterator {
            device: self.device.clone(),
            framebuffers: self.framebuffers.clone().into_iter()
        }
    }
}

#[derive(Debug)]
pub struct Connector {
    encoders: Vec<EncoderId>,
    size: (u32, u32)
}

