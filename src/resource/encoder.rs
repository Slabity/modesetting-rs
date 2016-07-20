use super::super::Device;
use super::Manager;
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
pub struct Encoders<'a> {
    manager: &'a Manager<'a>,
    encoders: IntoIter<EncoderId>
}

impl<'a> Iterator for Encoders<'a> {
    type Item = Result<Encoder>;
    fn next(&mut self) -> Option<Result<Encoder>> {
        match self.encoders.next() {
            Some(id) => Some(self.manager.encoder(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Manager<'a>, Vec<EncoderId>)> for Encoders<'a> {
    fn from(man_vec: (&'a Manager<'a>, Vec<EncoderId>)) -> Encoders<'a> {
        let (man, vec) = man_vec;
        Encoders {
            manager: man,
            encoders: vec.into_iter()
        }
    }
}

