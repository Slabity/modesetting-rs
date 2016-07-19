use super::super::Device;
use super::super::mode::Mode;
use super::super::error::Result;
use super::super::ffi;
use super::{ResourceId, EncoderId};
use super::{Encoder, Encoders};

use std::mem::transmute;
use std::vec::IntoIter;

pub type ConnectorId = ResourceId;

#[derive(Debug, Clone)]
pub struct Connector {
    device: Device,
    id: ConnectorId,
    interface: ConnectorInterface,
    state: ConnectorState,
    curr_encoder: EncoderId,
    encoders: Vec<EncoderId>,
    modes: Vec<Mode>,
    size: (u32, u32)
}

impl Connector {
    pub fn interface(&self) -> ConnectorInterface {
        self.interface
    }

    pub fn state(&self) -> ConnectorState {
        self.state
    }

    pub fn current_encoder(&self) -> Result<Encoder> {
        self.device.encoder(self.curr_encoder)
    }

    pub fn possible_encoders(&self) -> Encoders {
        Encoders::from((&self.device, &self.encoders))
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeGetConnector)> for Connector {
    fn from(dev_raw: (&Device, &ffi::DrmModeGetConnector)) -> Connector {
        let (dev, raw) = dev_raw;
        Connector {
            device: (*dev).clone(),
            id: raw.raw.connector_id,
            interface: ConnectorInterface::from(raw.raw.connector_type),
            state: ConnectorState::from(raw.raw.connection),
            curr_encoder: raw.raw.encoder_id,
            encoders: raw.encoders.clone(),
            modes: raw.modes.iter().map(| raw | Mode::from(*raw)).collect(),
            size: (raw.raw.mm_width, raw.raw.mm_height)
        }
    }
}

#[derive(Clone)]
pub struct Connectors {
    device: Device,
    connectors: IntoIter<ConnectorId>
}

impl Iterator for Connectors {
    type Item = Result<Connector>;
    fn next(&mut self) -> Option<Result<Connector>> {
        match self.connectors.next() {
            Some(id) => Some(self.device.connector(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Device, &'a Vec<ConnectorId>)> for Connectors {
    fn from(dev_vec: (&Device, &Vec<ConnectorId>)) -> Connectors {
        let (dev, vec) = dev_vec;
        Connectors {
            device: dev.clone(),
            connectors: vec.clone().into_iter()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectorInterface {
    Unknown = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Unknown as isize,
    VGA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_VGA as isize,
    DVII = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVII as isize,
    DVID = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVID as isize,
    DVIA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVIA as isize,
    Composite = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Composite as isize,
    SVideo = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_SVIDEO as isize,
    LVDS = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_LVDS as isize,
    Component = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Component as isize,
    NinePinDIN = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_9PinDIN as isize,
    DisplayPort = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DisplayPort as isize,
    HDMIA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_HDMIA as isize,
    HDMIB = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_HDMIB as isize,
    TV = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_TV as isize,
    EDP = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_eDP as isize,
    Virtual = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_VIRTUAL as isize,
    DSI = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DSI as isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectorState {
    Connected = ffi::Connection::FFI_DRM_MODE_CONNECTED as isize,
    Disconnected = ffi::Connection::FFI_DRM_MODE_DISCONNECTED as isize,
    Unknown = ffi::Connection::FFI_DRM_MODE_UNKNOWN as isize
}

impl From<u32> for ConnectorInterface {
    fn from(ty: u32) -> ConnectorInterface {
        unsafe { transmute(ty as u8) }
    }
}

impl From<u32> for ConnectorState {
    fn from(ty: u32) -> ConnectorState {
        unsafe { transmute(ty as u8) }
    }
}
