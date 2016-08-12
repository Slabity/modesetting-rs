/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block device and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a Device and using four types of resources:

  - Connectors: The physical interfaces on your GPU, such as HDMI, VGA, and
  LVDS ports.
  - Encoders: These modify and deliver the pixel data to the connectors.
  - CRTCs: Points to a scanout buffer in video memory and reads it based on the
  mode it is set to.
  - Framebuffer: Pixel data that can be used by a CRTC

  The standard procedure to do this is to first open the device. Then choose
  the connectors you wish to use. For each connector, get your desired mode and
  choose an available CRTC to use (in most situations, attaching a CRTC to a
  connector will automatically choose the preferred encoder). Once you have a
  suitable Connector, CRTC, and Mode, you can create a framebuffer for scanout.

  For more information, see the `drm-kms` man page.
  */

extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
pub mod resource;
pub mod mode;

use error::{Result, Error};
use mode::Mode;

use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Mutex;
use std::marker::PhantomData;
use std::mem::transmute;
use std::vec::IntoIter;

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

pub struct Device {
    file: File
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: File::from_raw_fd(fd)
        }
    }
}

impl IntoRawFd for Device {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

impl From<File> for Device {
    fn from(file: File) -> Device {
        Device {
            file: file
        }
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: file,
        };
        Ok(dev)
    }

    pub fn dumb_buffer(&self, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        DumbBuffer::create(self, width, height, bpp)
    }

    pub fn framebuffer(&self, buffer: &Buffer) -> Result<Framebuffer> {
        let (width, height) = buffer.size();
        let depth = buffer.depth();
        let bpp = buffer.bpp();
        let pitch = buffer.pitch();
        let handle = buffer.handle();
        let raw = try!(ffi::DrmModeAddFb::new(self.file.as_raw_fd(), width, height, depth, bpp, pitch, handle));
        let fb = Framebuffer {
            device: self,
            id: raw.raw.fb_id
        };
        Ok(fb)
    }
}

#[derive(Debug)]
pub struct MasterDevice<'a> {
    handle: RawFd,
    connectors: Mutex<Vec<ConnectorId>>,
    encoders: Mutex<Vec<EncoderId>>,
    controllers: Mutex<Vec<ControllerId>>,
    device: PhantomData<&'a Device>
}

impl<'a> AsRawFd for MasterDevice<'a> {
    fn as_raw_fd(&self) -> RawFd {
        self.handle
    }
}

impl<'a> FromRawFd for MasterDevice<'a> {
    unsafe fn from_raw_fd(fd: RawFd) -> MasterDevice<'a> {
        let raw = ffi::DrmModeCardRes::new(fd).unwrap();
        MasterDevice {
            handle: fd,
            connectors: Mutex::new(raw.connectors.clone()),
            encoders: Mutex::new(raw.encoders.clone()),
            controllers: Mutex::new(raw.crtcs.clone()),
            device: PhantomData
        }
    }
}

impl<'a> IntoRawFd for MasterDevice<'a> {
    fn into_raw_fd(self) -> RawFd {
        self.handle
    }
}

impl<'a> MasterDevice<'a> {
    fn from_device(device: &'a Device) -> MasterDevice<'a> {
        unsafe {
            Self::from_raw_fd(device.as_raw_fd())
        }
    }

    pub fn connectors(&'a self) -> Connectors<'a> {
        let guard = self.connectors.lock().unwrap();
        let iter = guard.clone().into_iter();
        Connectors::new(self, iter)
    }

    pub fn encoders(&'a self) -> Encoders<'a> {
        let guard = self.encoders.lock().unwrap();
        let iter = guard.clone().into_iter();
        Encoders::new(self, iter)
    }

    pub fn controllers(&'a self) -> DisplayControllers<'a> {
        let guard = self.controllers.lock().unwrap();
        let iter = guard.clone().into_iter();
        DisplayControllers::new(self, iter)
    }

    pub fn connector(&'a self, id: ConnectorId) -> Result<Connector<'a>> {
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

        let raw = try!(ffi::DrmModeGetConnector::new(self.handle, id));

        let connector = Connector {
            device: self,
            id: raw.raw.connector_id,
            interface: ConnectorInterface::from(raw.raw.connector_type),
            state: ConnectorState::from(raw.raw.connection),
            curr_encoder: raw.raw.encoder_id,
            encoders: raw.encoders.clone(),
            modes: raw.modes.iter().map(| raw | Mode::from(*raw)).collect(),
            size: (raw.raw.mm_width, raw.raw.mm_height)
        };

        Ok(connector)
    }

    pub fn encoder(&'a self, id: EncoderId) -> Result<Encoder<'a>> {
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

        let raw = try!(ffi::DrmModeGetEncoder::new(self.handle, id));

        let encoder = Encoder {
            device: self,
            id: raw.raw.encoder_id
        };

        Ok(encoder)
    }

    pub fn controller(&'a self, id: ControllerId) -> Result<DisplayController<'a>> {
        let pos = {
            let guard = self.controllers.lock().unwrap();
            guard.iter().position(| x | *x == id)
        };
        match pos {
            Some(p) => {
                let mut guard = self.controllers.lock().unwrap();
                guard.remove(p);
            },
            None => return Err(Error::NotAvailable)
        };

        let raw = try!(ffi::DrmModeGetCrtc::new(self.handle, id));

        let controller = DisplayController {
            device: self,
            id: raw.raw.crtc_id
        };

        Ok(controller)
    }
    fn unload_connector(&'a self, id: ConnectorId) {
        let mut guard = self.connectors.lock().unwrap();
        guard.push(id);
    }

    fn unload_encoder(&'a self, id: EncoderId) {
        let mut guard = self.encoders.lock().unwrap();
        guard.push(id);
    }

    fn unload_controller(&'a self, id: ControllerId) {
        let mut guard = self.controllers.lock().unwrap();
        guard.push(id);
    }
}

pub struct Framebuffer<'a> {
    device: &'a AsRawFd,
    id: FramebufferId
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        // TODO: Remove FB from device here.
    }
}

pub trait Buffer {
    fn size(&self) -> (u32, u32);
    fn depth(&self) -> u8;
    fn bpp(&self) -> u8;
    fn pitch(&self) -> u32;
    fn handle(&self) -> u32;
}

pub struct DumbBuffer<'a> {
    device: &'a AsRawFd,
    size: (u32, u32),
    depth: u8,
    bpp: u8,
    pitch: u32,
    handle: u32
}

impl<'a> DumbBuffer<'a> {
    fn create(device: &'a AsRawFd, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(device.as_raw_fd(), width, height, bpp));
        let buffer = DumbBuffer {
            device: device,
            size: (width, height),
            depth: 24,
            bpp: bpp,
            pitch: raw.raw.pitch,
            handle: raw.raw.handle
        };
        Ok(buffer)
    }
}

impl<'a> Drop for DumbBuffer<'a> {
    fn drop(&mut self) {
        ffi::DrmModeDestroyDumbBuffer::new(self.device.as_raw_fd(), self.handle).unwrap();
    }
}

impl<'a> Buffer for DumbBuffer<'a> {
    fn size(&self) -> (u32, u32) { self.size }
    fn depth(&self) -> u8 { self.depth }
    fn bpp(&self) -> u8 { self.bpp }
    fn pitch(&self) -> u32 { self.pitch }
    fn handle(&self) -> u32 { self.handle }
}

#[derive(Debug)]
pub struct Connector<'a> {
    device: &'a MasterDevice<'a>,
    id: ConnectorId,
    interface: ConnectorInterface,
    state: ConnectorState,
    curr_encoder: EncoderId,
    encoders: Vec<EncoderId>,
    modes: Vec<Mode>,
    size: (u32, u32)
}

impl<'a> Connector<'a> {
    pub fn interface(&self) -> ConnectorInterface {
        self.interface
    }

    pub fn state(&self) -> ConnectorState {
        self.state
    }
}

impl<'a> Drop for Connector<'a> {
    fn drop(&mut self) {
        self.device.unload_connector(self.id);
    }
}

#[derive(Clone)]
pub struct Connectors<'a> {
    device: &'a MasterDevice<'a>,
    connectors: IntoIter<ConnectorId>
}

impl<'a> Iterator for Connectors<'a> {
    type Item = Result<Connector<'a>>;
    fn next(&mut self) -> Option<Result<Connector<'a>>> {
        match self.connectors.next() {
            Some(id) => Some(self.device.connector(id)),
            None => None
        }
    }
}

impl<'a> Connectors<'a> {
    pub fn new(device: &'a MasterDevice<'a>, iter: IntoIter<ConnectorId>) -> Connectors<'a> {
        Connectors {
            device: device,
            connectors: iter
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectorInterface {
    Unknown = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Unknown as isize,
    VGA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_VGA as isize,
    DVII = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVII as isize,
    DVID = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVID as isize,
    DVIA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DVIA as isize,
    Composite = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Composite as isize,
    SVideo = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_SVIDEO as isize,
    LVDS = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_LVDS as isize,
    Component = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_Component as isize,
    NinePinDIN = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_9PinDIN as isize,
    DisplayPort = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DisplayPort as isize,
    HDMIA = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_HDMIA as isize,
    HDMIB = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_HDMIB as isize,
    TV = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_TV as isize,
    EDP = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_eDP as isize,
    Virtual = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_VIRTUAL as isize,
    DSI = ffi::ConnectorInterface::FFI_DRM_MODE_CONNECTOR_DSI as isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectorState {
    Connected = ffi::Connection::FFI_DRM_MODE_CONNECTED as isize,
    Disconnected = ffi::Connection::FFI_DRM_MODE_DISCONNECTED as isize,
    Unknown = ffi::Connection::FFI_DRM_MODE_UNKNOWN as isize
}

impl From<u32> for ConnectorInterface {
    fn from(ty: u32) -> ConnectorInterface {
        unsafe { transmute(ty as u8) }
    }
}

impl From<u32> for ConnectorState {
    fn from(ty: u32) -> ConnectorState {
        unsafe { transmute(ty as u8) }
    }
}

#[derive(Debug)]
pub struct Encoder<'a> {
    device: &'a MasterDevice<'a>,
    id: EncoderId,
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        self.device.unload_encoder(self.id);
    }
}

#[derive(Clone)]
pub struct Encoders<'a> {
    device: &'a MasterDevice<'a>,
    encoders: IntoIter<EncoderId>
}

impl<'a> Iterator for Encoders<'a> {
    type Item = Result<Encoder<'a>>;
    fn next(&mut self) -> Option<Result<Encoder<'a>>> {
        match self.encoders.next() {
            Some(id) => Some(self.device.encoder(id)),
            None => None
        }
    }
}

impl<'a> Encoders<'a> {
    pub fn new(device: &'a MasterDevice<'a>, iter: IntoIter<EncoderId>) -> Encoders<'a> {
        Encoders {
            device: device,
            encoders: iter
        }
    }
}

#[derive(Debug)]
pub struct DisplayController<'a> {
    device: &'a MasterDevice<'a>,
    id: ControllerId
}

impl<'a> Drop for DisplayController<'a> {
    fn drop(&mut self) {
        self.device.unload_controller(self.id);
    }
}

#[derive(Clone)]
pub struct DisplayControllers<'a> {
    device: &'a MasterDevice<'a>,
    controllers: IntoIter<ControllerId>
}

impl<'a> Iterator for DisplayControllers<'a> {
    type Item = Result<DisplayController<'a>>;
    fn next(&mut self) -> Option<Result<DisplayController<'a>>> {
        match self.controllers.next() {
            Some(id) => Some(self.device.controller(id)),
            None => return None
        }
    }
}

impl<'a> DisplayControllers<'a> {
    pub fn new(device: &'a MasterDevice, iter: IntoIter<ControllerId>) -> DisplayControllers<'a> {
        DisplayControllers {
            device: device,
            controllers: iter
        }
    }
}

