mod connector;
mod encoder;
mod crtc;

pub use self::connector::*;
pub use self::encoder::*;
pub use self::crtc::*;

use super::Device;
use super::ffi;

pub type ResourceId = u32;

#[derive(Debug)]
pub struct Resources {
    device: Device,
    connectors: Vec<ConnectorId>,
    encoders: Vec<EncoderId>,
    crtcs: Vec<CrtcId>,
}

impl Resources {
    pub fn connectors(&self) -> Connectors {
        Connectors::from((&self.device, &self.connectors))
    }

    pub fn encoders(&self) -> Encoders {
        Encoders::from((&self.device, &self.crtcs))
    }

    pub fn crtcs(&self) -> Crtcs {
        Crtcs::from((&self.device, &self.crtcs))
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeCardRes)> for Resources {
    fn from(dev_raw: (&Device, &ffi::DrmModeCardRes)) -> Resources {
        let (dev, raw) = dev_raw;
        Resources {
            device: (*dev).clone(),
            connectors: (*raw).connectors.clone(),
            encoders: (*raw).encoders.clone(),
            crtcs: (*raw).crtcs.clone()
        }
    }
}
