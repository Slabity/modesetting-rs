use drm_sys::*;
use super::*;
use super::util::*;

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

impl<T> Card<T> where T: AsRawFd {
    pub fn resource_ids(&self) -> Result<ResourceIds> {
        let mut raw: drm_mode_card_res = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);
        ptr_buffers! {
            conns = (&mut raw.connector_id_ptr, raw.count_connectors as usize, SM_SIZE, ConnectorId);
            encs = (&mut raw.encoder_id_ptr, raw.count_encoders as usize, SM_SIZE, EncoderId);
            conts = (&mut raw.crtc_id_ptr, raw.count_crtcs as usize, SM_SIZE, ControllerId);
            fbs = (&mut raw.fb_id_ptr, raw.count_fbs as usize, SM_SIZE, FramebufferId);
        };
        ioctl!(self, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &mut raw);

        let res = ResourceIds {
            conns: conns,
            encs: encs,
            conts: conts,
            fbs: fbs,
            width: (raw.min_width, raw.max_width),
            height: (raw.min_height, raw.max_height)
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct ResourceIds {
    conns: Buffer<ConnectorId>,
    encs: Buffer<EncoderId>,
    conts: Buffer<ControllerId>,
    fbs: Buffer<FramebufferId>,
    width: (u32, u32),
    height: (u32, u32)
}

