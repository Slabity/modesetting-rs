use drm_sys::*;
use super::util::*;
use super::result::*;
use super::Device;
use super::MasterDevice;

pub mod buffer;
use self::buffer::*;

use std::ffi::CStr;

/// The length of a GammaLookupTable
pub type GammaLength = u32;

type ConnectorName = [i8; DRM_CONNECTOR_NAME_LEN as usize];
type DisplayName = [i8; DRM_DISPLAY_INFO_LEN as usize];
type ModeName = [i8; DRM_DISPLAY_MODE_LEN as usize];
type PropertyName = [i8; DRM_PROP_NAME_LEN as usize];

/// A trait for devices that provide control (modesetting) functionality.
pub trait Control : Device {
    /// Attempts to read the list of all resource ids.
    fn resource_ids(&self) -> Result<ResourceIds> {
        let mut raw: drm_mode_card_res = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);
        let conns = ffi_buf!(raw.connector_id_ptr, raw.count_connectors);
        let encs = ffi_buf!(raw.encoder_id_ptr, raw.count_encoders);
        let crtcs = ffi_buf!(raw.crtc_id_ptr, raw.count_crtcs);
        let fbs = ffi_buf!(raw.fb_id_ptr, raw.count_fbs);
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);

        let res = ResourceIds {
            connectors: conns,
            encoders: encs,
            crtcs: crtcs,
            framebuffers: fbs,
            width: (raw.min_width, raw.max_width),
            height: (raw.min_height, raw.max_height)
        };

        Ok(res)
    }

    /// Attempts to read the list of all plane ids.
    fn plane_ids(&self) -> Result<PlaneResourceIds> {
        let mut raw: drm_mode_get_plane_res = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &mut raw);
        let planes = ffi_buf!(raw.plane_id_ptr, raw.count_planes);
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &mut raw);

        let res = PlaneResourceIds {
            planes: planes
        };

        Ok(res)
    }

    /// Attempts to get a connector given its id.
    fn connector(&self, id: ConnectorId) -> Result<ConnectorInfo> {
        ConnectorInfo::load_from_device(self, id)
    }

    /// Attempts to get an encoder given its id.
    fn encoder(&self, id: EncoderId) -> Result<EncoderInfo> {
        EncoderInfo::load_from_device(self, id)
    }

    /// Attempts to get a crtc given its id.
    fn crtc(&self, id: CrtcId) -> Result<CrtcInfo> {
        CrtcInfo::load_from_device(self, id)
    }

    /// Attempts to get a framebuffer given its id.
    fn framebuffer(&self, id: FramebufferId) -> Result<FramebufferInfo> {
        FramebufferInfo::load_from_device(self, id)
    }

    /// Attempts to get a plane given its id.
    fn plane(&self, id: PlaneId) -> Result<PlaneInfo> {
        PlaneInfo::load_from_device(self, id)
    }

    /// Attempts to get a Crtc's Gamma Lookup Table (LUT) given its CrtcId.
    //fn gamma<T>(&self, id: &T, len: GammaLength) -> Result<Gamma> where T: AsResourceId<CrtcId> {
    // TODO: Figure out why this won't work:
    //    id.gamma(self, len)
    //}

    // Create a Framebuffer from an object that implements CreateFramebuffer
    fn create_framebuffer<T>(&self, buffer: &T) -> Result<FramebufferId>
        where T: Buffer {
        let mut raw: drm_mode_fb_cmd = Default::default();
        raw.width = buffer.size().0;
        raw.height = buffer.size().1;
        raw.pitch = buffer.pitch();
        raw.bpp = buffer.bpp() as u32;
        raw.depth = buffer.depth();
        raw.handle = buffer.handle().0;
        ioctl!(self, MACRO_DRM_IOCTL_MODE_ADDFB, &mut raw);

        let id = unsafe {
            FramebufferId::from_raw_id(raw.fb_id)
        };

        Ok(id)
    }

    // TODO: Figure out a buffer2 trait?
    fn add_framebuffer2(&self) -> () { unimplemented!() }

    fn remove_framebuffer(&self, id: FramebufferId) -> Result<()> {
        // Need to make a mutable copy of the ID due to the macro requiring a
        // mutable pointer to the object.
        let mut mid = id;
        ioctl!(self, MACRO_DRM_IOCTL_MODE_RMFB, &mut mid);
        Ok(())
    }

    fn dumbbuffer<'a>(&'a self, size: (u16, u16), bpp: u8) ->
        Result<DumbBuffer<'a, Self>> {
            DumbBuffer::new(self, size, bpp)
        }

    // TODO: For atomic modesetting
    fn properties<T>(&self, resource: T) -> Result<ResourcePropertyHandles> where T: ResourceId {
        resource.load_properties(self)
    }

    // TODO: For atomic modesetting
    fn resource_property(&self, handle: PropertyHandle) -> Result<ResourcePropertyInfo> {
        ResourcePropertyInfo::load_from_device(self, handle)
    }

    // TODO: For atomic modesetting
    fn proberty_blob(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn create_property_blob(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn removeproperty_blob(&self) -> () { unimplemented!() }
}

/// A trait for devices that provide control (modesetting) functionality and
/// hold the DRM Master lock.
pub trait MasterControl : MasterDevice + Control {
    /// Applies a Crtc to a Framebuffer and outputs it on a list of Connectors.
    fn set_crtc(&self, crtc: CrtcId, buffer: FramebufferId, connectors:
                &[ConnectorId], position: (u32, u32), mode: Option<Mode>) -> Result<()> {
        let con_ids: Vec<_> = connectors.iter().map(| id |
                                                    id.as_raw_id()).collect();

        println!("{:?}", con_ids);
        let mut raw: drm_mode_crtc = Default::default();
        raw.crtc_id = crtc.as_raw_id();
        raw.fb_id = buffer.as_raw_id();
        raw.set_connectors_ptr = con_ids.as_ptr() as u64;
        raw.count_connectors = con_ids.len() as u32;
        raw.x = position.0;
        raw.y = position.1;
        match mode {
            Some(m) => {
                raw.mode = unsafe { mem::transmute(m) };
                raw.mode_valid = 1;
            },
            None => {
                raw.mode_valid = 0;
            }
        };
        println!("{:#?}", raw);
        ioctl!(self, MACRO_DRM_IOCTL_MODE_SETCRTC, &mut raw);

        Ok(())
    }
}

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

    /// Returns the type of object this ResourceId is connected to.
    fn raw_object_type(&self) -> ObjectInfoType;

    /// Creates and returns a ResourceIdType of this resource.
    fn as_id_type(&self) -> ResourceIdType;

    /// Loads the properties associated with this resource.
    fn load_properties<T>(&self, device: &T) -> Result<ResourcePropertyHandles>
        where T: Control;
}

/// A trait for an object that is owned by a control node.
pub trait ResourceInfo<T> : Sized where T: ResourceId {
    /// Load the info from the provided device.
    fn load_from_device<U>(device: &U, id: T) -> Result<Self> where U: Control;

    /// Get the associated ResourceId
    fn id(&self) -> T;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The set of resource ids that are associated with a DRM device.
pub struct ResourceIds {
    connectors: Array<ConnectorId>,
    encoders: Array<EncoderId>,
    crtcs: Array<CrtcId>,
    framebuffers: Array<FramebufferId>,
    width: (u32, u32),
    height: (u32, u32)
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The set of plane ids that are associated with a DRM device.
pub struct PlaneResourceIds {
    planes: Array<PlaneId>
}

#[derive(Debug, Clone)]
/// A set of properties and their values on a specific resource.
pub struct ResourcePropertyHandles {
    handles: Array<PropertyHandle>
}

impl ResourceIds {
    /// Returns a slice to the list of connector ids.
    pub fn connectors<'a>(&'a self) -> &'a [ConnectorId] {
        &self.connectors
    }

    /// Returns a slice to the list of encoder ids.
    pub fn encoders<'a>(&'a self) -> &'a [EncoderId] {
        &self.encoders
    }

    /// Returns a slice to the list of crtc ids.
    pub fn crtcs<'a>(&'a self) -> &'a [CrtcId] {
        &self.crtcs
    }

    /// Returns a slice to the list of framebuffer ids.
    pub fn framebuffers<'a>(&'a self) -> &'a [FramebufferId] {
        &self.framebuffers
    }

    /// TODO: Learn and document.
    pub fn width(&self) -> (u32, u32) {
        (self.width)
    }

    /// TODO: Learn and document.
    pub fn height(&self) -> (u32, u32) {
        (self.height)

    }

    pub fn filter_crtcs(&self, filter: CrtcListFilter) -> Array<CrtcId> {
        self.crtcs.iter().enumerate().filter(| &(n, _) | {
            (1 << n) & filter.0 != 0
        }).map(| (_, &e) | e).collect()
    }
}

impl PlaneResourceIds {
    /// Returns a slice to the list of plane ids.
    pub fn planes<'a>(&'a self) -> &'a [PlaneId] {
        &self.planes
    }
}

impl ResourcePropertyHandles {
    /// Loads the properties on a particular resource.
    pub fn load_properties<T, U>(device: &T, resource: U) ->
        Result<ResourcePropertyHandles> where T: Control, U: ResourceId {
            let mut raw: drm_mode_obj_get_properties = Default::default();
            raw.obj_id = resource.as_raw_id();
            raw.obj_type = resource.raw_object_type().into();
            ioctl!(device, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);
            let ids: Array<u32> = ffi_buf!(raw.props_ptr, raw.count_props);
            let vals: Array<u64> = ffi_buf!(raw.prop_values_ptr, raw.count_props);
            ioctl!(device, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);

            let parent = resource.as_id_type();

            let handles = ids.into_iter()
                .map(| id | unsafe { PropertyId::from_raw_id(id) })
                .zip(vals.into_iter())
                .map(| (id, val) | {
                    PropertyHandle(parent, id, val as i64)
                })
                .collect();

            let props = ResourcePropertyHandles {
                handles: handles
            };

            Ok(props)
        }

    pub fn handles(&self) -> &[PropertyHandle] {
        &self.handles
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for a Connector.
pub struct ConnectorId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for an Encoder.
pub struct EncoderId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for a Crtc.
pub struct CrtcId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for a Framebuffer.
pub struct FramebufferId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for a Plane.
pub struct PlaneId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A ResourceId for a generic Property on the device.
///
/// The difference between PropertyId and the more specific PropertyHandle
/// is that PropertyId has no associated resource or value. Multiple resources
/// can use the same property, but each resource could have a different value.
/// The PropertyHandle has a value and resource it is associated with.
pub struct PropertyId(RawId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A handle to a generic resource id
pub enum ResourceIdType {
    Connector(ConnectorId),
    Encoder(EncoderId),
    Crtc(CrtcId),
    Framebuffer(FramebufferId),
    Plane(PlaneId),
    Property(PropertyId),
    Unknown
}

impl ResourceId for ConnectorId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> Self{ ConnectorId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Connector }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Connector(*self) }
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            ResourcePropertyHandles::load_properties(device, *self)
        }
}

impl ResourceId for EncoderId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> Self { EncoderId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Encoder }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Encoder(*self) }

    /// Encoders do not have properties that can be loaded.
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            unimplemented!()
        }
}

impl ResourceId for CrtcId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> Self { CrtcId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Crtc }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Crtc(*self) }
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            ResourcePropertyHandles::load_properties(device, *self)
        }
}

impl ResourceId for FramebufferId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> FramebufferId { FramebufferId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Framebuffer }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Framebuffer(*self) }
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            ResourcePropertyHandles::load_properties(device, *self)
        }
}

impl ResourceId for PlaneId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> Self { PlaneId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Plane }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Plane(*self) }
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            ResourcePropertyHandles::load_properties(device, *self)
        }
}

impl ResourceId for PropertyId {
    fn as_raw_id(&self) -> RawId { self.0 }
    unsafe fn from_raw_id(id: RawId) -> Self { PropertyId(id) }
    fn raw_object_type(&self) -> ObjectInfoType { ObjectInfoType::Property }
    fn as_id_type(&self) -> ResourceIdType { ResourceIdType::Property(*self) }
    fn load_properties<T>(&self, device: &T) ->
        Result<ResourcePropertyHandles> where T: Control {
            ResourcePropertyHandles::load_properties(device, *self)
        }
}

#[derive(Debug, Clone)]
pub struct ConnectorInfo {
    id: ConnectorId,
    properties: Array<PropertyId>,
    modes: Array<Mode>,
    encoders: Array<EncoderId>,
    con_type: ConnectorType,
    con_state: ConnectorState,
    // TODO: Subpixel
    // TODO: Subconnector
    size: (u32, u32)
}

#[derive(Debug, Clone)]
pub struct EncoderInfo {
    id: EncoderId,
    crtc_id: CrtcId,
    enc_type: EncoderType,
    possible_crtcs: CrtcListFilter,
    // TODO: possible_clones
}

#[derive(Debug, Clone)]
pub struct CrtcInfo {
    id: CrtcId,
    position: (u32, u32),
    // TODO: mode
    fb: FramebufferId,
    gamma_length: GammaLength
}

#[derive(Debug, Clone)]
pub struct FramebufferInfo {
    id: FramebufferId,
    size: (u32, u32),
    pitch: u32,
    bpp: u32,
    // TODO: Handle?
    depth: u32
}

#[derive(Debug, Clone)]
pub struct PlaneInfo {
    id: PlaneId,
    crtc_id: CrtcId,
    fb_id: FramebufferId,
    // TODO: count_formats,
    // TODO: possible_crtcs
    gamma_length: GammaLength
    // TODO: formats
}

#[derive(Debug, Clone)]
pub struct PropertyInfo {
    id: PropertyId,
    name: PropertyName,
    mutable: bool,
    pending: bool,
    info: PropertyInfoType
}

#[derive(Debug, Clone)]
pub enum ResourceInfoType {
    Connector(ConnectorInfo),
    Encoder(EncoderInfo),
    Crtc(CrtcInfo),
    Framebuffer(FramebufferInfo),
    Plane(PlaneInfo),
    Property(PropertyInfo)
}

impl ResourceInfo<ConnectorId> for ConnectorInfo {
    fn load_from_device<T>(device: &T, id: ConnectorId) -> Result<Self>
        where T: Control {

        let mut raw: drm_mode_get_connector = Default::default();
        raw.connector_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &mut raw);
        // TODO: Figure out properties
        // let props = ffi_buf!(raw.props_ptr, raw.count_props);
        let encs = ffi_buf!(raw.encoders_ptr, raw.count_encoders);
        let modes = ffi_buf!(raw.modes_ptr, raw.count_modes);
        raw.count_props = 0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &mut raw);

        let con = Self {
            id: id,
            properties: Array::new(),
            modes: modes,
            encoders: encs,
            con_type: ConnectorType::from(raw.connector_type),
            con_state: ConnectorState::from(raw.connection),
            size: (raw.mm_width, raw.mm_height)
        };

        Ok(con)
    }

    fn id(&self) -> ConnectorId {
        self.id
    }
}

impl ResourceInfo<EncoderId> for EncoderInfo {
    fn load_from_device<T>(device: &T, id: EncoderId) -> Result<Self>
        where T: Control {

        let mut raw: drm_mode_get_encoder = Default::default();
        raw.encoder_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETENCODER, &mut raw);

        let enc = Self {
            id: id,
            crtc_id: CrtcId(raw.crtc_id),
            enc_type: EncoderType::from(raw.encoder_type),
            possible_crtcs: CrtcListFilter(raw.possible_crtcs)
        };

        Ok(enc)
    }

    fn id(&self) -> EncoderId {
        self.id
    }
}

impl ResourceInfo<CrtcId> for CrtcInfo {
    fn load_from_device<T>(device: &T, id: CrtcId) -> Result<Self>
        where T: Control {

        let mut raw: drm_mode_crtc = Default::default();
        raw.crtc_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETCRTC, &mut raw);

        let crtc = Self {
            id: id,
            position: (raw.x, raw.y),
            fb: FramebufferId(raw.fb_id),
            gamma_length: raw.gamma_size
        };

        Ok(crtc)
    }

    fn id(&self) -> CrtcId {
        self.id
    }
}

impl ResourceInfo<FramebufferId> for FramebufferInfo {
    fn load_from_device<T>(device: &T, id: FramebufferId) -> Result<Self>
        where T: Control {

        let mut raw: drm_mode_fb_cmd = Default::default();
        raw.fb_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETFB, &mut raw);

        let fb = Self {
            id: id,
            size: (raw.width, raw.height),
            pitch: raw.pitch,
            bpp: raw.bpp,
            depth: raw.depth
        };

        Ok(fb)
    }

    fn id(&self) -> FramebufferId {
        self.id
    }
}

impl ResourceInfo<PlaneId> for PlaneInfo {
    fn load_from_device<T>(device: &T, id: PlaneId) -> Result<Self>
        where T: Control {

        let mut raw: drm_mode_get_plane = Default::default();
        raw.plane_id = id.0;
        ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPLANE, &mut raw);

        let plane = Self {
            id: id,
            crtc_id: CrtcId(raw.crtc_id),
            fb_id: FramebufferId(raw.fb_id),
            gamma_length: raw.gamma_size,
        };

        Ok(plane)
    }

    fn id(&self) -> PlaneId {
        self.id
    }
}

impl ResourceInfo<PropertyId> for PropertyInfo {
    fn load_from_device<T>(device: &T, id: PropertyId) ->
        Result<PropertyInfo> where T: Control {

            let mut raw: drm_mode_get_property = Default::default();
            raw.prop_id = id.as_raw_id();
            ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

            let info = PropertyInfo {
                id: id,
                name: raw.name,
                mutable: raw.flags & (DRM_MODE_PROP_IMMUTABLE) == 0,
                pending: raw.flags & (DRM_MODE_PROP_PENDING) == 1,
                info: try!(PropertyInfoType::load_from_device(device, raw))
            };

            Ok(info)
        }

    fn id(&self) -> PropertyId {
        self.id
    }
}

impl ConnectorInfo {
    /// Returns the type of connector this is
    pub fn connector_type(&self) -> ConnectorType {
        self.con_type
    }

    /// Returns the state of this connector.
    pub fn connection_state(&self) -> ConnectorState {
        self.con_state
    }

    /// Returns a slice of each possible mode.
    pub fn modes(&self) -> &[Mode] {
        &self.modes
    }
}

impl EncoderInfo {
    /// Returns the type of encoder this is.
    pub fn encoder_type(&self) -> EncoderType {
        self.enc_type
    }

    /// Returns a CrtcListFilter that can be used to find which Crtc can work
    /// with this Encoder.
    pub fn possible_crtcs(&self) -> CrtcListFilter {
        self.possible_crtcs
    }
}

impl CrtcInfo {
    /// Returns the position the Crtc is attached to.
    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    /// Returns the id of the framebuffer the Crtc is attached to, or None if
    /// not attached.
    pub fn framebuffer(&self) -> Option<FramebufferId> {
        match self.fb.0 {
            0 => None,
            _ => Some(self.fb)
        }
    }
}

impl PropertyInfo {
    /// Returns the name of the property.
    pub fn name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(mem::transmute(&self.name))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The underlying type of connector.
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
    NinePinDIN,
    DisplayPort,
    HDMIA,
    HDMIB,
    TV,
    EmbeddedDisplayPort,
    Virtual,
    DSI,
    DPI
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The state of a connector.
pub enum ConnectorState {
    Connected,
    Disconnected,
    Unknown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The underlying type of encoder.
pub enum EncoderType {
    None,
    DAC,
    TMDS,
    LVDS,
    TVDAC,
    Virtual,
    DSI,
    DPMST,
    DPI
}

#[derive(Debug, Clone, PartialEq, Eq)]
// TODO: Document
pub struct GammaLookupTable {
    pub red: Array<u16>,
    pub green: Array<u16>,
    pub blue: Array<u16>,
}

#[derive(Debug, Clone, Copy)]
/// A filter that can be used with a ResourceIds to determine the set of Crtcs
/// that can attach to a specific encoder.
pub struct CrtcListFilter(u32);

impl From<u32> for ConnectorType {
    fn from(n: u32) -> Self {
        match n {
            DRM_MODE_CONNECTOR_Unknown => ConnectorType::Unknown,
            DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            DRM_MODE_CONNECTOR_Composite => ConnectorType::Composite,
            DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVideo,
            DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            DRM_MODE_CONNECTOR_Component => ConnectorType::Component,
            DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::NinePinDIN,
            DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DisplayPort,
            DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            DRM_MODE_CONNECTOR_eDP => ConnectorType::EmbeddedDisplayPort,
            DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::Virtual,
            DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            _ => ConnectorType::Unknown
        }
    }
}

impl From<u32> for ConnectorState {
    fn from(n: u32) -> Self {
        // These variables are not defined in drm_mode.h for some reason.
        // They were copied from libdrm's xf86DrmMode.h
        match n {
            1 => ConnectorState::Connected,
            2 => ConnectorState::Disconnected,
            _ => ConnectorState::Unknown
        }
    }
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

// TODO: Implement PartialEq and Eq
#[derive(Debug, Clone, Copy)]
pub struct Mode {
    // We're using the FFI struct because the DRM API expects it when giving it
    // to a CRTC or creating a blob from it. Maybe in the future we can look at
    // another option.
    mode: drm_mode_modeinfo
}

impl Mode {
    /// Returns the clock speed of this mode.
    pub fn clock(&self) -> u32 {
        self.mode.clock
    }

    /// Returns the size (resolution) of the mode.
    pub fn size(&self) -> (u16, u16) {
        (self.mode.hdisplay, self.mode.vdisplay)
    }

    /// Returns the horizontal sync start, end, and total.
    pub fn hsync(&self) -> (u16, u16, u16) {
        (self.mode.hsync_start, self.mode.hsync_end, self.mode.htotal)
    }

    /// Returns the vertical sync start, end, and total.
    pub fn vsync(&self) -> (u16, u16, u16) {
        (self.mode.vsync_start, self.mode.vsync_end, self.mode.vtotal)
    }

    /// Returns the horizontal skew of this mode.
    pub fn hskew(&self) -> u16 {
        self.mode.hskew
    }

    /// Returns the vertical scan of this mode.
    pub fn vscan(&self) -> u16 {
        self.mode.vscan
    }

    /// Returns the name of the mode.
    pub fn name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(&self.mode.name as *const _)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawPropertyName([i8; DRM_PROP_NAME_LEN as usize]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawPropertyValue(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A handle for a specific property on a specific resource.
///
/// The difference between PropertyId and the more specific PropertyHandle
/// is that PropertyId has no associated resource or value. Multiple
/// resources can use the same property, but each resource could have a
/// different value. The PropertyHandle has a value and resource it is
/// associated with.
pub struct PropertyHandle(ResourceIdType, PropertyId, i64);

impl PropertyHandle {
    /// Returns the PropertyId associated with this specific handle.
    pub fn id(&self) -> PropertyId {
        self.1
    }

    /// Returns the raw property value within this handle.
    pub fn raw_value(&self) -> i64 {
        self.2
    }

    /// Returns the ResourceId associated with this specific handle.
    pub fn resource_id(&self) -> ResourceIdType {
        self.0
    }

}

#[derive(Debug, Clone)]
/// The information associated with a specific property handle.
pub struct ResourcePropertyInfo {
    handle: PropertyHandle,
    info: PropertyInfo
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The possible values of a particular enum.
pub struct EnumInfo {
    possible: Array<(RawPropertyValue, RawPropertyName)>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The possible values of a particular unsigned range.
pub struct URangeInfo {
    possible: (u64, u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The possible values of a particular signed range.
pub struct IRangeInfo {
    possible: (i64, i64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Generic type for specific information.
pub enum PropertyInfoType {
    Enum(EnumInfo),
    Blob,
    URange(URangeInfo),
    IRange(IRangeInfo),
    Object,
    Unknown
}

impl ResourcePropertyInfo {
    pub fn load_from_device<T>(device: &T, handle: PropertyHandle) ->
        Result<Self> where T: Control {
            let info = try!(PropertyInfo::load_from_device(device, handle.1));

            let res_prop = ResourcePropertyInfo {
                handle: handle,
                info: info
            };

            Ok(res_prop)
        }

    pub fn info(&self) -> &PropertyInfo {
        &self.info
    }
}

impl PropertyInfoType {
    fn load_from_device<T>(device: &T, raw: drm_mode_get_property) -> Result<Self>
        where T: Control {

        let info = if Self::is_enum(raw.flags) {
            // Create an enum
            PropertyInfoType::Enum(EnumInfo::load_from_device(device, raw)?)
        } else if Self::is_blob(raw.flags) {
            PropertyInfoType::Blob
        } else if Self::is_urange(raw.flags) {
            PropertyInfoType::URange(URangeInfo::load_from_device(device, raw)?)
        } else if Self::is_irange(raw.flags) {
            PropertyInfoType::IRange(IRangeInfo::load_from_device(device, raw)?)
        } else if Self::is_object(raw.flags) {
            PropertyInfoType::Object
        } else {
            PropertyInfoType::Unknown
        };

        Ok(info)
    }

    fn is_enum(flag: u32) -> bool {
        flag & (DRM_MODE_PROP_ENUM | DRM_MODE_PROP_BITMASK) != 0
    }

    fn is_blob(flag: u32) -> bool {
        flag & DRM_MODE_PROP_BLOB != 0
    }

    fn is_urange(flag: u32) -> bool {
        flag & DRM_MODE_PROP_RANGE != 0
    }

    fn is_irange(flag: u32) -> bool {
        flag & MACRO_DRM_MODE_PROP_SIGNED_RANGE != 0
    }

    fn is_object(flag: u32) -> bool {
        flag & MACRO_DRM_MODE_PROP_OBJECT != 0
    }
}

impl EnumInfo {
    fn load_from_device<T>(device: &T, mut raw: drm_mode_get_property) ->
        Result<Self> where T: Control {
            let eblob = ffi_buf!(raw.enum_blob_ptr,
                                 raw.count_enum_blobs);

            // We set this to zero because an enum won't fill values_ptr
            // anyways. No need to create a buffer for it.
            raw.count_values = 0;

            ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

            // Collect the enums into a list of EnumPropertyValues
            let enums = eblob.iter().map(| en: &drm_mode_property_enum | {
                let val = RawPropertyValue(en.value as i64);
                let name = RawPropertyName(en.name);
                (val, name)
            }).collect();

            let en = EnumInfo {
                possible: enums
            };

            Ok(en)
        }
}

impl URangeInfo {
    fn load_from_device<T>(device: &T, mut raw: drm_mode_get_property) ->
        Result<Self> where T: Control {
            let values: Array<u64> =
                ffi_buf!(raw.values_ptr, raw.count_values);
            ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);


            let &min = values.get(0).unwrap_or(&0);
            let &max = values.get(1).unwrap_or(&u64::max_value());

            let range = URangeInfo {
                possible: (min, max)
            };

            Ok(range)
        }
}

impl IRangeInfo {
    fn load_from_device<T>(device: &T, mut raw: drm_mode_get_property) ->
        Result<Self> where T: Control {
            let values: Array<i64> =
                ffi_buf!(raw.values_ptr, raw.count_values);
            ioctl!(device, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

            let &min = values.get(0).unwrap_or(&i64::min_value());
            let &max = values.get(1).unwrap_or(&i64::max_value());

            let range = IRangeInfo {
                possible: (min, max)
            };

            Ok(range)
        }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInfoType {
    Connector,
    Encoder,
    Mode,
    Property,
    Framebuffer,
    Blob,
    Plane,
    Crtc,
    Unknown
}

impl From<ObjectInfoType> for u32 {
    fn from(n: ObjectInfoType) -> Self {
        match n {
            ObjectInfoType::Connector => DRM_MODE_OBJECT_CONNECTOR,
            ObjectInfoType::Encoder => DRM_MODE_OBJECT_ENCODER,
            ObjectInfoType::Mode => DRM_MODE_OBJECT_MODE,
            ObjectInfoType::Property => DRM_MODE_OBJECT_PROPERTY,
            ObjectInfoType::Framebuffer => DRM_MODE_OBJECT_FB,
            ObjectInfoType::Blob => DRM_MODE_OBJECT_BLOB,
            ObjectInfoType::Plane => DRM_MODE_OBJECT_PLANE,
            ObjectInfoType::Crtc => DRM_MODE_OBJECT_CRTC,
            ObjectInfoType::Unknown => DRM_MODE_OBJECT_ANY,
        }
    }
}
