use super::ffi;
use super::error::Result;
use super::Device;

use std::sync::Arc;
use std::mem::transmute;
use std::iter::Iterator;
use std::vec::IntoIter;
use std::ffi::CStr;

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
    connection: Connection,
    encoder: EncoderId,
    encoders: Vec<EncoderId>,
    modes: Vec<Mode>,
    properties: Vec<PropertyId>,
    size: (u32, u32)
}

#[derive(Debug)]
pub struct Encoder {
    device: Device,
    id: EncoderId,
    crtc: CrtcId
}

#[derive(Debug)]
pub struct Crtc {
    device: Device,
    id: CrtcId,
    framebuffer: Option<FramebufferId>,
    position: (u32, u32),
    mode: Option<Mode>
}

#[derive(Debug)]
pub struct Framebuffer {
    device: Device,
    id: FramebufferId,
    size: (u32, u32)
}

#[derive(Debug)]
pub struct Property {
    device: Device,
    id: PropertyId,
    name: String,
}

#[derive(Debug)]
pub struct Blob {
    device: Device,
    id: BlobId,
    data: Vec<u8>
}

impl Connector {
    pub fn id(&self) -> ConnectorId {
        self.id
    }

    pub fn connector_type(&self) -> ConnectorType {
        self.con_type
    }

    pub fn connection(&self) -> Connection {
        self.connection
    }

    pub fn current_encoder(&self) -> Result<Encoder> {
        self.device.encoder(self.encoder)
    }

    pub fn encoders(&self) -> EncoderIterator {
        EncoderIterator {
            device: self.device.clone(),
            encoders: self.encoders.clone().into_iter()
        }
    }

    pub fn properties(&self) -> PropertyIterator {
        PropertyIterator {
            device: self.device.clone(),
            properties: self.properties.clone().into_iter()
        }
    }
}

impl Encoder {
    pub fn id(&self) -> EncoderId {
        self.id
    }

    pub fn crtc(&self) -> Result<Crtc> {
        self.device.crtc(self.crtc)
    }
}

impl Crtc {
    pub fn id(&self) -> CrtcId {
        self.id
    }

    pub fn mode(&self) -> Option<Mode> {
        self.mode.clone()
    }
}

impl Property {
    pub fn id(&self) -> PropertyId {
        self.id
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Connection {
    Connected = ffi::Connection::FFI_DRM_MODE_CONNECTED as isize,
    Disconnected = ffi::Connection::FFI_DRM_MODE_DISCONNECTED as isize,
    Unknown = ffi::Connection::FFI_DRM_MODE_UNKNOWN as isize
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mode {
    name: String,
    clock: u32,
    display: (u16, u16),
    hsync: (u16, u16),
    vsync: (u16, u16),
    hskew: u16,
    vscan: u16,
    htotal: u16,
    vtotal: u16,
    vrefresh: u32,
    flags: u32,
    mode_type: u32,
}

impl From<u32> for ConnectorType {
    fn from(ty: u32) -> ConnectorType {
        unsafe { transmute(ty as u8) }
    }
}

impl From<u32> for Connection {
    fn from(ty: u32) -> Connection {
        unsafe { transmute(ty as u8) }
    }
}

impl From<ffi::drm_mode_modeinfo> for Mode {
    fn from(raw: ffi::drm_mode_modeinfo) -> Mode {
        let name = unsafe {
            CStr::from_ptr(raw.name.as_ptr()).to_str().unwrap()
        };

        Mode {
            name: name.to_string(),
            clock: raw.clock,
            display: (raw.hdisplay, raw.vdisplay),
            hsync: (raw.hsync_start, raw.hsync_end),
            vsync: (raw.vsync_start, raw.vsync_end),
            hskew: raw.hskew,
            vscan: raw.vscan,
            htotal: raw.htotal,
            vtotal: raw.vtotal,
            vrefresh: raw.vrefresh,
            flags: raw.flags,
            mode_type: raw.type_
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ConnectorId(pub u32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EncoderId(pub u32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CrtcId(pub u32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FramebufferId(pub u32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PropertyId(pub u32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BlobId(pub u32);

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

pub struct PropertyIterator {
    device: Device,
    properties: IntoIter<PropertyId>
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
            Some(id) => Some(self.device.framebuffer(id)),
            None => None
        }
    }
}

impl Iterator for PropertyIterator {
    type Item = Result<Property>;
    fn next(&mut self) -> Option<Result<Property>> {
        match self.properties.next() {
            Some(id) => Some(self.device.property(id)),
            None => None
        }
    }
}

pub trait IsBuffer {
    fn size(&self) -> (u32, u32);
    fn stride(&self) -> u32;
    fn bpp(&self) -> u32;
    fn depth(&self) -> u32;
    fn handle(&self) -> u32;
}

