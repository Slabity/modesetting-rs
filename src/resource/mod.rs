mod connector;
mod encoder;
mod crtc;
mod framebuffer;

pub use self::connector::*;
pub use self::encoder::*;
pub use self::crtc::*;
pub use self::framebuffer::*;

use super::Device;
use super::error::Error;
use super::error::Result;
use super::ffi;

use std::sync::Mutex;

pub type ResourceId = u32;

#[derive(Debug)]
pub struct Manager<'a> {
    device: &'a Device,
    connectors: Mutex<Vec<ConnectorId>>,
    encoders: Mutex<Vec<EncoderId>>,
    crtcs: Mutex<Vec<CrtcId>>,
    framebuffers: Mutex<Vec<FramebufferId>>,
}

impl<'a> Manager<'a> {
    pub fn from_device(dev: &'a Device) -> Result<Manager<'a>> {
        let raw = try!(dev.resources());
        Ok(Manager {
            device: dev,
            connectors: Mutex::new(raw.connectors.clone()),
            encoders: Mutex::new(raw.encoders.clone()),
            crtcs: Mutex::new(raw.crtcs.clone()),
            framebuffers: Mutex::new(raw.framebuffers.clone())
        })
    }

    pub fn connectors(&'a self) -> Connectors {
        let guard = self.connectors.lock().unwrap();
        let iter = guard.clone().into_iter();
        Connectors::new(self, iter)
    }

    pub fn encoders(&'a self) -> Encoders {
        let guard = self.encoders.lock().unwrap();
        let iter = guard.clone().into_iter();
        Encoders::new(self, iter)
    }

    pub fn crtcs(&'a self) -> Crtcs {
        let guard = self.crtcs.lock().unwrap();
        let iter = guard.clone().into_iter();
        Crtcs::new(self, iter)
    }

    pub fn framebuffers(&'a self) -> Framebuffers {
        let guard = self.framebuffers.lock().unwrap();
        let iter = guard.clone().into_iter();
        Framebuffers::new(self, iter)
    }

    fn load_connector(&'a self, id: ConnectorId) -> Result<ffi::DrmModeGetConnector> {
        let pos = {
            let guard = self.connectors.lock().unwrap();
            guard.iter().position(| x | *x == id)
        };
        match pos {
            Some(p) => {
                let mut guard = self.connectors.lock().unwrap();
                guard.remove(p);
            },
            None => return Err(Error::NotAvailable)
        };

        self.device.connector(id)
    }

    fn unload_connector(&'a self, id: ConnectorId) {
        let mut guard = self.connectors.lock().unwrap();
        guard.push(id);
    }

    fn load_encoder(&'a self, id: EncoderId) -> Result<ffi::DrmModeGetEncoder> {
        let pos = {
            let guard = self.encoders.lock().unwrap();
            guard.iter().position(| x | *x == id)
        };
        match pos {
            Some(p) => {
                let mut guard = self.encoders.lock().unwrap();
                guard.remove(p);
            },
            None => return Err(Error::NotAvailable)
        };

        self.device.encoder(id)
    }

    fn unload_encoder(&'a self, id: EncoderId) {
        let mut guard = self.encoders.lock().unwrap();
        guard.push(id);
    }

    fn load_crtc(&'a self, id: CrtcId) -> Result<ffi::DrmModeGetCrtc> {
        let pos = {
            let guard = self.crtcs.lock().unwrap();
            guard.iter().position(| x | *x == id)
        };
        match pos {
            Some(p) => {
                let mut guard = self.crtcs.lock().unwrap();
                guard.remove(p);
            },
            None => return Err(Error::NotAvailable)
        };

        self.device.crtc(id)
    }

    fn unload_crtc(&'a self, id: CrtcId) {
        let mut guard = self.crtcs.lock().unwrap();
        guard.push(id);
    }

    fn load_framebuffer(&'a self, id: FramebufferId) -> Result<ffi::DrmModeGetFb> {
        let pos = {
            let guard = self.framebuffers.lock().unwrap();
            guard.iter().position(| x | *x == id)
        };
        match pos {
            Some(p) => {
                let mut guard = self.framebuffers.lock().unwrap();
                guard.remove(p);
            },
            None => return Err(Error::NotAvailable)
        };

        self.device.framebuffer(id)
    }

    fn unload_framebuffer(&'a self, id: FramebufferId) {
        let mut guard = self.framebuffers.lock().unwrap();
        guard.push(id);
    }
}
