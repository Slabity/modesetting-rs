use std::mem;
use std::os::unix::io::RawFd;
use std::io::Error as IoError;
use libc::ioctl;
use ::result::{Result, ErrorKind};

use super::*;

macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
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
    pub values: Vec<u64>,
    pub enums: Vec<drm_mode_property_enum>
}

#[derive(Debug)]
pub struct DrmModePropertyBlobs {
    pub values: Vec<u32>,
    pub blobs: Vec<u32>
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
                return Err(ErrorKind::UnknownPropertyType(raw.flags).into());
            };

        ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

        let prop = DrmModeGetProperty {
            raw: raw,
            values: values
        };
        Ok(prop)
    }
}
