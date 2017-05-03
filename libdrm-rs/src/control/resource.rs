use drm_sys::*;
use super::super::util::*;
use super::super::result::*;
use super::Control;

/// A trait for a resource id to be referenced or created by a RawId
pub trait ResourceId {
    /// Extracts the RawId.
    fn as_raw_id(&self) -> RawId;

    /// Creates a ResourceId from a RawId.
    ///
    /// While not actually unsafe, errors will appear that are hard to debug
    /// unless you are certain of what type of object a RawId represents. We
    /// designate this unsafe to ensure the user knows what they're doing.
    unsafe fn from_raw_id(id: RawId) -> Self;
}

/// A trait for extracting a ResourceId from an object.
pub trait AsResourceId<T> where T: ResourceId {
    /// Extracts the ResourceId.
    fn as_resource_id(&self) -> T;
}

/// A trait for an object that is owned by a control node.
pub trait Resource<T, U> : AsResourceId<U> where T: Control, U: ResourceId {
    fn resource_type(&self) -> ObjectType;

    /// Attempts to acquire a handle to the Resource given a Control and
    /// ResourceId.
    fn from_device_and_id(device: &T, id: U) -> Result<Self> where Self: Sized;
}

#[derive(Debug, Clone, Copy)]
/// A ResourceId for a Connector.
pub struct ConnectorId(RawId);

#[derive(Debug, Clone, Copy)]
/// A ResourceId for an Encoder.
pub struct EncoderId(RawId);

#[derive(Debug, Clone, Copy)]
/// A ResourceId for a Crtc.
pub struct CrtcId(RawId);

#[derive(Debug, Clone, Copy)]
/// A ResourceId for a Framebuffer.
pub struct FramebufferId(RawId);

#[derive(Debug, Clone, Copy)]
/// A ResourceId for a Plane.
pub struct PlaneId(RawId);

#[derive(Debug, Clone, Copy)]
/// A ResourceId for a Property.
pub struct PropertyId(RawId);

impl ResourceId for ConnectorId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> ConnectorId { ConnectorId(id) }
}

impl AsResourceId<Self> for ConnectorId {
    fn as_resource_id(&self) -> Self { *self }
}

impl ResourceId for EncoderId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> EncoderId { EncoderId(id) }
}

impl AsResourceId<Self> for EncoderId {
    fn as_resource_id(&self) -> Self { *self }
}

impl ResourceId for CrtcId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> CrtcId { CrtcId(id) }
}

impl AsResourceId<Self> for CrtcId {
    fn as_resource_id(&self) -> Self { *self }
}

impl AsResourceId<CrtcId> {
    /// Given the length of the Gamma's Lookup Table (LUT), attempt to acquire
    /// the Gamma value of the Crtc represented by the CrtcId.
    pub fn gamma<T>(&self, dev: &T, len: GammaLength) -> Result<Gamma>
        where T: Control {
        let mut raw: drm_mode_crtc_lut = Default::default();
        raw.crtc_id = self.as_resource_id().as_raw_id();
        raw.gamma_size = len;
        let red = ffi_buf!(raw.red, len);
        let green = ffi_buf!(raw.green, len);
        let blue = ffi_buf!(raw.blue, len);
        ioctl!(dev, MACRO_DRM_IOCTL_MODE_GETGAMMA, &mut raw);

        let gamma = Gamma {
            red: red,
            green: green,
            blue: blue,
        };

        Ok(gamma)
    }
}

impl ResourceId for FramebufferId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> FramebufferId { FramebufferId(id) }
}

impl AsResourceId<Self> for FramebufferId {
    fn as_resource_id(&self) -> Self { *self }
}

impl ResourceId for PlaneId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> PlaneId { PlaneId(id) }
}

impl AsResourceId<Self> for PlaneId {
    fn as_resource_id(&self) -> Self { *self }
}

impl ResourceId for PropertyId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> PropertyId{ PropertyId(id) }
}

impl AsResourceId<Self> for PropertyId {
    fn as_resource_id(&self) -> Self { *self }
}

pub type GammaLength = u32;

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

#[derive(Debug)]
/// The underlying type of encoder.
pub enum EncoderType {
    None = DRM_MODE_ENCODER_NONE as isize,
    DAC = DRM_MODE_ENCODER_DAC as isize,
    TMDS = DRM_MODE_ENCODER_TMDS as isize,
    LVDS = DRM_MODE_ENCODER_LVDS as isize,
    TVDAC = DRM_MODE_ENCODER_TVDAC as isize,
    Virtual = DRM_MODE_ENCODER_VIRTUAL as isize,
    DSI = DRM_MODE_ENCODER_DSI as isize,
    DPMST = DRM_MODE_ENCODER_DPMST as isize,
    DPI = DRM_MODE_ENCODER_DPI as isize
}

impl From<u32> for EncoderType {
    fn from(n: u32) -> Self {
        match n {
            DRM_MODE_ENCODER_NONE => EncoderType::None,
            DRM_MODE_ENCODER_DAC => EncoderType::DAC,
            DRM_MODE_ENCODER_TMDS => EncoderType::TMDS,
            DRM_MODE_ENCODER_LVDS => EncoderType::LVDS,
            DRM_MODE_ENCODER_TVDAC => EncoderType::TVDAC,
            DRM_MODE_ENCODER_VIRTUAL => EncoderType::Virtual,
            DRM_MODE_ENCODER_DSI => EncoderType::DSI,
            DRM_MODE_ENCODER_DPMST => EncoderType::DPMST,
            DRM_MODE_ENCODER_DPI => EncoderType::DPI,
            _ => EncoderType::None
        }
    }
}

#[derive(Debug)]
pub struct Connector {
    id: ConnectorId,
    properties: Array<PropertyId>,
    // TODO: modes
    encoders: Array<EncoderId>,
    size: (u32, u32)
}

#[derive(Debug)]
pub struct Encoder {
    id: EncoderId,
    crtc_id: CrtcId,
    enc_type: EncoderType,
    // TODO: possible_crtcs,
    // TODO: possible_clones
}

#[derive(Debug)]
pub struct Crtc {
    id: CrtcId,
    size: (u32, u32),
    // TODO: mode
    fb: FramebufferId,
    gamma_length: GammaLength
}

#[derive(Debug)]
pub struct Framebuffer {
    id: FramebufferId,
    size: (u32, u32),
    pitch: u32,
    bpp: u32,
    // TODO: Handle?
    depth: u32
}

#[derive(Debug)]
pub struct Plane {
    id: PlaneId,
    crtc_id: CrtcId,
    fb_id: FramebufferId,
    // TODO: count_formats,
    // TODO: possible_crtcs
    gamma_length: GammaLength
    // TODO: formats
}

#[derive(Debug)]
pub struct Gamma {
    pub red: Array<u16>,
    pub green: Array<u16>,
    pub blue: Array<u16>,
}

impl AsResourceId<ConnectorId> for Connector {
    fn as_resource_id(&self) -> ConnectorId {
        self.id
    }
}

impl<T> Resource<T, ConnectorId> for Connector where T: Control {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Connector
    }

    fn from_device_and_id(device: &T, id: ConnectorId) -> Result<Self> {
        let mut raw: drm_mode_get_connector = Default::default();
        raw.connector_id = id.0;
        let props = ffi_buf!(raw.props_ptr, raw.count_props);
        let encs = ffi_buf!(raw.encoders_ptr, raw.count_encoders);
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &mut raw);

        let con = Connector {
            id: id,
            properties: props,
            encoders: encs,
            size: (raw.mm_width, raw.mm_height)
        };

        Ok(con)
    }
}

impl AsResourceId<EncoderId> for Encoder {
    fn as_resource_id(&self) -> EncoderId {
        self.id
    }
}

impl<T> Resource<T, EncoderId> for Encoder where T: Control {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Encoder
    }

    fn from_device_and_id(device: &T, id: EncoderId) -> Result<Self> {
        let mut raw: drm_mode_get_encoder = Default::default();
        raw.encoder_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETENCODER, &mut raw);

        let enc = Encoder {
            id: id,
            crtc_id: CrtcId(raw.crtc_id),
            enc_type: EncoderType::from(raw.encoder_type)
        };

        Ok(enc)
    }
}

impl AsResourceId<CrtcId> for Crtc {
    fn as_resource_id(&self) -> CrtcId {
        self.id
    }
}

impl<T> Resource<T, CrtcId> for Crtc where T: Control {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Crtc
    }

    fn from_device_and_id(device: &T, id: CrtcId) -> Result<Self> {
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

impl AsResourceId<FramebufferId> for Framebuffer {
    fn as_resource_id(&self) -> FramebufferId {
        self.id
    }
}

impl<T> Resource<T, FramebufferId> for Framebuffer where T: Control {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Framebuffer
    }

    fn from_device_and_id(device: &T, id: FramebufferId) -> Result<Self> {
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

impl AsResourceId<PlaneId> for Plane {
    fn as_resource_id(&self) -> PlaneId {
        self.id
    }
}

impl<T> Resource<T, PlaneId> for Plane where T: Control {
    fn resource_type(&self) -> ObjectType {
        ObjectType::Plane
    }

    fn from_device_and_id(device: &T, id: PlaneId) -> Result<Self> {
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
