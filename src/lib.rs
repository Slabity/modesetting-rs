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

use result::{Result, Error, ErrorKind};

use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::marker::PhantomData;
use std::fs::{File, OpenOptions};
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
pub type BlobId = ResourceId;

#[derive(Debug)]
pub struct Context<T> where T: AsRawFd {
    handle: T,
    driver_name: String,
    driver_date: String,
    driver_desc: String,
    driver_vers: (i32, i32, i32),
    connectors: Vec<Connector>,
    encoders: Vec<Encoder>,
    controllers: Vec<Controller>,
    planes: Vec<Plane>
}

impl<T> AsRawFd for Context<T> where T: AsRawFd {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.borrow().as_raw_fd()
    }
}

impl Context<File> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Context<File>> {
        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        let file = options.open(path)?;
        Self::from_file(file)
    }
}

impl<T> Context<T> where T: AsRawFd {
    pub fn from_file(file: T) -> Result<Context<T>> {
        let fd = file.borrow().as_raw_fd();

        // Get the version information from the handle.
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

        // Enable atomic modesetting
        if let Err(_) = ffi::enable_atomic(fd) {
            let msg = "handle does not support atomic modesetting";
            return Err(ErrorKind::Unsupported(msg).into());
        }

        // Enable universal planes
        if let Err(_) = ffi::enable_universal_planes(fd) {
            let msg = "handle does not support universal planes";
            return Err(ErrorKind::Unsupported(msg).into());
        }

        // Get all static resource ids.
        let cres = ffi::get_card_resources(fd)?;
        let pres = ffi::get_plane_resources(fd)?;

        // Load the connectors
        let connectors = cres.connectors.iter().filter_map(| &id | {
            match ffi::get_connector(fd, id) {
                Ok(c) => {
                    let con = Connector {
                        fd: fd,
                        id: id,
                        con_type: ConnectorType::from(c.raw.connector_type),
                    };

                    Some(con)
                },
                _ => None
            }
        });

        // Load the encoders
        let encoders = cres.encoders.iter().filter_map(| &id | {
            match ffi::get_encoder(fd, id) {
                Ok(_) => {
                    let enc = Encoder {
                        fd: fd,
                        id: id,
                    };

                    Some(enc)
                },
                _ => None
            }
        });

        // Load the controllers
        let controllers = cres.crtcs.iter().filter_map(| &id | {
            match ffi::get_crtc(fd, id) {
                Ok(_) => {
                    let con = Controller {
                        fd: fd,
                        id: id,
                    };

                    Some(con)
                },
                _ => None
            }
        });

        // Load the planes.
        let planes = pres.planes.iter().filter_map(| &id  | {
            match ffi::get_plane(fd, id) {
                Ok(_) => {
                    let pl = Plane {
                        fd: fd,
                        id: id,
                    };

                    Some(pl)
                },
                _ => None
            }
        });

        let mut ctx = Context {
            handle: file,
            driver_name: driver_name,
            driver_date: driver_date,
            driver_desc: driver_desc,
            driver_vers: version.number,
            connectors: connectors.collect(),
            encoders: encoders.collect(),
            controllers: controllers.collect(),
            planes: planes.collect()
        };

        Ok(ctx)
    }

    pub fn connectors(&self) -> &[Connector] { &self.connectors }

    pub fn encoders(&self) -> &[Encoder] { &self.encoders }

    pub fn controllers(&self) -> &[Controller] { &self.controllers }

    pub fn planes(&self) -> &[Plane] { &self.planes }

    pub fn create_framebuffer<'a, B>(&'a self, buffer: &B) -> Result<Framebuffer<'a>> where B: Buffer {
        let fd = self.handle.borrow().as_raw_fd();
        let (width, height) = buffer.borrow().size();
        let raw = try!(ffi::create_framebuffer(fd, width, height, buffer.borrow().pitch(),
                                               buffer.borrow().bpp() as u32,
                                               buffer.borrow().depth() as u32,
                                               buffer.borrow().handle()));
        let fb = Framebuffer {
            _phantom: PhantomData,
            fd: fd,
            id: raw.raw.fb_id,
        };

        Ok(fb)
    }
}

pub struct PropertyUpdate {
    id: ResourceId,
    property: Property
}

pub trait Resource {
    fn get_id(&self) -> ResourceId;
    fn get_properties(&self) -> Result<Vec<Property>>;
}

#[derive(Debug)]
pub struct Connector {
    fd: RawFd,
    id: ConnectorId,
    con_type: ConnectorType,
}

#[derive(Debug)]
pub struct Encoder {
    fd: RawFd,
    id: EncoderId,
}

#[derive(Debug)]
pub struct Controller {
    fd: RawFd,
    id: ControllerId,
}

#[derive(Debug)]
pub struct Framebuffer<'a> {
    _phantom: PhantomData<&'a ()>,
    fd: RawFd,
    id: FramebufferId,
}

#[derive(Debug)]
pub struct Plane {
    fd: RawFd,
    id: PlaneId,
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

impl Connector {
    pub fn connector_type(&self) -> ConnectorType {
        self.con_type
    }

    pub fn connector_state(&self) -> Result<ConnectorState> {
        let raw = ffi::get_connector(self.fd, self.id)?;

        let connection = match raw.raw.connection {
            1 => {
                let subpixel = match raw.raw.subpixel {
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

pub trait PropertyInfo<'a, V, P> where P: 'a {
    fn value(&self) -> V;
    fn possible(&'a self) -> P;
}

#[derive(Debug, Clone)]
pub struct PropertyEnum {
    value: i64,
    possible_values: Vec<(i64, String)>
}

impl<'a> PropertyInfo<'a, i64, &'a [(i64, String)]> for PropertyEnum {
    fn value(&self) -> i64 {
        self.value
    }

    fn possible(&'a self) -> &'a [(i64, String)] {
        &self.possible_values
    }
}

#[derive(Debug, Clone)]
pub struct PropertyBlob {
    id: BlobId,
    data: Vec<u8>
}

impl<'a> PropertyInfo<'a, BlobId, ObjectType> for PropertyBlob {
    fn value(&self) -> BlobId {
        self.id
    }

    fn possible(&'a self) -> ObjectType {
        ObjectType::Blob
    }
}

#[derive(Debug, Clone)]
pub struct PropertyURange {
    value: u64,
    range: (u64, u64)
}

impl<'a> PropertyInfo<'a, u64, (u64, u64)> for PropertyURange {
    fn value(&self) -> u64 {
        self.value
    }

    fn possible(&'a self) -> (u64, u64) {
        self.range
    }
}
#[derive(Debug, Clone)]
pub struct PropertyIRange {
    value: i64,
    range: (i64, i64)
}

impl<'a> PropertyInfo<'a, i64, (i64, i64)> for PropertyIRange {
    fn value(&self) -> i64 {
        self.value
    }

    fn possible(&'a self) -> (i64, i64) {
        self.range
    }
}

#[derive(Debug, Clone)]
pub struct PropertyObject {
    value: ResourceId,
    obj_type: ObjectType
}

impl<'a> PropertyInfo<'a, ResourceId, ObjectType> for PropertyObject {
    fn value(&self) -> ResourceId {
        self.value
    }

    fn possible(&'a self) -> ObjectType {
        self.obj_type
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Connector,
    Encoder,
    Controller,
    Framebuffer,
    Plane,
    Property,
    Mode,
    Blob,
    Unknown
}

#[derive(Debug, Clone)]
pub enum PropertyValue {
    Enum(PropertyEnum),
    Blob(PropertyBlob),
    URange(PropertyURange),
    IRange(PropertyIRange),
    Object(PropertyObject),
    Unknown
}

#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    id: PropertyId,
    value: PropertyValue,
    mutable: bool
}

impl Property {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &PropertyValue {
        &self.value
    }

    pub fn mutable(&self) -> bool {
        self.mutable
    }

    fn get_props(fd: RawFd, id: ResourceId, obj_type: ffi::ObjectType) -> Result<Vec<Property>> {
        let (ids, vals) = match ffi::get_resource_properties(fd, id, obj_type) {
            Ok(p) => (p.prop_ids, p.prop_values),
            Err(_) => (Vec::new(), Vec::new())
        };

        let mut props = Vec::new();
        for (&id, &val) in ids.iter().zip(vals.iter()) {
            match Property::get_property(fd, id, val) {
                Ok(p) => props.push(p),
                Err(Error(e, _)) => match e {
                    e @ ErrorKind::PermissionDenied => bail!(e),
                    _ => continue
                }
            };
        }

        Ok(props)
    }

    fn get_property(fd: RawFd, id: PropertyId, value: u64) -> Result<Property> {
        let property = match ffi::get_property(fd, id, value) {
            Ok(p) => {
                let value = match p.value {
                    ffi::PropertyValue::Enum(e) => {
                        let prop = PropertyEnum {
                            value: value as i64,
                            possible_values: e.enums
                        };

                        PropertyValue::Enum(prop)
                    },
                    ffi::PropertyValue::URange(r) => {
                        let prop = PropertyURange {
                            value: value,
                            range: r.values
                        };

                        PropertyValue::URange(prop)
                    },
                    ffi::PropertyValue::IRange(r) => {
                        let prop = PropertyIRange {
                            value: value as i64,
                            range: r.values
                        };

                        PropertyValue::IRange(prop)
                    },
                    ffi::PropertyValue::Object(o) => {
                        let prop = PropertyObject {
                            obj_type: match o.value {
                                ffi::ObjectType::Connector => ObjectType::Connector,
                                ffi::ObjectType::Encoder => ObjectType::Encoder,
                                ffi::ObjectType::Controller => ObjectType::Controller,
                                ffi::ObjectType::Framebuffer => ObjectType::Framebuffer,
                                ffi::ObjectType::Plane => ObjectType::Plane,
                                ffi::ObjectType::Property => ObjectType::Property,
                                ffi::ObjectType::Mode => ObjectType::Mode,
                                ffi::ObjectType::Blob => ObjectType::Blob,
                                ffi::ObjectType::Unknown => ObjectType::Unknown
                            },
                            value: value as ResourceId
                        };

                        PropertyValue::Object(prop)
                    },
                    ffi::PropertyValue::Blob(b) => {
                        let prop = PropertyBlob {
                            id: b.id as BlobId,
                            data: b.data
                        };

                        PropertyValue::Blob(prop)
                    }
                };
                Property {
                    name: p.name,
                    id: id,
                    value: value,
                    mutable: p.mutable
                }
            },
            Err(Error(e, _)) => match e {
                e @ErrorKind::PermissionDenied => bail!(e),
                _ => {
                    Property {
                        name: "Unknown".to_string(),
                        id: id,
                        value: PropertyValue::Unknown,
                        mutable: false
                    }
                }
            }
        };

        Ok(property)
    }
}

impl Resource for Connector {
    fn get_id(&self) -> ConnectorId { self.id }
    fn get_properties(&self) -> Result<Vec<Property>> {
        Property::get_props(self.fd, self.id, ffi::ObjectType::Connector)
    }
}

impl Resource for Encoder {
    fn get_id(&self) -> EncoderId { self.id }
    fn get_properties(&self) -> Result<Vec<Property>> {
        Property::get_props(self.fd, self.id, ffi::ObjectType::Encoder)
    }
}

impl Resource for Controller {
    fn get_id(&self) -> ControllerId { self.id }
    fn get_properties(&self) -> Result<Vec<Property>> {
        Property::get_props(self.fd, self.id, ffi::ObjectType::Controller)
    }
}

impl<'a> Resource for Framebuffer<'a> {
    fn get_id(&self) -> FramebufferId { self.id }
    fn get_properties(&self) -> Result<Vec<Property>> {
        Property::get_props(self.fd, self.id, ffi::ObjectType::Framebuffer)
    }
}

impl Resource for Plane {
    fn get_id(&self) -> PlaneId { self.id }
    fn get_properties(&self) -> Result<Vec<Property>> {
        Property::get_props(self.fd, self.id, ffi::ObjectType::Plane)
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

