use std::mem;
use std::ffi::CStr;
use std::{u64, i64};
use std::os::unix::io::RawFd;
use std::io::Error as IoError;
use libc::ioctl;
use ::result::{Result, Error, ErrorKind};

use super::*;

macro_rules! ioctl {
    ( $fd:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($fd, $code as u64, $obj) != 0 {
            return Err(IoError::last_os_error().into());
        }
    })
}

#[derive(Debug)]
pub struct ResourceProperties {
    pub raw: drm_mode_obj_get_properties,
    pub prop_ids: Vec<u32>,
    pub prop_values: Vec<u64>
}

pub fn get_resource_properties(fd: RawFd, id: u32, obj_type: ObjectType) -> Result<ResourceProperties> {
    let obj_type = match obj_type {
        ObjectType::Connector => DRM_MODE_OBJECT_CONNECTOR,
        ObjectType::Encoder => DRM_MODE_OBJECT_ENCODER,
        ObjectType::Mode => DRM_MODE_OBJECT_MODE,
        ObjectType::Property => DRM_MODE_OBJECT_PROPERTY,
        ObjectType::Framebuffer => DRM_MODE_OBJECT_FB,
        ObjectType::Blob => DRM_MODE_OBJECT_BLOB,
        ObjectType::Plane => DRM_MODE_OBJECT_PLANE,
        ObjectType::Controller => DRM_MODE_OBJECT_CRTC,
        ObjectType::Unknown => DRM_MODE_OBJECT_ANY
    };

    let mut raw: drm_mode_obj_get_properties = unsafe { mem::zeroed() };
    raw.obj_id = id;
    raw.obj_type = obj_type;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);

    let mut prop_ids: Vec<u32> =
        vec![unsafe { mem::zeroed() }; raw.count_props as usize];
    let mut prop_val: Vec<u64> =
        vec![unsafe { mem::zeroed() }; raw.count_props as usize];

    raw.props_ptr = prop_ids.as_mut_ptr() as u64;
    raw.prop_values_ptr = prop_val.as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_OBJ_GETPROPERTIES, &raw);

    let props = ResourceProperties {
        raw: raw,
        prop_ids: prop_ids,
        prop_values: prop_val
    };
    Ok(props)
}

pub type PropertyEnumVal = (i64, String);

#[derive(Debug)]
pub struct PropertyEnum {
    pub values: Vec<i64>,
    pub enums: Vec<PropertyEnumVal>
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct PropertyBlob {
    pub id: u64,
    pub data: Vec<u8>
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct PropertyURange {
    pub values: (u64, u64)
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct PropertyIRange {
    pub values: (i64, i64)
}

// TODO: Unsure how to handle this yet
#[derive(Debug)]
pub struct PropertyObject {
    pub value: ObjectType
}

#[derive(Debug)]
pub enum ObjectType {
    Connector,
    Encoder,
    Mode,
    Property,
    Framebuffer,
    Blob,
    Plane,
    Controller,
    Unknown
}

#[derive(Debug)]
pub enum PropertyValue {
    Enum(PropertyEnum),
    Blob(PropertyBlob),
    URange(PropertyURange),
    IRange(PropertyIRange),
    Object(PropertyObject)
}

#[derive(Debug)]
pub struct Property {
    pub raw: drm_mode_get_property,
    pub name: String,
    pub mutable: bool,
    pub pending: bool,
    pub value: PropertyValue
}

pub fn get_property(fd: RawFd, id: u32, val: u64) -> Result<Property> {
    let mut raw: drm_mode_get_property = unsafe { mem::zeroed() };
    raw.prop_id = id;
    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, &raw);

    // Check if the properties are in enums or blobs
    let value =
        if (raw.flags & (DRM_MODE_PROP_ENUM | DRM_MODE_PROP_BITMASK)) != 0 {
            new_enum(fd, &mut raw)
        } else if (raw.flags & DRM_MODE_PROP_BLOB) != 0 {
            match new_blob(fd, val) {
                Ok(b) => Ok(b),
                Err(Error(e, n)) => match e {
                    ErrorKind::PermissionDenied => Err(Error(e, n)),
                    _ => {
                        let blob = PropertyBlob {
                            id: val,
                            data: Vec::new()
                        };
                        Ok(PropertyValue::Blob(blob))
                    }
                }
            }
        } else if (raw.flags & DRM_MODE_PROP_RANGE) != 0 {
            new_urange(fd, &mut raw)
        } else if (raw.flags & MACRO_DRM_MODE_PROP_SIGNED_RANGE) != 0 {
            new_irange(fd, &mut raw)
        } else if (raw.flags & MACRO_DRM_MODE_PROP_OBJECT) != 0 {
            new_object(fd, &mut raw)
        } else {
            return Err(ErrorKind::UnknownPropertyType(raw.flags).into());
        };

    let value = match value {
        Ok(v) => v,
        Err(e) => bail!(e)
    };

    let cstr = unsafe { CStr::from_ptr(&raw.name as *const _) };
    let name = match cstr.to_str() {
        Ok(n) => n,
        Err(_) => "Unknown"
    };

    let prop = Property {
        raw: raw,
        name: name.to_string(),
        mutable: (raw.flags & DRM_MODE_PROP_IMMUTABLE) == 0,
        pending: (raw.flags & DRM_MODE_PROP_PENDING) == 1,
        value: value
    };


    Ok(prop)
}

fn new_enum(fd: RawFd, raw: &mut drm_mode_get_property) -> Result<PropertyValue> {
    // Create buffers to hold the data
    let mut values: Vec<i64> =
        vec![unsafe { mem::zeroed() }; raw.count_values as usize];
    let mut enums: Vec<drm_mode_property_enum> =
        vec![unsafe { mem::zeroed() }; raw.count_enum_blobs as usize];

    // Assign the raw pointers of the buffers to the raw struct
    raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;
    raw.enum_blob_ptr = enums.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, raw);

    // Collect the enums into a list of EnumPropertyValues
    let enums = enums.iter().map(| &en | {
        let cstr = unsafe { CStr::from_ptr(&en.name as *const _) };
        let name = match cstr.to_str() {
            Ok(n) => n,
            Err(_) => "Unknown"
        };
        (en.value as i64, name.to_string())
    }).collect();

    let prop = PropertyEnum {
        values: values,
        enums: enums
    };

    Ok(PropertyValue::Enum(prop))
}

// TODO: Currently does not work. Need to figure out where blob ids are stored.
fn new_blob(fd: RawFd, id: u64) -> Result<PropertyValue> {
    let mut raw_blob: drm_mode_get_blob = unsafe { mem::zeroed() };
    raw_blob.blob_id = id as u32;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPBLOB, &raw_blob);

    let mut data: Vec<u8> =
        vec![unsafe { mem::zeroed() }; raw_blob.length as usize];

    raw_blob.data = data.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPBLOB, &raw_blob);

    let prop = PropertyBlob {
        id: id,
        data: data
    };

    Ok(PropertyValue::Blob(prop))
}

fn new_urange(fd: RawFd, raw: &mut drm_mode_get_property) -> Result<PropertyValue> {
    let mut values: Vec<u64> =
        vec![unsafe { mem::zeroed() }; raw.count_values as usize];

    raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, raw);

    let &min = values.get(0).unwrap_or(&0);
    let &max = values.get(1).unwrap_or(&u64::MAX);

    let prop = PropertyURange {
        values: (min, max)
    };

    Ok(PropertyValue::URange(prop))
}

fn new_irange(fd: RawFd, raw: &mut drm_mode_get_property) -> Result<PropertyValue> {
    let mut values: Vec<i64> =
        vec![unsafe { mem::zeroed() }; raw.count_values as usize];

    raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, raw);

    let &min = values.get(0).unwrap_or(&i64::MIN);
    let &max = values.get(1).unwrap_or(&i64::MAX);

    let prop = PropertyIRange {
        values: (min, max)
    };

    Ok(PropertyValue::IRange(prop))
}

fn new_object(fd: RawFd, raw: &mut drm_mode_get_property) -> Result<PropertyValue> {
    let mut values: Vec<u32> =
        vec![unsafe { mem::zeroed() }; raw.count_values as usize];

    raw.values_ptr = values.as_mut_slice().as_mut_ptr() as u64;

    ioctl!(fd, MACRO_DRM_IOCTL_MODE_GETPROPERTY, raw);

    let &val = values.get(0).unwrap_or(&0);

    let obj_type = match val {
        DRM_MODE_OBJECT_CONNECTOR => ObjectType::Connector,
        DRM_MODE_OBJECT_ENCODER => ObjectType::Encoder,
        DRM_MODE_OBJECT_MODE => ObjectType::Mode,
        DRM_MODE_OBJECT_PROPERTY => ObjectType::Property,
        DRM_MODE_OBJECT_FB => ObjectType::Framebuffer,
        DRM_MODE_OBJECT_BLOB => ObjectType::Blob,
        DRM_MODE_OBJECT_PLANE => ObjectType::Plane,
        DRM_MODE_OBJECT_CRTC => ObjectType::Controller,
        _ => ObjectType::Unknown
    };

    let prop = PropertyObject {
        value: obj_type
    };

    Ok(PropertyValue::Object(prop))
}
