mod connector;
mod encoder;
mod crtc;
mod framebuffer;

pub use self::connector::*;
pub use self::encoder::*;
pub use self::crtc::*;
pub use self::framebuffer::*;

use super::Device;
use super::error::Result;
use super::ffi;

pub type ResourceId = u32;

#[derive(Debug)]
pub struct Manager<'a> {
    device: &'a Device,
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
    framebuffers: Vec<FramebufferId>
}

impl<'a> Manager<'a> {
    pub fn connectors(&self) -> Connectors {
        Connectors::from((self, self.connectors.clone()))
    }

    pub fn encoders(&self) -> Encoders {
        Encoders::from((self, self.encoders.clone()))
    }

    pub fn crtcs(&self) -> Crtcs {
        Crtcs::from((self, self.crtcs.clone()))
    }

    pub fn framebuffers(&self) -> Framebuffers {
        Framebuffers::from((self, self.framebuffers.clone()))
    }

    fn connector(&self, id: ConnectorId) -> Result<Connector> {
        self.device.connector(id)
    }

    fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        self.device.encoder(id)
    }

    fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        self.device.crtc(id)
    }

    fn framebuffer(&self, id: FramebufferId) -> Result<Framebuffer> {
        self.device.framebuffer(id)
    }
}

impl<'a, 'b> From<(&'a Device, &'b ffi::DrmModeCardRes)> for Manager<'a> {
    fn from(dev_raw: (&'a Device, &ffi::DrmModeCardRes)) -> Manager<'a> {
        let (dev, raw) = dev_raw;
        Manager {
            device: dev,
            connectors: (*raw).connectors.clone(),
            encoders: (*raw).encoders.clone(),
            crtcs: (*raw).crtcs.clone(),
            framebuffers: (*raw).framebuffers.clone()
        }
    }
}
