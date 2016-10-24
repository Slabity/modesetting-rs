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
  - Display Controllers: Controls the scanout of a Framebuffer to one or more
  Connectos.
  - Framebuffer: Pixel data that can be used by a Display Controller

  The standard procedure to do this is to first open the device and select the
  Connectors you will use. For each Connector, decide on a mode you will use
  and attach the proper Encoders. Create the Framebuffers you wish to display
  and set up the Display Controllers for proper scanout.

  For more information, see the `drm-kms` man page.
  */

#[macro_use]
extern crate error_chain;
extern crate libc;

mod ffi;
pub mod result;

#[cfg(feature="dumbbuffer")]
pub mod dumbbuffer;

use result::{Result, ErrorKind};

use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use std::mem::transmute;
use std::vec::IntoIter;
use std::ffi::CStr;

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

/// A `MasterLock` is a lock for a `MasterDevice`. It ensures that only one
/// handle to the DRM Master is in use at once.
struct MasterLock<'a> {
    device: &'a Device,
    _guard: MutexGuard<'a, ()>
}

impl<'a> MasterLock<'a> {
    #[cfg(not(feature="user"))]
    fn from_device(device: &'a Device) -> Result<MasterLock<'a>> {
        try!(ffi::set_master(device.file.as_raw_fd()));
        let guard = device.master_lock.lock().unwrap();
        let lock = MasterLock {
            device: device,
            _guard: guard
        };
        Ok(lock)
    }

    #[cfg(feature="user")]
    fn from_device(device: &'a Device) -> Result<MasterLock<'a>> {
        let guard = device.master_lock.lock().unwrap();
        let lock = MasterLock {
            device: device,
            _guard: guard
        };
        Ok(lock)
    }
}

#[cfg(not(feature="user"))]
impl<'a> Drop for MasterLock<'a> {
    fn drop(&mut self) {
        let _ = ffi::drop_master(self.device.file.as_raw_fd());
    }
}

/// A `Device` is an unprivileged handle to the character device file that
/// provides modesetting capabilities.
pub struct Device {
    file: File,
    master_lock: Mutex<()>
}

impl AsRef<File> for Device {
    fn as_ref(&self) -> &File {
        &self.file
    }
}

impl From<File> for Device {
    fn from(file: File) -> Device {
        Device {
            file: file,
            master_lock: Mutex::new(())
        }
    }
}

impl<'a> Device {
    /// Attempt to open the file specified at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Self::from(file);
        Ok(dev)
    }

    /// Acquire the master lock and create a `MasterDevice`
    pub fn lock_master(&'a self) -> Result<MasterDevice<'a>> {
        let lock = try!(MasterLock::from_device(self));
        let fd = self.file.as_raw_fd();
        let raw = try!(ffi::DrmModeCardRes::new(fd));
        let master = MasterDevice {
            handle: &self.file,
            _guard: lock,
            connectors: Mutex::new(raw.connectors.clone()),
            encoders: Mutex::new(raw.encoders.clone()),
            controllers: Mutex::new(raw.crtcs.clone()),
            controllers_order: raw.crtcs.clone(),
        };
        Ok(master)
    }
}

/// A `MasterDevice` is an privileged handle to the character device file that
/// provides full modesetting capabilities.
pub struct MasterDevice<'a> {
    handle: &'a File,
    _guard: MasterLock<'a>,
    connectors: Mutex<Vec<ConnectorId>>,
    encoders: Mutex<Vec<EncoderId>>,
    controllers: Mutex<Vec<ControllerId>>,
    controllers_order: Vec<ControllerId>,
}

impl<'a> AsRef<File> for MasterDevice<'a> {
    fn as_ref(&self) -> &File {
        self.handle
    }
}

impl<'a> MasterDevice<'a> {
    /// Attempt to create an abstract `Framebuffer` object from the provided
    /// `Buffer`.
    pub fn framebuffer<T: Buffer>(&self, buffer: &T) -> Result<Framebuffer> {
        Framebuffer::create(self, buffer)
    }

    /// Return an iterator over the list of connectors.
    pub fn connectors(&'a self) -> Connectors<'a> {
        let guard = self.connectors.lock().unwrap();
        let iter = guard.clone().into_iter();
        Connectors::new(self, iter)
    }

    /// Return an iterator over the list of encoders.
    pub fn encoders(&'a self) -> Encoders<'a> {
        let guard = self.encoders.lock().unwrap();
        let iter = guard.clone().into_iter();
        Encoders::new(self, iter)
    }

    /// Return an iterator over the list of display controllers.
    pub fn controllers(&'a self) -> DisplayControllers<'a> {
        let guard = self.controllers.lock().unwrap();
        let iter = guard.clone().into_iter();
        DisplayControllers::new(self, iter)
    }

    /// Attempt to load a `Connector` with the given `ConnectorId`.
    ///
    /// # Errors
    ///
    /// `Error::NotAvailable` - Returned if ownership of the resource has
    /// already been taken.
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
            None => return Err(ErrorKind::NotAvailable.into())
        };

        let raw = try!(ffi::DrmModeGetConnector::new(self.handle.as_raw_fd(), id));

        let connector = Connector {
            device: self,
            id: raw.raw.connector_id,
            interface: ConnectorInterface::from(raw.raw.connector_type),
            state: ConnectorState::from(raw.raw.connection),
            encoders: raw.encoders.clone(),
            modes: raw.modes.iter().map(| raw | Mode::from(*raw)).collect(),
            size: (raw.raw.mm_width, raw.raw.mm_height)
        };

        Ok(connector)
    }

    /// Attempt to load a `Encoder` with the given `EncoderId`.
    ///
    /// # Errors
    ///
    /// `Error::NotAvailable` - Returned if ownership of the resource has
    /// already been taken.
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
            None => return Err(ErrorKind::NotAvailable.into())
        };

        let raw = try!(ffi::DrmModeGetEncoder::new(self.handle.as_raw_fd(), id));
        let mut possible_controllers = Vec::new();
        let mut pos_bits = raw.raw.possible_crtcs;
        for id in self.controllers_order.iter() {
            if (pos_bits & 0x1) == 0x1 {
                possible_controllers.push(*id);
            }
            pos_bits = pos_bits >> 1;
        }

        let encoder = Encoder {
            device: self,
            id: raw.raw.encoder_id,
            controllers: possible_controllers
        };

        Ok(encoder)
    }

    /// Attempt to load a `DisplayController` with the given `ControllerId`.
    ///
    /// # Errors
    ///
    /// `Error::NotAvailable` - Returned if ownership of the resource has
    /// already been taken.
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
            None => return Err(ErrorKind::NotAvailable.into())
        };

        let raw = try!(ffi::DrmModeGetCrtc::new(self.handle.as_raw_fd(), id));

        let controller = DisplayController {
            device: self,
            id: raw.raw.crtc_id,
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

/// A framebuffer is a virtual object that is implemented by the graphics
/// driver. It can be created from any object that implements the `Buffer`
/// trait.
pub struct Framebuffer<'a> {
    device: &'a MasterDevice<'a>,
    id: FramebufferId
}

impl<'a> Framebuffer<'a> {
    fn create<T: Buffer>(device: &'a MasterDevice<'a>, buffer: &T) -> Result<Self> {
        let (width, height) = buffer.size();
        let depth = buffer.depth();
        let bpp = buffer.bpp();
        let pitch = buffer.pitch();
        let handle = buffer.handle();
        let fd = device.handle.as_raw_fd();
        let raw = try!(ffi::DrmModeAddFb::new(fd, width, height, depth, bpp, pitch, handle));
        let fb = Framebuffer {
            device: device,
            id: raw.raw.fb_id
        };
        Ok(fb)
    }
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        ffi::DrmModeRmFb::new(self.device.handle.as_raw_fd(), self.id).unwrap();
    }
}

/// A `Connector` is a representation of a physical display interface on the
/// system, such as an HDMI or VGA port.
pub struct Connector<'a> {
    device: &'a MasterDevice<'a>,
    id: ConnectorId,
    interface: ConnectorInterface,
    state: ConnectorState,
    encoders: Vec<EncoderId>,
    modes: Vec<Mode>,
    size: (u32, u32)
}

impl<'a> Connector<'a> {
    /// Returns the interface type of the connector.
    pub fn interface(&self) -> ConnectorInterface {
        self.interface
    }

    /// Returns the current connection state of the connector.
    pub fn state(&self) -> ConnectorState {
        self.state
    }

    /// Return an iterator over all compatible encoders for this connector.
    pub fn encoders(&self) -> Encoders<'a> {
        Encoders {
            device: self.device,
            encoders: self.encoders.clone().into_iter()
        }
    }

    /// Return a list of display modes that this `Connector` can support.
    pub fn modes(&self) -> Vec<Mode> {
        self.modes.clone()
    }

    /// Return the size of the display in millimeters.
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}

impl<'a> Drop for Connector<'a> {
    fn drop(&mut self) {
        self.device.unload_connector(self.id);
    }
}

/// An iterator over a list of `Connector` objects.
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
    pub fn new(device: &'a MasterDevice, iter: IntoIter<ConnectorId>) -> Connectors<'a> {
        Connectors {
            device: device,
            connectors: iter
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// The type of interface a `Connector` is.
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
/// The state of a `Connector`.
pub enum ConnectorState {
    /// The `Connector` is plugged in and ready for use.
    Connected = ffi::Connection::FFI_DRM_MODE_CONNECTED as isize,
    /// The `Connector` is unplugged.
    Disconnected = ffi::Connection::FFI_DRM_MODE_DISCONNECTED as isize,
    /// Sometimes a `Connector` will have an unkown state. It is safe to use,
    /// but may not provide the expected functionality.
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

/// An `Encoder` is responsibly for converting the pixel data into a format
/// that the `Connector` can use. Each `Encoder` can only be attached to one
/// `Connector` at a time, and not all `Encoder` objects are compatible with
/// all `Connector` objects.
pub struct Encoder<'a> {
    device: &'a MasterDevice<'a>,
    id: EncoderId,
    controllers: Vec<ControllerId>
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        self.device.unload_encoder(self.id);
    }
}

/// An iterator over a list of `Encoder` objects.
pub struct Encoders<'a> {
    device: &'a MasterDevice<'a>,
    encoders: IntoIter<EncoderId>
}

impl<'a> Encoder<'a> {
    pub fn controllers(&self) -> DisplayControllers<'a> {
        DisplayControllers {
            device: self.device,
            controllers: self.controllers.clone().into_iter()
        }
    }
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
    pub fn new(device: &'a MasterDevice, iter: IntoIter<EncoderId>) -> Encoders<'a> {
        Encoders {
            device: device,
            encoders: iter
        }
    }
}

/// A `DisplayController` controls the timing and scanout of a `Framebuffer` to
/// one or more `Connector` objects.
pub struct DisplayController<'a> {
    device: &'a MasterDevice<'a>,
    id: ControllerId,
}

impl<'a, 'b, 'c, 'd> DisplayController<'a> {
    /// Sets the controller. Unstable.
    pub fn set_controller(self, fb: &'b Framebuffer,
                          connector: &'c Connector,
                          encoder: &'d Encoder, mode: Mode) -> Result<()> {
        try!(
            ffi::DrmModeSetCrtc::new(self.device.handle.as_raw_fd(),
            self.id, fb.id, 0, 0, vec![connector.id], mode.into())
        );
        Ok(())
    }
}

impl<'a> Drop for DisplayController<'a> {
    fn drop(&mut self) {
        self.device.unload_controller(self.id);
    }
}

/// An iterator over a list of `DisplayController` objects.
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

#[derive(Debug, PartialEq, Clone)]
pub struct Mode {
    pub name: String,
    pub clock: u32,
    pub display: (u16, u16),
    pub hsync: (u16, u16),
    pub vsync: (u16, u16),
    pub hskew: u16,
    pub vscan: u16,
    pub htotal: u16,
    pub vtotal: u16,
    pub vrefresh: u32,
    pub flags: u32,
    pub mode_type: u32,
}

impl From<ffi::drm_mode_modeinfo> for Mode {
    fn from(raw: ffi::drm_mode_modeinfo) -> Mode {
        let name = unsafe {
            CStr::from_ptr(raw.name.as_ptr()).to_str().unwrap()
        };

        Mode {
            name: name.to_string(),
            clock: raw.clock,
            display: (raw.hdisplay, raw.vdisplay),
            hsync: (raw.hsync_start, raw.hsync_end),
            vsync: (raw.vsync_start, raw.vsync_end),
            hskew: raw.hskew,
            vscan: raw.vscan,
            htotal: raw.htotal,
            vtotal: raw.vtotal,
            vrefresh: raw.vrefresh,
            flags: raw.flags,
            mode_type: raw.type_
        }
    }
}

impl Into<ffi::drm_mode_modeinfo> for Mode {
    fn into(self) -> ffi::drm_mode_modeinfo {
        let (hdisplay, vdisplay) = self.display;
        let (hsync_start, hsync_end) = self.hsync;
        let (vsync_start, vsync_end) = self.vsync;

        ffi::drm_mode_modeinfo {
            name: [0; 32],
            clock: self.clock,
            hdisplay: hdisplay,
            vdisplay: vdisplay,
            hsync_start: hsync_start,
            hsync_end: hsync_end,
            vsync_start: vsync_start,
            vsync_end: vsync_end,
            hskew: self.hskew,
            vscan: self.vscan,
            htotal: self.htotal,
            vtotal: self.vtotal,
            vrefresh: self.vrefresh,
            flags: self.flags,
            type_: self.mode_type
        }
    }
}

/// An object that implements the `Buffer` trait allows it to be used as a part
/// of a `Framebuffer`.
pub trait Buffer {
    /// The width and height of the buffer.
    fn size(&self) -> (u32, u32);
    /// The depth size of the buffer.
    fn depth(&self) -> u8;
    /// The number of 'bits per pixel'
    fn bpp(&self) -> u8;
    /// The pitch of the buffer.
    fn pitch(&self) -> u32;
    /// A handle provided by your graphics driver that can be used to reference
    /// the buffer, such as a dumb buffer handle or a handle provided by mesa's
    /// libgbm.
    fn handle(&self) -> u32;
}


