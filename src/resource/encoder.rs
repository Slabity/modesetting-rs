use super::super::Device;
use super::super::error::Result;
use super::super::ffi;
use super::{ResourceId, CrtcId};
use super::Crtc;

use std::vec::IntoIter;

pub type EncoderId = ResourceId;

#[derive(Debug)]
pub struct Encoder {
    device: Device,
    id: EncoderId,
    crtc: CrtcId
}

impl Encoder {
    pub fn current_crtc(&self) -> Result<Crtc> {
        self.device.crtc(self.crtc)
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeGetEncoder)> for Encoder {
    fn from(dev_raw: (&Device, &ffi::DrmModeGetEncoder)) -> Encoder {
        let (dev, raw) = dev_raw;
        Encoder {
            device: (*dev).clone(),
            id: raw.raw.encoder_id,
            crtc: raw.raw.crtc_id
        }
    }
}

#[derive(Clone)]
pub struct Encoders {
    device: Device,
    encoders: IntoIter<EncoderId>
}

impl Iterator for Encoders {
    type Item = Result<Encoder>;
    fn next(&mut self) -> Option<Result<Encoder>> {
        match self.encoders.next() {
            Some(id) => Some(self.device.encoder(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Device, &'a Vec<EncoderId>)> for Encoders {
    fn from(dev_vec: (&Device, &Vec<EncoderId>)) -> Encoders {
        let (dev, vec) = dev_vec;
        Encoders {
            device: dev.clone(),
            encoders: vec.clone().into_iter()
        }
    }
}

