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
pub struct Manager {
    device: Device,
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
    framebuffers: Vec<FramebufferId>
}

impl Manager {
    pub fn connectors(&self) -> Connectors {
        Connectors::from((&self.device, &self.connectors))
    }

    pub fn encoders(&self) -> Encoders {
        Encoders::from((&self.device, &self.encoders))
    }

    pub fn crtcs(&self) -> Crtcs {
        Crtcs::from((&self.device, &self.crtcs))
    }

    pub fn framebuffers(&self) -> Framebuffers {
        Framebuffers::from((&self.device, &self.framebuffers))
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeCardRes)> for Manager {
    fn from(dev_raw: (&Device, &ffi::DrmModeCardRes)) -> Manager {
        let (dev, raw) = dev_raw;
        Manager {
            device: (*dev).clone(),
            connectors: (*raw).connectors.clone(),
            encoders: (*raw).encoders.clone(),
            crtcs: (*raw).crtcs.clone(),
            framebuffers: (*raw).framebuffers.clone()
        }
    }
}
