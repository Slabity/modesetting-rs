use drm_sys::*;
use super::super::util::*;
use super::super::result::*;

use std::os::unix::io::{RawFd, AsRawFd};

#[derive(Debug, Clone, Copy)]
pub struct ConnectorId(pub ResourceId);

#[derive(Debug, Clone, Copy)]
pub struct EncoderId(pub ResourceId);

#[derive(Debug, Clone, Copy)]
pub struct CrtcId(pub ResourceId);

#[derive(Debug, Clone, Copy)]
pub struct FramebufferId(pub ResourceId);

#[derive(Debug, Clone, Copy)]
pub struct PlaneId(pub ResourceId);

#[derive(Debug, Clone, Copy)]
pub struct PropertyId(pub ResourceId);

pub type GammaLength = u32;

#[derive(Debug)]
pub struct Connector {
    pub id: ConnectorId,
    pub properties: Array<PropertyId>,
    // TODO: modes
    pub encoders: Array<EncoderId>
}

impl Connector {
    pub fn get<T>(device: &T, id: ConnectorId) -> Result<Connector> where T: AsRawFd {
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

#[derive(Debug)]
pub struct Encoder {
    pub id: EncoderId,
    pub crtc_id: CrtcId,
    // TODO: encoder_type
    // TODO: possible_crtcs,
    // TODO: possible_clones
}

impl Encoder {
    pub fn get<T>(device: &T, id: EncoderId) -> Result<Encoder> where T: AsRawFd {
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

#[derive(Debug)]
pub struct Crtc {
    pub id: CrtcId,
    pub size: (u32, u32),
    // TODO: drm_mode_modeinfo
    pub fb: FramebufferId,
    pub gamma_length: GammaLength
}

impl Crtc {
    pub fn get<T>(device: &T, id: CrtcId) -> Result<Crtc> where T: AsRawFd {
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

#[derive(Debug)]
pub struct Framebuffer {
    pub id: FramebufferId,
    pub size: (u32, u32),
    pub pitch: u32,
    pub bpp: u32,
    // TODO: Handle
    pub depth: u32
}

impl Framebuffer {
    pub fn get<T>(device: &T, id: FramebufferId) -> Result<Framebuffer> where T: AsRawFd {
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

impl Plane {
    pub fn get<T>(device: &T, id: PlaneId) -> Result<Plane> where T: AsRawFd {
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

trait Resource {
    fn resource_type(&self) -> ObjectType;
    fn resource_id(&self) -> ResourceId;
}

impl Resource for Connector {
    fn resource_type(&self) -> ObjectType { ObjectType::Connector }
    fn resource_id(&self) -> ResourceId { self.id.0 }
}

impl Resource for Encoder {
    fn resource_type(&self) -> ObjectType { ObjectType::Encoder }
    fn resource_id(&self) -> ResourceId { self.id.0 }
}

impl Resource for Crtc {
    fn resource_type(&self) -> ObjectType { ObjectType::Crtc }
    fn resource_id(&self) -> ResourceId { self.id.0 }
}

impl Resource for Framebuffer {
    fn resource_type(&self) -> ObjectType { ObjectType::Framebuffer }
    fn resource_id(&self) -> ResourceId { self.id.0 }
}

impl Resource for Plane {
    fn resource_type(&self) -> ObjectType { ObjectType::Plane }
    fn resource_id(&self) -> ResourceId { self.id.0 }
}
