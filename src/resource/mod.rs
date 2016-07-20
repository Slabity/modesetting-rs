mod connector;
mod encoder;
mod crtc;
mod framebuffer;

pub use self::connector::*;
pub use self::encoder::*;
pub use self::crtc::*;
pub use self::framebuffer::*;

use super::Device;
use super::ffi;

pub type ResourceId = u32;

#[derive(Debug)]
enum ResourceWrapper<T> {
    Unloaded(ResourceId),
    Loaded(T)
}

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
        Connectors::from((self.device, &self.connectors))
    }

    pub fn encoders(&self) -> Encoders {
        Encoders::from((self.device, &self.encoders))
    }

    pub fn crtcs(&self) -> Crtcs {
        Crtcs::from((self.device, &self.crtcs))
    }

    pub fn framebuffers(&self) -> Framebuffers {
        Framebuffers::from((self.device, &self.framebuffers))
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
