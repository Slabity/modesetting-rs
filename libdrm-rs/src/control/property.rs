use drm_sys::*;
use super::super::util::*;
use super::super::result::*;

use std::os::unix::io::{RawFd, AsRawFd};

#[derive(Debug)]
pub struct Property<V, P> {
    pub name: (),
    pub mutable: bool,
    pub pending: bool,
    pub value: V,
    pub possible: P
}

pub type PropertyEnum = Property<i64, Vec<PropertyEnumVal>>;
pub type PropertyBlob = Property<(u64, Vec<u8>), ObjectType>;
pub type PropertyURange = Property<u64, (u64, u64)>;
pub type PropertyIRange = Property<i64, (i64, i64)>;
pub type PropertyObject = Property<i64, ObjectType>;

#[derive(Debug)]
pub enum PropertyValue {
    Enum(PropertyEnum),
    Blob(PropertyBlob),
    URange(PropertyURange),
    IRange(PropertyIRange),
    Object(PropertyObject)
}

