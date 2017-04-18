use drm_sys::*;
use super::util::*;
use super::result::*;
use super::DRMDevice;
use std::os::unix::io::{RawFd, AsRawFd};

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

#[derive(Debug)]
pub struct ResourceIds {
    conns: Buffer<ConnectorId>,
    encs: Buffer<EncoderId>,
    conts: Buffer<ControllerId>,
    fbs: Buffer<FramebufferId>,
    width: (u32, u32),
    height: (u32, u32)
}

pub trait Control : DRMDevice {
    fn resource_ids(&self) -> Result<ResourceIds> {
        let mut raw: drm_mode_card_res = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);

        let conn_cnt = raw.count_connectors as usize;
        let enc_cnt = raw.count_encoders as usize;
        let crtc_cnt = raw.count_crtcs as usize;
        let fb_cnt = raw.count_fbs as usize;

        ptr_buffers! {
            conns = (raw.connector_id_ptr, conn_cnt, ConnectorId);
            encs = (raw.encoder_id_ptr, enc_cnt, EncoderId);
            crtcs = (raw.crtc_id_ptr, crtc_cnt, ControllerId);
            fbs = (raw.fb_id_ptr, fb_cnt, FramebufferId);
        };
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);

        let res = ResourceIds {
            conns: conns,
            encs: encs,
            conts: crtcs,
            fbs: fbs,
            width: (raw.min_width, raw.max_width),
            height: (raw.min_height, raw.max_height)
        };

        Ok(res)
    }

    fn plane_resource_ids(&self) -> () { unimplemented!() }
    fn crtc(&self) -> () { unimplemented!() }
    fn plane(&self) -> () { unimplemented!() }
    fn gamma(&self) -> () { unimplemented!() }
    fn encoder(&self) -> () { unimplemented!() }
    fn connector(&self) -> () { unimplemented!() }
    fn property(&self) -> () { unimplemented!() }
    fn proberty_blob(&self) -> () { unimplemented!() }
    fn framebuffer(&self) -> () { unimplemented!() }
    fn add_framebuffer(&self) -> () { unimplemented!() }
    fn add_framebuffer2(&self) -> () { unimplemented!() }
    fn remove_framebuffer(&self) -> () { unimplemented!() }
    fn create_dumbbuffer(&self) -> () { unimplemented!() }
    fn map_dumbbuffer(&self) -> () { unimplemented!() }
    fn remove_dumbbuffer(&self) -> () { unimplemented!() }
    fn properties(&self) -> () { unimplemented!() }
    fn create_property_blob(&self) -> () { unimplemented!() }
    fn remove_property_blob(&self) -> () { unimplemented!() }
}
