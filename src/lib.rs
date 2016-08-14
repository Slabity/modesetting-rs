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

extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
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

use libc::{mmap, munmap, PROT_READ, PROT_WRITE, MAP_SHARED};

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

/// A `Device` is an unprivileged handle to the character device file that
/// provides modesetting capabilities.
pub struct Device {
    file: File
}

/// Retrieve the raw file descriptor of the character device.
impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

/// Create a `Device` from an already opened file descriptor.
///
/// # Safety
///
/// The newly created `Device` assumes it is the sole owner of the file
/// descriptor. Closing the file descriptor elsewhere will lead to a panic.
///
/// It is assumed that the provided file descriptor has both read and write
/// options enabled.
impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: File::from_raw_fd(fd)
        }
    }
}

/// Consumes ownership of the `Device` and returns a raw file descriptor.
impl IntoRawFd for Device {
    fn into_raw_fd(self) -> RawFd {
        self.file.into_raw_fd()
    }
}

/// Take an already opened `File` and treat it as a `Device`
impl From<File> for Device {
    fn from(file: File) -> Device {
        Device {
            file: file
        }
    }
}

impl Device {
    /// Attempt to open the file specified at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: file,
        };
        Ok(dev)
    }

    /// Attempt to create a `DumbBuffer` object for this device.
    pub fn dumb_buffer(&self, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        DumbBuffer::create(self, width, height, bpp)
    }

    /// Attempt to create an abstract `Framebuffer` object from the provided
    /// `Buffer`.
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

/// A `MasterDevice` is an privileged handle to the character device file that
/// provides modesetting capabilities.
///
/// Unlike a `Device`, a `MasterDevice` does not own the file descriptor used.
/// It is the responsibility of the program to open and close the file
/// descriptor.
///
/// A `MasterDevice` can be used to access various modesetting resources. It
/// also prevents dual ownership of any single resource in multiple locations.
pub struct MasterDevice<'a> {
    handle: RawFd,
    connectors: Mutex<Vec<ConnectorId>>,
    encoders: Mutex<Vec<EncoderId>>,
    controllers: Mutex<Vec<ControllerId>>,
    controllers_order: Vec<ControllerId>,
    device: PhantomData<&'a Device>
}

impl<'a> FromRawFd for MasterDevice<'a> {
    /// Create a `MasterDevice` from an already opened file descriptor.
    ///
    /// # Safety
    ///
    /// The `MasterDevice` does not assume ownership of the file descriptor.
    /// Closing the file descriptor while the `MasterDevice` is in scope may lead
    /// to panics and errors.
    ///
    /// It is assumed that the provided file descriptor has both read and write
    /// options enabled.
    unsafe fn from_raw_fd(fd: RawFd) -> MasterDevice<'a> {
        let raw = ffi::DrmModeCardRes::new(fd).unwrap();
        MasterDevice {
            handle: fd,
            connectors: Mutex::new(raw.connectors.clone()),
            encoders: Mutex::new(raw.encoders.clone()),
            controllers: Mutex::new(raw.crtcs.clone()),
            controllers_order: raw.crtcs.clone(),
            device: PhantomData
        }
    }
}

impl<'a> MasterDevice<'a> {
    /// Create a `MasterDevice` from a `Device`. The newly created
    /// `MasterDevice` will not outlive the `Device` it is created from.
    fn from_device(device: &'a Device) -> MasterDevice<'a> {
        unsafe {
            Self::from_raw_fd(device.as_raw_fd())
        }
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
            None => return Err(Error::NotAvailable)
        };

        let raw = try!(ffi::DrmModeGetConnector::new(self.handle, id));

        let connector = Connector {
            device: self,
            id: raw.raw.connector_id,
            interface: ConnectorInterface::from(raw.raw.connector_type),
            state: ConnectorState::from(raw.raw.connection),
            curr_encoder: None,
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
            None => return Err(Error::NotAvailable)
        };

        let raw = try!(ffi::DrmModeGetEncoder::new(self.handle, id));
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
            None => return Err(Error::NotAvailable)
        };

        let raw = try!(ffi::DrmModeGetCrtc::new(self.handle, id));

        let controller = DisplayController {
            device: self,
            id: raw.raw.crtc_id,
            connectors: Vec::new(),
            framebuffer: None
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
    device: &'a AsRawFd,
    id: FramebufferId
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        // TODO: Remove FB from device here.
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

/// A `DumbBuffer` is a simple buffer type provided by all major graphics
/// drivers. It can be mapped to main memory and provided direct access to the
/// pixel data to be displayed.
pub struct DumbBuffer<'a> {
    device: &'a AsRawFd,
    size: (u32, u32),
    depth: u8,
    bpp: u8,
    pitch: u32,
    handle: u32,
    raw_size: usize
}

impl<'a> DumbBuffer<'a> {
    /// Attempts to create a `DumbBuffer` from the given size and bits per
    /// pixel.
    fn create(device: &'a AsRawFd, width: u32, height: u32, bpp: u8) -> Result<DumbBuffer> {
        let raw = try!(ffi::DrmModeCreateDumbBuffer::new(device.as_raw_fd(), width, height, bpp));
        let buffer = DumbBuffer {
            device: device,
            size: (width, height),
            depth: 24,
            bpp: bpp,
            pitch: raw.raw.pitch,
            handle: raw.raw.handle,
            raw_size: raw.raw.size as usize
        };
        Ok(buffer)
    }

    /// Attempts to map the buffer directly into main memory as represented by
    /// a mutable `&[u8]`. Because this data is copied to the graphics card on
    /// each write, it is recommended to draw into another buffer of identical
    /// size and then copy its contents using `copy_from_slice`.
    pub fn map(&self) -> Result<&mut [u8]> {
        let raw = try!(ffi::DrmModeMapDumbBuffer::new(self.device.as_raw_fd(), self.handle));
        let ptr = unsafe {
            mmap(std::ptr::null_mut(), self.raw_size, PROT_READ | PROT_WRITE, MAP_SHARED, self.device.as_raw_fd(), raw.raw.offset as i64)
        } as *mut u8;
        Ok(unsafe {
            std::slice::from_raw_parts_mut(ptr, self.raw_size)
        })
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

/// A `Connector` is a representation of a physical display interface on the
/// system, such as an HDMI or VGA port.
pub struct Connector<'a> {
    device: &'a MasterDevice<'a>,
    id: ConnectorId,
    interface: ConnectorInterface,
    state: ConnectorState,
    curr_encoder: Option<Encoder<'a>>,
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

    /// Attach an `Encoder` to the `Connector`.
    pub fn attach_encoder(&mut self, encoder: Encoder<'a>) -> Result<()> {
        match self.encoders.iter().any(| &enc | enc == encoder.id) {
            true => {
                self.curr_encoder = Some(encoder);
                Ok(())
            },
            false => Err(Error::Incompatible)
        }
    }

    /// Return a list of display modes that this `Connector` can support.
    pub fn modes(&self) -> Vec<Mode> {
        self.modes.clone()
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
    pub fn new(device: &'a MasterDevice<'a>, iter: IntoIter<ConnectorId>) -> Connectors<'a> {
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
    pub fn new(device: &'a MasterDevice<'a>, iter: IntoIter<EncoderId>) -> Encoders<'a> {
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
    connectors: Vec<Connector<'a>>,
    framebuffer: Option<&'a Framebuffer<'a>>
}

impl<'a> DisplayController<'a> {
    /// Sets the controller. Unstable.
    pub fn set_controller(&mut self, fb: &'a Framebuffer<'a>, connectors: Vec<Connector<'a>>, mode: Mode) {
        self.framebuffer = Some(fb);
        self.connectors = connectors;

        let connector_ids: Vec<u32> = self.connectors.iter().map(| con | con.id).collect();
        ffi::DrmModeSetCrtc::new(self.device.handle, self.id, fb.id, 0, 0, connector_ids, mode.into());
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

