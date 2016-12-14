#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

mod resources;
mod properties;

pub use self::resources::*;
pub use self::properties::*;

use std::mem;
use std::ptr::null;
use std::os::unix::io::RawFd;
use std::io::Error as IoError;
use libc::{ioctl, c_void};
use ::result::Result;

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
}

pub fn set_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, MACRO_DRM_IOCTL_SET_MASTER, null() as *const c_void);
    Ok(())
}

pub fn drop_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, MACRO_DRM_IOCTL_DROP_MASTER, null() as *const c_void);
    Ok(())
}

#[derive(Debug)]
pub struct DrmModeCreateDumbBuffer {
    pub raw: drm_mode_create_dumb
}

impl DrmModeCreateDumbBuffer {
    pub fn new(fd: RawFd, width: u32, height: u32, bpp: u8) -> Result<DrmModeCreateDumbBuffer> {
        let mut raw: drm_mode_create_dumb = unsafe { mem::zeroed() };
        raw.width = width;
        raw.height = height;
        raw.bpp = bpp as u32;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_CREATE_DUMB, &raw);
        let buffer = DrmModeCreateDumbBuffer { raw: raw };
        Ok(buffer)
    }
}

#[derive(Debug)]
pub struct DrmModeMapDumbBuffer {
    pub raw: drm_mode_map_dumb
}

impl DrmModeMapDumbBuffer {
    pub fn new(fd: RawFd, handle: u32) -> Result<DrmModeMapDumbBuffer> {
        let mut raw: drm_mode_map_dumb = unsafe { mem::zeroed() };
        raw.handle = handle;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let map = DrmModeMapDumbBuffer { raw: raw };
        Ok(map)
    }
}

#[derive(Debug)]
pub struct DrmModeDestroyDumbBuffer {
    pub raw: drm_mode_destroy_dumb
}

impl DrmModeDestroyDumbBuffer {
    pub fn new(fd: RawFd, handle: u32) -> Result<DrmModeDestroyDumbBuffer> {
        let mut raw: drm_mode_destroy_dumb = unsafe { mem::zeroed() };
        raw.handle = handle;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_MAP_DUMB, &raw);
        let destroy = DrmModeDestroyDumbBuffer { raw: raw };
        Ok(destroy)
    }
}

#[test]
fn card_and_plane_resources() {
    use std::fs::{File, OpenOptions};
    use std::os::unix::io::AsRawFd;

    // Open the device
    let device = OpenOptions::new().read(true).write(true).open("/dev/dri/card0").unwrap();

    // Print out the resources and planes
    let resources = DrmModeCardRes::new(device.as_raw_fd()).unwrap();
    let planes = DrmModePlaneRes::new(device.as_raw_fd()).unwrap();
    println!("Resources: {:#?}", resources);
    println!("Planes: {:#?}", planes);

    // Print out a connector
    let &conn_id = resources.connectors.get(0).unwrap();
    let connector = DrmModeGetConnector::new(device.as_raw_fd(), conn_id).unwrap();
    let props = DrmModeObjectGetProperties::new(device.as_raw_fd(), conn_id, DRM_MODE_OBJECT_CONNECTOR);
    println!("Connector: {:#?}", connector);
    println!("Connector props: {:#?}", props);

    let &prop_id = props.unwrap().prop_ids.get(0).unwrap();
    let property = DrmModeGetProperty::new(device.as_raw_fd(), prop_id).unwrap();
    println!("Property: {:#?}", property);

    // Print out an encoder
    let &enc_id = resources.encoders.get(0).unwrap();
    let encoder = DrmModeGetEncoder::new(device.as_raw_fd(), enc_id).unwrap();
    let props = DrmModeObjectGetProperties::new(device.as_raw_fd(), enc_id, DRM_MODE_OBJECT_ENCODER);
    println!("Encoder: {:#?}", encoder);
    println!("Encoder props: {:#?}", props);

    // Print out a connector
    let &crtc_id = resources.crtcs.get(0).unwrap();
    let crtc = DrmModeGetCrtc::new(device.as_raw_fd(), crtc_id).unwrap();
    let props = DrmModeObjectGetProperties::new(device.as_raw_fd(), crtc_id, DRM_MODE_OBJECT_CRTC);
    println!("CRTC: {:#?}", crtc);
    println!("CRTC props: {:#?}", props);

    let &prop_id = props.unwrap().prop_ids.get(0).unwrap();
    let property = DrmModeGetProperty::new(device.as_raw_fd(), prop_id).unwrap();
    println!("Property: {:#?}", property);

    // Print out a plane
    let &plane_id = planes.planes.get(0).unwrap();
    let plane = DrmModeGetPlane::new(device.as_raw_fd(), plane_id).unwrap();
    let props = DrmModeObjectGetProperties::new(device.as_raw_fd(), plane_id, DRM_MODE_OBJECT_PLANE);
    println!("Plane: {:#?}", plane);
    println!("Plane props: {:#?}", props);

    let &prop_id = props.unwrap().prop_ids.get(0).unwrap();
    let property = DrmModeGetProperty::new(device.as_raw_fd(), prop_id).unwrap();
    println!("Property: {:#?}", property);
}

