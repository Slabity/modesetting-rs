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

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct DrmModePropertyEnum {
    pub values: Vec<u64>,
    pub enums: Vec<drm_mode_property_enum>
}

#[derive(Debug)]
pub struct DrmModePropertyBlob {
    pub values: Vec<u32>,
    pub blob: Vec<u32>
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct DrmModePropertyRange {
    pub values: Vec<u64>
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct DrmModePropertyObject {
}

#[derive(Debug)]
pub enum DrmModePropertyValues {
    Enum(DrmModePropertyEnum),
    Blob(DrmModePropertyBlob),
    Range(DrmModePropertyRange),
    Object(DrmModePropertyObject)
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
                DrmModePropertyValues::Enum(Self::new_enum(&mut raw))
            } else if (raw.flags & DRM_MODE_PROP_BLOB) != 0 {
                DrmModePropertyValues::Blob(Self::new_blob(&mut raw))
            } else if (raw.flags & DRM_MODE_PROP_RANGE) != 0 {
                DrmModePropertyValues::Range(Self::new_range(&mut raw))
            } else if (raw.flags & DRM_MODE_PROP_RANGE) != 0 {
                DrmModePropertyValues::Object(Self::new_object(&mut raw))
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

    fn new_enum(raw: &mut drm_mode_get_property) -> DrmModePropertyEnum {
        // Create buffers to hold the data
        let mut values: Vec<u64> =
            vec![unsafe { mem::zeroed() }; raw.count_values as usize];
        let mut enums: Vec<drm_mode_property_enum> =
            vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];

        // Assign the raw pointers of the buffers to the raw struct
        raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;
        raw.enum_blob_ptr = enums.as_mut_slice().as_mut_ptr() as u64;

        DrmModePropertyEnum {
            values: values,
            enums: enums
        }
    }

    fn new_blob(raw: &mut drm_mode_get_property) -> DrmModePropertyBlob {
        let mut values: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];
        let mut blob: Vec<u32> =
            vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];

        raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;
        raw.enum_blob_ptr = blob.as_mut_slice().as_mut_ptr() as u64;

        DrmModePropertyBlob {
            values: values,
            blob: blob
        }
    }

    fn new_range(raw: &mut drm_mode_get_property) -> DrmModePropertyRange {
        let mut values: Vec<u64> =
            vec![unsafe { mem::zeroed() }; raw.count_values as usize];

        raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;

        DrmModePropertyRange {
            values: values
        }
    }

    fn new_object(raw: &mut drm_mode_get_property) -> DrmModePropertyObject {
        DrmModePropertyObject {}
    }
}
