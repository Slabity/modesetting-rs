use drm_sys::*;
use super::util::*;
use super::result::*;
use super::DRMDevice;

pub mod resource;
pub mod buffer;
use self::resource::*;
use self::resource::AsResourceId;
use self::buffer::*;

#[derive(Debug)]
/// The set of resource ids that are associated with a DRM device.
pub struct ResourceIds {
    connectors: Array<ConnectorId>,
    encoders: Array<EncoderId>,
    crtcs: Array<CrtcId>,
    framebuffers: Array<FramebufferId>,
    width: (u32, u32),
    height: (u32, u32)
}

#[derive(Debug)]
/// The set of plane ids that are associated with a DRM device.
pub struct PlaneResourceIds {
    planes: Array<PlaneId>
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
}

impl PlaneResourceIds {
    /// Returns a slice to the list of plane ids.
    pub fn planes<'a>(&'a self) -> &'a [PlaneId] {
        &self.planes
    }
}

/// Methods for devices that provide control (modesetting) functionality that do
/// not require being the DRM Master.
pub trait Control : DRMDevice + Sized {
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
    fn connector(&self, id: ConnectorId) -> Result<Connector> {
        Connector::from_device_and_id(self, id)
    }

    /// Attempts to get an encoder given its id.
    fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        Encoder::from_device_and_id(self, id)
    }

    /// Attempts to get a crtc given its id.
    fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        Crtc::from_device_and_id(self, id)
    }

    /// Attempts to get a framebuffer given its id.
    fn framebuffer(&self, id: FramebufferId) -> Result<Framebuffer> {
        Framebuffer::from_device_and_id(self, id)
    }

    /// Attempts to get a plane given its id.
    fn plane(&self, id: PlaneId) -> Result<Plane> {
        Plane::from_device_and_id(self, id)
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

    fn dumbbuffer<'a>(&'a self, size: (u32, u32), bpp: u8) ->
        Result<DumbBuffer<'a, Self>> {
            DumbBuffer::new(self, size, bpp)
        }

    // TODO: For simple displaying
    fn map_dumbbuffer(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn properties(&self) -> Result<()> { unimplemented!() }

    // TODO: For atomic modesetting
    fn property(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn proberty_blob(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn create_property_blob(&self) -> () { unimplemented!() }

    // TODO: For atomic modesetting
    fn removeproperty_blob(&self) -> () { unimplemented!() }
}

pub trait MasterControl : DRMMaster + Control {
}
