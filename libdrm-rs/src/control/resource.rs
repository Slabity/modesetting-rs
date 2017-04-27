use drm_sys::*;
use super::super::util::*;
use super::super::result::*;

use std::os::unix::io::{RawFd, AsRawFd};

trait AsRawId {
    fn as_raw_id(&self) -> ResourceId { self.0 }
}

pub trait Resource<T, U> where T: AsRawFd, U: AsRawId {
    fn resource_type(&self) -> ObjectType;
    fn get(device: &T, id: U) -> Result<Self>;
}

#[derive(Debug, Copy)]
pub struct ConnectorId(ResourceId);
impl AsRawId for ConnectorId {}

#[derive(Debug, Copy)]
pub struct EncoderId(ResourceId);
impl AsRawId for EncoderId {}

#[derive(Debug, Copy)]
pub struct CrtcId(ResourceId);
impl AsRawId for CrtcId {}

#[derive(Debug, Copy)]
pub struct FramebufferId(ResourceId);
impl AsRawId for FramebufferId {}

#[derive(Debug, Copy)]
pub struct PlaneId(ResourceId);
impl AsRawId for PlaneId {}

#[derive(Debug, Copy)]
pub struct PropertyId(ResourceId);
impl AsRawId for PropertyId {}

pub type GammaLength = u32;

#[derive(Debug)]
pub struct Connector {
    pub id: ConnectorId,
    pub properties: Array<PropertyId>,
    // TODO: modes
    pub encoders: Array<EncoderId>
}

#[derive(Debug)]
pub struct Encoder {
    pub id: EncoderId,
    pub crtc_id: CrtcId,
    // TODO: encoder_type
    // TODO: possible_crtcs,
    // TODO: possible_clones
}

#[derive(Debug)]
pub struct Crtc {
    pub id: CrtcId,
    pub size: (u32, u32),
    // TODO: mode
    pub fb: FramebufferId,
    pub gamma_length: GammaLength
}

#[derive(Debug)]
pub struct Framebuffer {
    pub id: FramebufferId,
    pub size: (u32, u32),
    pub pitch: u32,
    pub bpp: u32,
    // TODO: Handle?
    pub depth: u32
}

#[derive(Debug)]
pub struct Plane {
    pub id: PlaneId,
    pub crtc_id: CrtcId,
    pub fb_id: FramebufferId,
    // TODO: count_formats,
    // TODO: possible_crtcs
    pub gamma_length: GammaLength
    // TODO: formats
}

#[derive(Debug)]
pub struct Gamma {
    pub red: Array<u16>,
    pub green: Array<u16>,
    pub blue: Array<u16>,
}

#[derive(Debug)]
pub enum ObjectType {
    Connector = DRM_MODE_OBJECT_CONNECTOR as isize,
    Encoder = DRM_MODE_OBJECT_ENCODER as isize,
    Crtc = DRM_MODE_OBJECT_CRTC as isize,
    Framebuffer = DRM_MODE_OBJECT_FB as isize,
    Plane = DRM_MODE_OBJECT_PLANE as isize,
    Property = DRM_MODE_OBJECT_PROPERTY as isize,
    Blob = DRM_MODE_OBJECT_BLOB as isize,
    Mode = DRM_MODE_OBJECT_MODE as isize,
    Unknown = DRM_MODE_OBJECT_ANY as isize
}

impl<T> Resource<T, ConnectorId> for Connector where T: AsRawFd {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Connector
    }

    fn get(device: &T, id: ConnectorId) -> Result<Self> {
        let mut raw: drm_mode_get_connector = Default::default();
        raw.connector_id = id.0;
        let props = ffi_buf!(raw.props_ptr, raw.count_props);
        let encs = ffi_buf!(raw.encoders_ptr, raw.count_encoders);
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &mut raw);

        let con = Connector {
            id: id,
            properties: props,
            encoders: encs
        };

        Ok(con)
    }
}

impl<T> Resource<T, EncoderId> for Encoder where T: AsRawFd {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Encoder
    }

    fn get(device: &T, id: EncoderId) -> Result<Self> {
        let mut raw: drm_mode_get_encoder = Default::default();
        raw.encoder_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETENCODER, &mut raw);

        let enc = Encoder {
            id: id,
            crtc_id: CrtcId(raw.crtc_id)
        };

        Ok(enc)
    }
}

impl<T> Resource<T, CrtcId> for Crtc where T: AsRawFd {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Crtc
    }

    fn get(device: &T, id: CrtcId) -> Result<Self> {
        let mut raw: drm_mode_crtc = Default::default();
        raw.crtc_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCRTC, &mut raw);

        let crtc = Crtc {
            id: id,
            size: (raw.x, raw.y),
            fb: FramebufferId(raw.fb_id),
            gamma_length: raw.gamma_size
        };

        Ok(crtc)
    }
}

impl<T> Resource<T, FramebufferId> for Framebuffer where T: AsRawFd {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Framebuffer
    }

    fn get(device: &T, id: FramebufferId) -> Result<Self> {
        let mut raw: drm_mode_fb_cmd = Default::default();
        raw.fb_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETFB, &mut raw);

        let fb = Framebuffer {
            id: id,
            size: (raw.width, raw.height),
            pitch: raw.pitch,
            bpp: raw.bpp,
            depth: raw.depth
        };

        Ok(fb)
    }
}

impl<T> Resource<T, PlaneId> for Plane where T: AsRawFd {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Plane
    }

    fn get(device: &T, id: PlaneId) -> Result<Self> {
        let mut raw: drm_mode_get_plane = Default::default();
        raw.plane_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPLANE, &mut raw);

        let plane = Plane {
            id: id,
            crtc_id: CrtcId(raw.crtc_id),
            fb_id: FramebufferId(raw.fb_id),
            gamma_length: raw.gamma_size,
        };

        Ok(plane)
    }
}
