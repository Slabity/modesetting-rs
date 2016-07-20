use super::super::Device;
use super::Manager;
use super::super::error::Result;
use super::super::ffi;
use super::ResourceId;

use std::vec::IntoIter;

pub type FramebufferId = ResourceId;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    device: Device,
    id: FramebufferId,
    size: (u32, u32),
    pitch: u32,
    bpp: u32,
    depth: u32,
    handle: u32
}

impl<'a> From<(&'a Device, &'a ffi::DrmModeGetFb)> for Framebuffer {
    fn from(dev_raw: (&Device, &ffi::DrmModeGetFb)) -> Framebuffer {
        let (dev, raw) = dev_raw;
        Framebuffer {
            device: (*dev).clone(),
            id: raw.raw.fb_id,
            size: (raw.raw.width, raw.raw.height),
            pitch: raw.raw.pitch,
            bpp: raw.raw.bpp,
            depth: raw.raw.depth,
            handle: raw.raw.handle
        }
    }
}

#[derive(Clone)]
pub struct Framebuffers<'a> {
    manager: &'a Manager<'a>,
    fbs: IntoIter<FramebufferId>
}

impl<'a> Iterator for Framebuffers<'a> {
    type Item = Result<Framebuffer>;
    fn next(&mut self) -> Option<Result<Framebuffer>> {
        match self.fbs.next() {
            Some(id) => Some(self.manager.framebuffer(id)),
            None => None
        }
    }
}

impl<'a> From<(&'a Manager<'a>, Vec<FramebufferId>)> for Framebuffers<'a> {
    fn from(man_vec: (&'a Manager<'a>, Vec<FramebufferId>)) -> Framebuffers<'a> {
        let (man, vec) = man_vec;
        Framebuffers {
            manager: man,
            fbs: vec.into_iter()
        }
    }
}

