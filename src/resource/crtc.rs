use super::super::Device;
use super::Manager;
use super::super::error::Result;
use super::super::mode::Mode;
use super::super::ffi;
use super::ResourceId;
use super::Framebuffer;

use std::vec::IntoIter;

pub type CrtcId = ResourceId;

#[derive(Debug)]
pub struct Crtc {
    device: Device,
    id: CrtcId,
    framebuffer: Option<u32>,
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
    pub fn framebuffer(&self) -> Option<Result<Framebuffer>> {
        match self.framebuffer {
            Some(id) => Some(self.device.framebuffer(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeGetCrtc)> for Crtc {
    fn from(dev_raw: (&Device, &ffi::DrmModeGetCrtc)) -> Crtc {
        let (dev, raw) = dev_raw;
        Crtc {
            device: (*dev).clone(),
            id: raw.raw.crtc_id,
            framebuffer: match raw.raw.fb_id {
                0 => None,
                _ => Some(raw.raw.fb_id)
            },
            position: (raw.raw.x, raw.raw.y),
            mode: match raw.raw.mode.clock {
                0 => None,
                _ => Some(Mode::from(raw.raw.mode))
            }
        }
    }
}

#[derive(Clone)]
pub struct Crtcs<'a> {
    manager: &'a Manager<'a>,
    crtcs: IntoIter<CrtcId>
}

impl<'a> Iterator for Crtcs<'a> {
    type Item = Result<Crtc>;
    fn next(&mut self) -> Option<Result<Crtc>> {
        match self.crtcs.next() {
            Some(id) => Some(self.manager.crtc(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Manager<'a>, Vec<CrtcId>)> for Crtcs<'a> {
    fn from(man_vec: (&'a Manager<'a>, Vec<CrtcId>)) -> Crtcs<'a> {
        let (man, vec) = man_vec;
        Crtcs {
            manager: man,
            crtcs: vec.into_iter()
        }
    }
}

