use super::super::Device;
use super::super::error::Result;
use super::super::mode::Mode;
use super::super::ffi;
use super::ResourceId;

use std::vec::IntoIter;

pub type CrtcId = ResourceId;

#[derive(Debug)]
pub struct Crtc {
    device: Device,
    id: CrtcId,
    //framebuffer: Option<FramebufferId>,
    position: (u32, u32),
    mode: Option<Mode>
}

impl Crtc {
    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    pub fn mode(&self) -> Option<Mode> {
        self.mode.clone()
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeGetCrtc)> for Crtc {
    fn from(dev_raw: (&Device, &ffi::DrmModeGetCrtc)) -> Crtc {
        let (dev, raw) = dev_raw;
        Crtc {
            device: (*dev).clone(),
            id: raw.raw.crtc_id,
            /*framebuffer: match raw.raw.fb_id {
                0 => None,
                _ => Some(raw.raw.fb_id)
            },*/
            position: (raw.raw.x, raw.raw.y),
            mode: match raw.raw.mode.clock {
                0 => None,
                _ => Some(Mode::from(raw.raw.mode))
            }
        }
    }
}

#[derive(Clone)]
pub struct Crtcs {
    device: Device,
    crtcs: IntoIter<CrtcId>
}

impl<'a> From<(&'a Device, &'a Vec<CrtcId>)> for Crtcs {
    fn from(dev_vec: (&Device, &Vec<CrtcId>)) -> Crtcs {
        let (dev, vec) = dev_vec;
        Crtcs {
            device: dev.clone(),
            crtcs: vec.clone().into_iter()
        }
    }
}

impl Iterator for Crtcs {
    type Item = Result<Crtc>;
    fn next(&mut self) -> Option<Result<Crtc>> {
        match self.crtcs.next() {
            Some(id) => Some(self.device.crtc(id)),
            None => None
        }
    }
}

