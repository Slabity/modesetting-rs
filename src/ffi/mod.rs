#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

use std::io::Error as IoError;
use std::mem;
use ::result::{Result, ErrorKind};
use std::os::unix::io::RawFd;
use std::ptr::null;
use libc::{ioctl, c_void};

// This macro simply wraps the ioctl call to return errno on failure
macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
}

pub fn set_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, DRM_IOCTL_SET_MASTER, null() as *const c_void);
    Ok(())
}

pub fn drop_master(fd: RawFd) -> Result<()> {
    ioctl!(fd, DRM_IOCTL_DROP_MASTER, null() as *const c_void);
    Ok(())
}

#[derive(Debug)]
pub struct DrmModeCardRes {
    pub raw: drm_mode_card_res,
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub framebuffers: Vec<u32>
}

impl DrmModeCardRes {
    pub fn new(fd: RawFd) -> Result<DrmModeCardRes> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_card_res = unsafe { mem::zeroed() };
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &raw);

        // Create buffers for each array
        let mut connectors: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_connectors as usize];
        let mut encoders: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_encoders as usize];
        let mut crtcs: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_crtcs as usize];
        let mut framebuffers: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_fbs as usize];

        // Pass a handle to the buffers to the raw struct
        raw.connector_id_ptr = connectors.as_mut_slice().as_mut_ptr() as u64;
        raw.encoder_id_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.crtc_id_ptr = crtcs.as_mut_slice().as_mut_ptr() as u64;
        raw.fb_id_ptr = framebuffers.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETRESOURCES, &raw);

        let res = DrmModeCardRes{
            raw: raw,
            connectors: connectors,
            encoders: encoders,
            crtcs: crtcs,
            framebuffers: framebuffers
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct DrmModeGetConnector {
    pub raw: drm_mode_get_connector,
    pub encoders: Vec<u32>,
    pub modes: Vec<drm_mode_modeinfo>,
    pub properties: Vec<u32>,
    pub prop_values: Vec<u64>
}

impl DrmModeGetConnector {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetConnector> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_get_connector = unsafe { mem::zeroed() };
        raw.connector_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

        // Create buffers for each array
        let mut encoders: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_encoders as usize];
        let mut modes: Vec<drm_mode_modeinfo> =
            vec![unsafe { mem::zeroed() }; raw.count_modes as usize];
        let mut properties: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];
        let mut prop_values: Vec<u64> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];

        // Pass a handle to the buffers to the raw struct
        raw.encoders_ptr = encoders.as_mut_slice().as_mut_ptr() as u64;
        raw.modes_ptr = modes.as_mut_slice().as_mut_ptr() as u64;
        raw.props_ptr = properties.as_mut_slice().as_mut_ptr() as u64;
        raw.prop_values_ptr = prop_values.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCONNECTOR, &raw);

        let conn = DrmModeGetConnector{
            raw: raw,
            encoders: encoders,
            modes: modes,
            properties: properties,
            prop_values: prop_values
        };

        Ok(conn)
    }
}

#[derive(Debug)]
pub struct DrmModeGetEncoder {
    pub raw: drm_mode_get_encoder
}

impl DrmModeGetEncoder {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetEncoder> {
        let mut raw: drm_mode_get_encoder = unsafe { mem::zeroed() };
        raw.encoder_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETENCODER, &raw);
        let enc = DrmModeGetEncoder { raw: raw };
        Ok(enc)
    }
}

#[derive(Debug)]
pub struct DrmModeGetCrtc {
    pub raw: drm_mode_crtc
}

impl DrmModeGetCrtc {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetCrtc> {
        let mut raw: drm_mode_crtc = unsafe { mem::zeroed() };
        raw.crtc_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETCRTC, &raw);
        let crtc = DrmModeGetCrtc { raw: raw };
        Ok(crtc)
    }
}

#[derive(Debug)]
pub struct DrmModeObjectGetProperties {
    pub raw: drm_mode_obj_get_properties,
    pub prop_ids: Vec<u32>,
    pub prop_values: Vec<u64>
}

impl DrmModeObjectGetProperties {
    pub fn new(fd: RawFd, id: u32, obj_type: u32) -> Result<DrmModeObjectGetProperties> {
        let mut raw: drm_mode_obj_get_properties = unsafe { mem::zeroed() };
        raw.obj_id = id;
        raw.obj_type = obj_type;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);

        let mut prop_ids: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];
        let mut prop_values: Vec<u64> =
            vec![unsafe { mem::zeroed() }; raw.count_props as usize];

        raw.props_ptr = prop_ids.as_mut_slice().as_mut_ptr() as u64;
        raw.prop_values_ptr = prop_values.as_mut_slice().as_mut_ptr() as u64;

        ioctl!(fd, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);

        let props = DrmModeObjectGetProperties {
            raw: raw,
            prop_ids: prop_ids,
            prop_values: prop_values
        };
        Ok(props)
    }
}

#[derive(Debug)]
pub struct DrmModePropertyEnums {
    values: Vec<u64>,
    enums: Vec<drm_mode_property_enum>
}

#[derive(Debug)]
pub struct DrmModePropertyBlobs {
    values: Vec<u32>,
    blobs: Vec<u32>
}

#[derive(Debug)]
pub enum DrmModePropertyValues {
    Enums(DrmModePropertyEnums),
    Blobs(DrmModePropertyBlobs)
}

#[derive(Debug)]
pub struct DrmModeGetProperty {
    pub raw: drm_mode_get_property,
    pub values: DrmModePropertyValues
}

impl DrmModeGetProperty {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetProperty> {
        let mut raw: drm_mode_get_property = unsafe { mem::zeroed() };
        raw.prop_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

        // Check if the properties are in enums or blobs
        let values =
            if (raw.flags & (DRM_MODE_PROP_ENUM | DRM_MODE_PROP_BITMASK)) != 0 {
                let mut values: Vec<u64> =
                    vec![unsafe { mem::zeroed() }; raw.count_values as usize];
                let mut enums: Vec<drm_mode_property_enum> =
                    vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];

                raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;
                raw.enum_blob_ptr = enums.as_mut_slice().as_mut_ptr() as u64;

                DrmModePropertyValues::Enums(DrmModePropertyEnums {
                    values: values,
                    enums: enums
                })
            } else if (raw.flags & DRM_MODE_PROP_BLOB) != 0 {
                let mut values: Vec<u32> =
                    vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];
                let mut blobs: Vec<u32> =
                    vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];

                raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;
                raw.enum_blob_ptr = blobs.as_mut_slice().as_mut_ptr() as u64;

                DrmModePropertyValues::Blobs(DrmModePropertyBlobs {
                    values: values,
                    blobs: blobs
                })
            } else {
                return Err(ErrorKind::UnknownPropertyType.into());
            };

        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

        let prop = DrmModeGetProperty {
            raw: raw,
            values: values
        };
        Ok(prop)
    }
}

#[derive(Debug)]
pub struct DrmModePlaneRes {
    pub raw: drm_mode_get_plane_res,
    pub planes: Vec<u32>,
}

impl DrmModePlaneRes {
    pub fn new(fd: RawFd) -> Result<DrmModePlaneRes> {
        // Call ioctl to get the initial structure and buffer sizes
        let mut raw: drm_mode_get_plane_res = unsafe { mem::zeroed() };
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &raw);

        // Create buffers for each array
        let mut planes: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_planes as usize];

        // Pass a handle to the buffers to the raw struct
        raw.plane_id_ptr = planes.as_mut_slice().as_mut_ptr() as u64;

        // Call the ioctl again to fill up the structs
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANERESOURCES, &raw);

        let res = DrmModePlaneRes{
            raw: raw,
            planes: planes,
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct DrmModeGetPlane {
    pub raw: drm_mode_get_plane
}

impl DrmModeGetPlane {
    pub fn new(fd: RawFd, id: u32) -> Result<DrmModeGetPlane> {
        let mut raw: drm_mode_get_plane = unsafe { mem::zeroed() };
        raw.plane_id = id;
        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPLANE, &raw);
        let plane = DrmModeGetPlane { raw: raw };
        Ok(plane)
    }
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

