use super::Manager;
use super::super::error::Result;
use super::super::ffi;
use super::{ResourceId, CrtcId};

use std::vec::IntoIter;

pub type EncoderId = ResourceId;

#[derive(Debug)]
pub struct Encoder<'a> {
    manager: &'a Manager<'a>,
    id: EncoderId,
    crtc: CrtcId
}

impl<'a> Encoder<'a> {
    fn from_raw(man: &'a Manager, raw: ffi::DrmModeGetEncoder) -> Encoder<'a> {
        Encoder {
            manager: man,
            id: raw.raw.encoder_id,
            crtc: raw.raw.crtc_id
        }
    }
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        self.manager.unload_encoder(self.id);
    }
}

#[derive(Clone)]
pub struct Encoders<'a> {
    manager: &'a Manager<'a>,
    encoders: IntoIter<EncoderId>
}

impl<'a> Iterator for Encoders<'a> {
    type Item = Result<Encoder<'a>>;
    fn next(&mut self) -> Option<Result<Encoder<'a>>> {
        let raw = match self.encoders.next() {
            Some(id) => self.manager.load_encoder(id),
            None => return None
        };

        match raw {
            Ok(r) => Some(Ok(Encoder::from_raw(self.manager, r))),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Encoders<'a> {
    pub fn new(man: &'a Manager<'a>, iter: IntoIter<EncoderId>) -> Encoders<'a> {
        Encoders {
            manager: man,
            encoders: iter
        }
    }
}

