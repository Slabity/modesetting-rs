use ::ffi;

use ::Resource;
use ::ResourceId;
use ::PropertyId;
use ::BlobId;

#[derive(Debug, Clone)]
pub struct Property<V, P> {
    name: String,
    parent: ResourceId,
    id: PropertyId,
    mutable: bool,
    value: V,
    possible: P
}

trait Valueu64 {
    fn value_u64(&self) -> u64;
}

pub trait Update<T> {
    fn update(&self, value: T) -> PropertyUpdate;
}

pub type Enum = Property<i64, Vec<(i64, String)>>;
pub type Blob = Property<(BlobId, Vec<u8>), ObjectType>;
pub type URange = Property<u64, (u64, u64)>;
pub type IRange = Property<i64, (i64, i64)>;
pub type Object = Property<ResourceId, ObjectType>;

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Connector,
    Encoder,
    Controller,
    Framebuffer,
    Plane,
    Property,
    Mode,
    Blob,
    Unknown
}

#[derive(Debug, Clone)]
pub enum Value {
    Enum(Enum),
    Blob(Blob),
    URange(URange),
    IRange(IRange),
    Object(Object),
    Unknown
}

impl Value {
    pub fn name(&self) -> &str {
        match self {
            &Value::Enum(ref p) => p.name(),
            &Value::Blob(ref p) => p.name(),
            &Value::URange(ref p) => p.name(),
            &Value::IRange(ref p) => p.name(),
            &Value::Object(ref p) => p.name(),
            &Value::Unknown => "Unknown"
        }
    }

    pub fn id(&self) -> ResourceId {
        match self {
            &Value::Enum(ref p) => p.id(),
            &Value::Blob(ref p) => p.id(),
            &Value::URange(ref p) => p.id(),
            &Value::IRange(ref p) => p.id(),
            &Value::Object(ref p) => p.id(),
            &Value::Unknown => 0
        }
    }
}

impl<V, P> Property<V, P> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> PropertyId {
        self.id
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn possible(&self) -> &P {
        &self.possible
    }
}

impl From<(ResourceId, ffi::PropertyValue)> for Value {
    fn from(raw: (ResourceId, ffi::PropertyValue)) -> Value {
        match raw.1 {
            ffi::PropertyValue::Enum(p) => {
                let mut prop = Enum::from(p);
                prop.parent = raw.0;
                Value::Enum(prop)
            },
            ffi::PropertyValue::Blob(p) => {
                let mut prop = Blob::from(p);
                prop.parent = raw.0;
                Value::Blob(prop)
            },
            ffi::PropertyValue::URange(p) => {
                let mut prop = URange::from(p);
                prop.parent = raw.0;
                Value::URange(prop)
            },
            ffi::PropertyValue::IRange(p) => {
                let mut prop = IRange::from(p);
                prop.parent = raw.0;
                Value::IRange(prop)
            },
            ffi::PropertyValue::Object(p) => {
                let mut prop = Object::from(p);
                prop.parent = raw.0;
                Value::Object(prop)
            }
        }
    }
}

impl From<ffi::PropertyEnum> for Enum {
    fn from(raw: ffi::PropertyEnum) -> Enum {
        Enum {
            name: raw.name,
            parent: 0,
            id: raw.raw.prop_id,
            mutable: raw.mutable,
            value: raw.value as i64,
            possible: raw.possible
        }
    }
}

impl Valueu64 for Enum {
    fn value_u64(&self) -> u64 {
        self.value as u64
    }
}

impl Update<i64> for Enum {
    fn update(&self, value: i64) -> PropertyUpdate {
        PropertyUpdate {
            resource: self.parent,
            property: self.id,
            value: value
        }
    }
}

impl From<ffi::PropertyBlob> for Blob {
    fn from(raw: ffi::PropertyBlob) -> Blob {
        Blob {
            name: raw.name,
            parent: 0,
            id: raw.raw.prop_id,
            mutable: raw.mutable,
            value: (raw.value.0 as u32, raw.value.1),
            possible: ObjectType::Blob
        }
    }
}

impl From<ffi::PropertyURange> for URange {
    fn from(raw: ffi::PropertyURange) -> URange {
        URange {
            name: raw.name,
            parent: 0,
            id: raw.raw.prop_id,
            mutable: raw.mutable,
            value: raw.value as u64,
            possible: raw.possible
        }
    }
}

impl Update<u64> for URange {
    fn update(&self, value: u64) -> PropertyUpdate {
        PropertyUpdate {
            resource: self.parent,
            property: self.id,
            value: value as i64
        }
    }
}

impl Valueu64 for URange {
    fn value_u64(&self) -> u64 {
        self.value as u64
    }
}

impl From<ffi::PropertyIRange> for IRange {
    fn from(raw: ffi::PropertyIRange) -> IRange {
        IRange {
            name: raw.name,
            parent: 0,
            id: raw.raw.prop_id,
            mutable: raw.mutable,
            value: raw.value as i64,
            possible: raw.possible
        }
    }
}

impl Valueu64 for IRange {
    fn value_u64(&self) -> u64 {
        self.value as u64
    }
}

impl Update<i64> for IRange {
    fn update(&self, value: i64) -> PropertyUpdate {
        PropertyUpdate {
            resource: self.parent,
            property: self.id,
            value: value
        }
    }
}

impl From<ffi::PropertyObject> for Object {
    fn from(raw: ffi::PropertyObject) -> Object {
        let obj_type = match raw.possible {
            ffi::ObjectType::Connector => ObjectType::Connector,
            ffi::ObjectType::Encoder => ObjectType::Encoder,
            ffi::ObjectType::Mode => ObjectType::Mode,
            ffi::ObjectType::Property => ObjectType::Property,
            ffi::ObjectType::Framebuffer => ObjectType::Framebuffer,
            ffi::ObjectType::Blob => ObjectType::Blob,
            ffi::ObjectType::Plane => ObjectType::Plane,
            ffi::ObjectType::Controller => ObjectType::Controller,
            ffi::ObjectType::Unknown => ObjectType::Unknown,
        };
        Object {
            name: raw.name,
            parent: 0,
            id: raw.raw.prop_id,
            mutable: raw.mutable,
            value: raw.value as ResourceId,
            possible: obj_type
        }
    }
}

impl Valueu64 for Object {
    fn value_u64(&self) -> u64 {
        self.value as u64
    }
}

impl<'a, T> Update<&'a Resource<T>> for Object {
    fn update(&self, value: &'a Resource<T>) -> PropertyUpdate {
        PropertyUpdate {
            resource: self.parent,
            property: self.id,
            value: value.id() as i64
        }
    }
}

#[derive(Debug)]
pub struct PropertyUpdate {
    resource: ResourceId,
    property: PropertyId,
    value: i64
}

