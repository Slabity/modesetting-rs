extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
pub mod resources;

use error::Result;
pub use resources::*;

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;
use std::ffi::CStr;
use std::mem::transmute;

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
        let res = try!(ffi::DrmModeCardRes::new(self.as_raw_fd()));

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

    pub fn connector(&self, id: ConnectorId) -> Result<Connector> {
        let raw = try!(ffi::DrmModeGetConnector::new(self.as_raw_fd(), id.0));
        let con = Connector {
            device: self.clone(),
            id: id,
            con_type: ConnectorType::from(raw.raw.connector_type),
            connection: Connection::from(raw.raw.connection),
            encoder: EncoderId(raw.raw.encoder_id),
            encoders: unsafe { transmute(raw.encoders) },
            modes: raw.modes.iter().map(| raw | Mode::from(*raw)).collect(),
            properties: unsafe { transmute(raw.properties) },
            size: (raw.raw.mm_width, raw.raw.mm_height)
        };

        Ok(con)
    }

    pub fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        let raw = try!(ffi::DrmModeGetEncoder::new(self.as_raw_fd(), id.0));
        let enc = Encoder {
            device: self.clone(),
            id: id,
            crtc: CrtcId(raw.raw.crtc_id)
        };

        Ok(enc)
    }

    pub fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        let raw = try!(ffi::DrmModeGetCrtc::new(self.as_raw_fd(), id.0));
        let crtc = Crtc {
            device: self.clone(),
            id: id,
            framebuffer: match raw.raw.fb_id {
                0 => None,
                _ => Some(FramebufferId(raw.raw.fb_id))
            },
            position: (raw.raw.x, raw.raw.y),
            mode: match raw.raw.mode.clock {
                0 => None,
                _ => Some(Mode::from(raw.raw.mode))
            }
        };

        Ok(crtc)
    }

    pub fn framebuffer(&self, id: FramebufferId) -> Result<Framebuffer> {
        let raw = try!(ffi::DrmModeGetFb::new(self.as_raw_fd(), id.0));
        let fb = Framebuffer {
            device: self.clone(),
            id: id,
            size: (raw.raw.width, raw.raw.height)
        };

        Ok(fb)
    }

    pub fn property(&self, id: PropertyId) -> Result<Property> {
        let raw = try!(ffi::DrmModeGetProperty::new(self.as_raw_fd(), id.0));
        let name = unsafe {
            CStr::from_ptr(raw.raw.name.as_ptr()).to_str().unwrap()
        };
        let property = Property {
            device: self.clone(),
            id: id,
            name: name.to_string()
        };

        Ok(property)
    }

    pub fn blob(&self, id: BlobId) -> Result<Blob> {
        let raw = try!(ffi::DrmModeGetBlob::new(self.as_raw_fd(), id.0));
        let blob = Blob {
            device: self.clone(),
            id: id,
            data: raw.data
        };

        Ok(blob)
    }
}


