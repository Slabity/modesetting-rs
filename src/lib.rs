/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block device and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a UnprivilegedDevice and using four types of resources:

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
pub mod mode;

#[cfg(feature="dumbbuffer")]
pub mod dumbbuffer;

use result::Result;

use std::os::unix::io::{AsRawFd, RawFd};
use std::marker::PhantomData;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::borrow::Borrow;
use std::mem::transmute;

pub type ResourceId = u32;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConnectorId(ResourceId);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EncoderId(ResourceId);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ControllerId(ResourceId);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FramebufferId(ResourceId);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlaneId(ResourceId);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PropertyId(ResourceId);

/// A `Device` is a privileged handle to the character device that provides
/// modesetting capabilities.
#[derive(Debug)]
pub struct Device<T>(T) where T: Borrow<File>;

impl<T> AsRawFd for Device<T> where T: Borrow<File> {
    fn as_raw_fd(&self) -> RawFd {
        self.0.borrow().as_raw_fd()
    }
}

impl Device<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device<File>> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        ffi::enable_universal_planes(file.as_raw_fd());
        ffi::enable_atomic(file.as_raw_fd());
        Ok(Device(file))
    }
}

impl<T> Device<T> where T: Borrow<File> {
    pub fn resources(&self) -> Result<Resources> {
        let card = try!(ffi::DrmModeCardRes::new(self.as_raw_fd()));
        let planes = try!(ffi::DrmModePlaneRes::new(self.as_raw_fd()));
        let res = unsafe {
            Resources {
                connectors: transmute(card.connectors),
                encoders: transmute(card.encoders),
                controllers: transmute(card.crtcs),
                framebuffers: transmute(card.framebuffers),
                planes: transmute(planes.planes)
            }
        };
        Ok(res)
    }

    pub fn connector<'a>(&'a self, id: ConnectorId) -> Result<Connector<'a>> {
        let con = Connector {
            _phantom: PhantomData,
            device_fd: self.as_raw_fd(),
            id: id
        };
        Ok(con)
    }

    pub fn encoder<'a>(&'a self, id: EncoderId) -> Result<Encoder<'a>> {
        let enc = Encoder {
            _phantom: PhantomData,
            device_fd: self.as_raw_fd(),
            id: id
        };
        Ok(enc)
    }

    pub fn controller<'a>(&'a self, id: ControllerId) -> Result<Controller<'a>> {
        let con = Controller {
            _phantom: PhantomData,
            device_fd: self.as_raw_fd(),
            id: id
        };
        Ok(con)
    }

    pub fn plane<'a>(&'a self, id: PlaneId) -> Result<Plane<'a>> {
        let plane = Plane {
            _phantom: PhantomData,
            device_fd: self.as_raw_fd(),
            id: id
        };
        Ok(plane)
    }

    pub fn property<'a>(&'a self, id: PropertyId) -> Result<Property<'a>> {
        let ffi = try!(ffi::DrmModeGetProperty::new(self.as_raw_fd(), id.0));

        let name = unsafe {
            let cstr = std::ffi::CStr::from_ptr(&ffi.raw.name as *const _);
            cstr.to_str().unwrap().to_string()
        };

        let value = match ffi.values {
            // Value is an enum. Generate the enum values.
            ffi::DrmModePropertyValues::Enum(e) => {
                // Collect each possible enum value.
                let values = e.enums.iter().map(| &en | {
                    let name = unsafe {
                        let cstr = std::ffi::CStr::from_ptr(&en.name as *const _);
                        cstr.to_str().unwrap().to_string()
                    };
                    let value = en.value;
                    (value, name)
                }).collect();

                // Current value is...?
                let current = 0;

                let prop_enum = PropertyEnum {
                    value: current,
                    possible_values: values
                };

                PropertyType::Enum(prop_enum)
            },
            ffi::DrmModePropertyValues::Blob(b) => {
                let prop_blob = PropertyBlob {
                    values: b.values,
                    blobs: b.blob
                };

                PropertyType::Blob(prop_blob)
            },
            ffi::DrmModePropertyValues::Range(r) => {
                let &min = r.values.get(0).unwrap();
                let &max = r.values.get(1).unwrap();

                let prop_range = PropertyRange {
                    value: 0,
                    range: (min, max)
                };

                PropertyType::Range(prop_range)
            },
            ffi::DrmModePropertyValues::Object(r) => {
                let prop_obj = PropertyObject {
                    value: 0,
                };

                PropertyType::Object(prop_obj)
            }
        };

        let prop = Property {
            _phantom: PhantomData,
            name: name,
            id: id,
            value: value
        };
        Ok(prop)
    }
}

pub trait Resource {
    fn get_property_ids(&self) -> Result<Vec<PropertyId>>;
}

#[derive(Debug)]
pub struct Resources {
    pub connectors: Vec<ConnectorId>,
    pub encoders: Vec<EncoderId>,
    pub controllers: Vec<ControllerId>,
    pub framebuffers: Vec<FramebufferId>,
    pub planes: Vec<PlaneId>
}

#[derive(Debug)]
pub struct Connector<'a> {
    _phantom: PhantomData<&'a ()>,
    device_fd: RawFd,
    id: ConnectorId,
}

#[derive(Debug)]
pub struct Encoder<'a> {
    _phantom: PhantomData<&'a ()>,
    device_fd: RawFd,
    id: EncoderId,
}

#[derive(Debug)]
pub struct Controller<'a> {
    _phantom: PhantomData<&'a ()>,
    device_fd: RawFd,
    id: ControllerId,
}

#[derive(Debug)]
pub struct Plane<'a> {
    _phantom: PhantomData<&'a ()>,
    device_fd: RawFd,
    id: PlaneId,
}

#[derive(Debug)]
pub struct PropertyEnum {
    value: u64,
    possible_values: Vec<(u64, String)>
}

#[derive(Debug)]
// Unsure how to handle blobs. Just using this representation for now.
pub struct PropertyBlob {
    values: Vec<u32>,
    blobs: Vec<u32>
}

#[derive(Debug)]
pub struct PropertyRange {
    value: u64,
    range: (u64, u64)
}

#[derive(Debug)]
pub struct PropertyObject {
    value: u64
}

#[derive(Debug)]
pub enum PropertyType {
    Enum(PropertyEnum),
    Blob(PropertyBlob),
    Range(PropertyRange),
    Object(PropertyObject)
}

#[derive(Debug)]
pub struct Property<'a> {
    _phantom: PhantomData<&'a ()>,
    name: String,
    id: PropertyId,
    value: PropertyType
}

impl<'a> Resource for Connector<'a> {
    fn get_property_ids(&self) -> Result<Vec<PropertyId>> {
        let ffi = try!(ffi::DrmModeObjectGetProperties::new(self.device_fd, self.id.0, ffi::DRM_MODE_OBJECT_CONNECTOR));
        let prop = unsafe {
            transmute(ffi.prop_ids)
        };
        Ok(prop)
    }
}

impl<'a> Resource for Encoder<'a> {
    fn get_property_ids(&self) -> Result<Vec<PropertyId>> {
        let ffi = try!(ffi::DrmModeObjectGetProperties::new(self.device_fd, self.id.0, ffi::DRM_MODE_OBJECT_ENCODER));
        let prop = unsafe {
            transmute(ffi.prop_ids)
        };
        Ok(prop)
    }
}

impl<'a> Resource for Controller<'a> {
    fn get_property_ids(&self) -> Result<Vec<PropertyId>> {
        let ffi = try!(ffi::DrmModeObjectGetProperties::new(self.device_fd, self.id.0, ffi::DRM_MODE_OBJECT_CRTC));
        let prop = unsafe {
            transmute(ffi.prop_ids)
        };
        Ok(prop)
    }
}

impl<'a> Resource for Plane<'a> {
    fn get_property_ids(&self) -> Result<Vec<PropertyId>> {
        let ffi = try!(ffi::DrmModeObjectGetProperties::new(self.device_fd, self.id.0, ffi::DRM_MODE_OBJECT_PLANE));
        let prop = unsafe {
            transmute(ffi.prop_ids)
        };
        Ok(prop)
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

