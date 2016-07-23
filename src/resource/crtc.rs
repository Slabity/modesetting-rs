use super::Manager;
use super::super::error::Result;
use super::super::mode::Mode;
use super::super::ffi;
use super::ResourceId;

use std::vec::IntoIter;

pub type CrtcId = ResourceId;

#[derive(Debug)]
pub struct Crtc<'a> {
    manager: &'a Manager<'a>,
    id: CrtcId,
    position: (u32, u32),
    mode: Option<Mode>
}

impl<'a> Crtc<'a> {
    fn from_raw(man: &'a Manager, raw: ffi::DrmModeGetCrtc) -> Crtc<'a> {
        Crtc {
            manager: man,
            id: raw.raw.crtc_id,
            position: (raw.raw.x, raw.raw.y),
            mode: match raw.raw.mode.clock {
                0 => None,
                _ => Some(Mode::from(raw.raw.mode))
            }
        }
    }

    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    pub fn mode(&self) -> Option<Mode> {
        self.mode.clone()
    }
}

impl<'a> Drop for Crtc<'a> {
    fn drop(&mut self) {
        self.manager.unload_crtc(self.id);
    }
}

#[derive(Clone)]
pub struct Crtcs<'a> {
    manager: &'a Manager<'a>,
    crtcs: IntoIter<CrtcId>
}

impl<'a> Iterator for Crtcs<'a> {
    type Item = Result<Crtc<'a>>;
    fn next(&mut self) -> Option<Result<Crtc<'a>>> {
        let raw = match self.crtcs.next() {
            Some(id) => self.manager.load_crtc(id),
            None => return None
        };

        match raw {
            Ok(r) => Some(Ok(Crtc::from_raw(self.manager, r))),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Crtcs<'a> {
    pub fn new(man: &'a Manager<'a>, iter: IntoIter<CrtcId>) -> Crtcs<'a> {
        Crtcs {
            manager: man,
            crtcs: iter
        }
    }
}

