use super::Manager;
use super::super::error::Result;
use super::super::ffi;
use super::ResourceId;

use std::vec::IntoIter;

pub type FramebufferId = ResourceId;

#[derive(Debug)]
pub struct Framebuffer<'a> {
    manager: &'a Manager<'a>,
    id: FramebufferId,
    size: (u32, u32),
    pitch: u32,
    bpp: u32,
    depth: u32,
    handle: u32
}

impl<'a> Framebuffer<'a> {
    fn from_raw(man: &'a Manager, raw: ffi::DrmModeGetFb) -> Framebuffer<'a> {
        Framebuffer {
            manager: man,
            id: raw.raw.fb_id,
            size: (raw.raw.width, raw.raw.height),
            pitch: raw.raw.pitch,
            bpp: raw.raw.bpp,
            depth: raw.raw.depth,
            handle: raw.raw.handle
        }
    }
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        self.manager.unload_framebuffer(self.id);
    }
}

#[derive(Clone)]
pub struct Framebuffers<'a> {
    manager: &'a Manager<'a>,
    framebuffers: IntoIter<FramebufferId>
}

impl<'a> Iterator for Framebuffers<'a> {
    type Item = Result<Framebuffer<'a>>;
    fn next(&mut self) -> Option<Result<Framebuffer<'a>>> {
        let raw = match self.framebuffers.next() {
            Some(id) => self.manager.load_framebuffer(id),
            None => return None
        };

        match raw {
            Ok(r) => Some(Ok(Framebuffer::from_raw(self.manager, r))),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Framebuffers<'a> {
    pub fn new(man: &'a Manager<'a>, iter: IntoIter<FramebufferId>) -> Framebuffers<'a> {
        Framebuffers {
            manager: man,
            framebuffers: iter
        }
    }
}

