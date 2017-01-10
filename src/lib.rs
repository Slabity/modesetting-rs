/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block handle and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a UnprivilegedDevice and using four types of resources:

  - Connectors: The physical interfaces on your GPU, such as HDMI, VGA, and
  LVDS ports.
  - Encoders: These modify and deliver the pixel data to the connectors.
  - Display Controllers: Controls the scanout of a Framebuffer to one or more
  Connectos.
  - Framebuffer: Pixel data that can be used by a Display Controller

  The standard procedure to do this is to first open the handle and select the
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
pub mod mode;
pub mod property;

use result::{Result, Error, ErrorKind};
use property::*;

use std::path::Path;
use std::os::unix::io::{AsRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::rc::{Rc, Weak};
use std::borrow::Borrow;

#[cfg(feature="dumbbuffer")]
mod dumbbuffer;

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;
pub type PlaneId = ResourceId;
pub type PropertyId = ResourceId;
pub type ModeId = ResourceId;
pub type BlobId = ResourceId;

#[derive(Debug, Clone)]
struct Device {
    device: Rc<File>,
    driver: DriverVersion
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.device.as_raw_fd()
    }
}

impl Device {
    fn from_file(file: File) -> Result<Device> {
        let fd = file.as_raw_fd();

        let version = match ffi::get_version(fd) {
            Ok(v) => v,
            Err(Error(e, _)) => match e {
                ErrorKind::InvalidVersion => bail!(ErrorKind::InvalidVersion),
                _ => bail!(ErrorKind::InvalidNode)
            }
        };

        let driver_name = match version.name.into_string() {
            Ok(n) => n,
            Err(_) => "Unknown".to_string()
        };
        let driver_date = match version.date.into_string() {
            Ok(n) => n,
            Err(_) => "Unknown".to_string()
        };
        let driver_desc = match version.desc.into_string() {
            Ok(n) => n,
            Err(_) => "Unknown".to_string()
        };

        let driver = DriverVersion {
            driver_name: driver_name,
            driver_date: driver_date,
            driver_desc: driver_desc,
            driver_vers: version.number,
        };

        if let Err(_) = ffi::enable_atomic(fd) {
            let msg = "handle does not support atomic modesetting";
            return Err(ErrorKind::Unsupported(msg).into());
        }

        if let Err(_) = ffi::enable_universal_planes(fd) {
            let msg = "handle does not support universal planes";
            return Err(ErrorKind::Unsupported(msg).into());
        }

        let device = Device {
            device: Rc::new(file),
            driver: driver
        };

        Ok(device)
    }
}

#[derive(Debug, Clone)]
pub struct DriverVersion {
    driver_name: String,
    driver_date: String,
    driver_desc: String,
    driver_vers: (i32, i32, i32),
}

#[derive(Debug)]
pub struct Context {
    device: Device,
    connectors: Vec<Connector>,
    encoders: Vec<Encoder>,
    controllers: Vec<Controller>,
    planes: Vec<Plane>
}

impl Context {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Context> {
        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        let file = options.open(path)?;
        Self::from_file(file)
    }

    pub fn from_file(file: File) -> Result<Context> {
        let device = Device::from_file(file)?;
        let fd = device.as_raw_fd();
        let weak = Rc::downgrade(&device.device);

        // Get all static resource ids.
        let cres = ffi::get_card_resources(fd)?;
        let pres = ffi::get_plane_resources(fd)?;

        // Load the connectors
        let connectors = cres.connectors.iter().filter_map(| &id | {
            match ffi::get_connector(fd, id) {
                Ok(c) => {
                    let con = Connector {
                        device: (&weak).clone(),
                        id: id,
                        data: ConnectorType::from(c.raw.connector_type),
                    };

                    Some(con)
                },
                _ => None
            }
        }).collect();

        // Load the encoders
        let encoders = cres.encoders.iter().filter_map(| &id | {
            match ffi::get_encoder(fd, id) {
                Ok(_) => {
                    let enc = Encoder {
                        device: (&weak).clone(),
                        id: id,
                        data: ()
                    };

                    Some(enc)
                },
                _ => None
            }
        }).collect();

        // Load the controllers
        let controllers = cres.crtcs.iter().filter_map(| &id | {
            match ffi::get_crtc(fd, id) {
                Ok(_) => {
                    let con = Controller {
                        device: (&weak).clone(),
                        id: id,
                        data: ()
                    };

                    Some(con)
                },
                _ => None
            }
        }).collect();

        // Load the planes.
        let planes = pres.planes.iter().filter_map(| &id  | {
            match ffi::get_plane(fd, id) {
                Ok(_) => {
                    let pl = Plane {
                        device: (&weak).clone(),
                        id: id,
                        data: ()
                    };

                    Some(pl)
                },
                _ => None
            }
        }).collect();

        let ctx = Context {
            device: device,
            connectors: connectors,
            encoders: encoders,
            controllers: controllers,
            planes: planes
        };

        Ok(ctx)
    }

    pub fn connectors(&self) -> &[Connector] { &self.connectors }

    pub fn encoders(&self) -> &[Encoder] { &self.encoders }

    pub fn controllers(&self) -> &[Controller] { &self.controllers }

    pub fn planes(&self) -> &[Plane] { &self.planes }

    pub fn create_framebuffer<B>(&self, buffer: &B) -> Result<Framebuffer> where B: Buffer {
        let fd = self.device.as_raw_fd();
        let (width, height) = buffer.borrow().size();
        let raw = try!(ffi::create_framebuffer(fd, width, height, buffer.borrow().pitch(),
                                               buffer.borrow().bpp() as u32,
                                               buffer.borrow().depth() as u32,
                                               buffer.borrow().handle()));
        let fb = Framebuffer {
            device: Rc::downgrade(&self.device.device),
            id: raw.raw.fb_id,
            data: ()
        };

        Ok(fb)
    }

    pub fn commit<'a, T>(&self, updates: T) -> Result<()>
        where T: Iterator<Item=&'a PropertyUpdate> {
        let fd = self.device.as_raw_fd();
        let updates: Vec<_> = updates.map(| u | *u).collect();

        let objs = updates.iter().map(| u | u.resource as u32).collect();
        let props = updates.iter().map(| u | u.property as u32).collect();
        let vals = updates.iter().map(| u | u.value as u64).collect();

        ffi::atomic_commit(fd, objs, props, vals)
    }

    fn get_props(fd: RawFd, id: ResourceId, obj_type: ffi::ObjectType) -> Result<Vec<Value>> {
        let (ids, vals) = match ffi::get_resource_properties(fd, id, obj_type) {
            Ok(p) => (p.prop_ids, p.prop_values),
            Err(_) => (Vec::new(), Vec::new())
        };

        let mut props = Vec::new();
        for (&prop_id, &val) in ids.iter().zip(vals.iter()) {
            match Self::get_property(fd, id, prop_id, val) {
                Ok(p) => props.push(p),
                Err(Error(e, _)) => match e {
                    e @ ErrorKind::PermissionDenied => bail!(e),
                    _ => continue
                }
            };
        }

        Ok(props)
    }

    fn get_property(fd: RawFd, res: ResourceId, id: PropertyId, value: u64) -> Result<Value> {
        match ffi::get_property(fd, id, value) {
            Ok(p) => Ok(Value::from((res, p))),
            Err(Error(e, _)) => match e {
                e @ ErrorKind::PermissionDenied => bail!(e),
                _ => {
                    Ok(Value::Unknown)
                }
            }
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

#[derive(Debug)]
pub struct Resource<T> {
    device: Weak<File>,
    id: ResourceId,
    data: T
}

impl<T> Resource<T> {
    pub fn id(&self) -> ResourceId { self.id }

    pub fn properties(&self) -> Result<Vec<Value>> {
        let upgraded = Weak::upgrade(&self.device).unwrap();
        let fd = upgraded.as_raw_fd();
        Context::get_props(fd, self.id, ffi::ObjectType::Unknown)
    }
}

pub type Connector = Resource<ConnectorType>;
pub type Encoder = Resource<()>;
pub type Controller = Resource<()>;
pub type Framebuffer = Resource<()>;
pub type Plane = Resource<()>;

impl Connector {
    pub fn connector_type(&self) -> ConnectorType {
        self.data
    }

    pub fn connector_state(&self) -> Result<ConnectorState> {
        let upgraded = Weak::upgrade(&self.device).unwrap();
        let fd = upgraded.as_raw_fd();

        let raw = ffi::get_connector(fd, self.id)?;

        let connection = match raw.raw.connection {
            1 => {
                let subpixel = match raw.raw.subpixel {
                    1 => SubPixelType::Unknown,
                    2 => SubPixelType::HorizontalRGB,
                    3 => SubPixelType::HorizontalBGR,
                    4 => SubPixelType::VerticalRGB,
                    5 => SubPixelType::VerticalBGR,
                    6 => SubPixelType::None,
                    _ => SubPixelType::Unknown
                };

                ConnectorState::Connected(subpixel, (raw.raw.mm_width, raw.raw.mm_height))
            },
            2 => ConnectorState::Disconnected,
            _ => ConnectorState::Unknown
        };

        Ok(connection)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectorType {
    Unknown,
    VGA,
    DVII,
    DVID,
    DVIA,
    Composite,
    SVideo,
    LVDS,
    Component,
    DIN,
    DisplayPort,
    HDMIA,
    HDMIB,
    TV,
    EmbeddedDP,
    Virtual,
    DSI,
    DPI
}

impl From<u32> for ConnectorType {
    fn from(ffi_type: u32) -> ConnectorType {
        match ffi_type {
            ffi::DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            ffi::DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            ffi::DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            ffi::DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            ffi::DRM_MODE_CONNECTOR_Composite => ConnectorType::Composite,
            ffi::DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVideo,
            ffi::DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            ffi::DRM_MODE_CONNECTOR_Component => ConnectorType::Component,
            ffi::DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::DIN,
            ffi::DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DisplayPort,
            ffi::DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            ffi::DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            ffi::DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            ffi::DRM_MODE_CONNECTOR_eDP => ConnectorType::EmbeddedDP,
            ffi::DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::Virtual,
            ffi::DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            ffi::DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            _ => ConnectorType::Unknown
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectorState {
    Connected(SubPixelType, (u32, u32)),
    Disconnected,
    Unknown
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubPixelType {
    Unknown,
    HorizontalRGB,
    HorizontalBGR,
    VerticalRGB,
    VerticalBGR,
    None
}

#[derive(Debug, Copy, Clone)]
pub struct PropertyUpdate {
    resource: ResourceId,
    property: PropertyId,
    value: i64
}

