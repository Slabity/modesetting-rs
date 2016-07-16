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
            encoders: unsafe { transmute(raw.encoders) },
            size: (raw.raw.mm_width, raw.raw.mm_height)
        };

        Ok(con)
    }

    pub fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        let raw = try!(ffi::DrmModeGetEncoder::new(self.as_raw_fd(), id.0));
        let enc = Encoder {
            device: self.clone(),
            id: id
        };

        Ok(enc)
    }

    pub fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        let raw = try!(ffi::DrmModeGetCrtc::new(self.as_raw_fd(), id.0));
        let crtc = Crtc {
            device: self.clone(),
            id: id
        };

        Ok(crtc)
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
    device: Device,
    id: ConnectorId,
    con_type: ConnectorType,
    encoders: Vec<EncoderId>,
    size: (u32, u32),
}

#[derive(Debug)]
pub struct Encoder {
    device: Device,
    id: EncoderId,
}

#[derive(Debug)]
pub struct Crtc {
    device: Device,
    id: CrtcId
}

#[derive(Debug)]
pub struct Framebuffer {
    device: Device,
    id: FramebufferId
}

impl Connector {
    pub fn encoders(&self) -> EncoderIterator {
        EncoderIterator {
            device: self.device.clone(),
            encoders: self.encoders.clone().into_iter()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConnectorType {
    Unknown = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_Unknown as isize,
    VGA = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_VGA as isize,
    DVII = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_DVII as isize,
    DVID = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_DVID as isize,
    DVIA = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_DVIA as isize,
    Composite = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_Composite as isize,
    SVideo = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_SVIDEO as isize,
    LVDS = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_LVDS as isize,
    Component = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_Component as isize,
    NinePinDIN = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_9PinDIN as isize,
    DisplayPort = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_DisplayPort as isize,
    HDMIA = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_HDMIA as isize,
    HDMIB = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_HDMIB as isize,
    TV = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_TV as isize,
    EDP = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_eDP as isize,
    Virtual = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_VIRTUAL as isize,
    DSI = ffi::ConnectorType::FFI_DRM_MODE_CONNECTOR_DSI as isize,
}

impl From<u32> for ConnectorType {
    fn from(ty: u32) -> ConnectorType {
        unsafe { transmute(ty as u8) }
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
        match self.connectors.next() {
            Some(id) => Some(self.device.connector(id)),
            None => None
        }
    }
}

impl Iterator for EncoderIterator {
    type Item = Result<Encoder>;
    fn next(&mut self) -> Option<Result<Encoder>> {
        match self.encoders.next() {
            Some(id) => Some(self.device.encoder(id)),
            None => None
        }
    }
}

impl Iterator for CrtcIterator {
    type Item = Result<Crtc>;
    fn next(&mut self) -> Option<Result<Crtc>> {
        match self.crtcs.next() {
            Some(id) => Some(self.device.crtc(id)),
            None => None
        }
    }
}

impl Iterator for FramebufferIterator {
    type Item = Result<Framebuffer>;
    fn next(&mut self) -> Option<Result<Framebuffer>> {
        match self.framebuffers.next() {
            Some(id) => Some(Ok(Framebuffer {
                device: self.device.clone(),
                id: id,
            })),
            None => None
        }
    }
}


