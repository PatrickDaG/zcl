#![feature(f16)]

use thiserror::Error;
pub mod types;

fn main() {}

enum AttributeRange<T: ZCLType> {
    Attr(u16),
    Size(usize),
    Value(T),
}

pub struct Attribute<'a, T: ZCLType> {
    code: u16,
    name: &'a str,
    side: AttributeSide,
    writable: bool,
    readable: bool,
    reportable: bool,
    scene: bool,
    optional: bool,
    default: T,
    min: AttributeRange<T>,
    max: AttributeRange<T>,
}

pub enum AnyAttribute<'a> {
    NoData(Attribute<'a, types::NoData>),
    Data8(Attribute<'a, types::Data8>),
    Data16(Attribute<'a, types::Data16>),
    Data24(Attribute<'a, types::Data24>),
    Data32(Attribute<'a, types::Data32>),
    Data40(Attribute<'a, types::Data40>),
    Data48(Attribute<'a, types::Data48>),
    Data56(Attribute<'a, types::Data56>),
    Data64(Attribute<'a, types::Data64>),
    Bool(Attribute<'a, types::Bool>),
    Bitmap8(Attribute<'a, types::Bitmap8>),
    Bitmap16(Attribute<'a, types::Bitmap16>),
    Bitmap24(Attribute<'a, types::Bitmap24>),
    Bitmap32(Attribute<'a, types::Bitmap32>),
    Bitmap40(Attribute<'a, types::Bitmap40>),
    Bitmap48(Attribute<'a, types::Bitmap48>),
    Bitmap56(Attribute<'a, types::Bitmap56>),
    Bitmap64(Attribute<'a, types::Bitmap64>),
    U8(Attribute<'a, types::U8>),
    U16(Attribute<'a, types::U16>),
    U24(Attribute<'a, types::U24>),
    U32(Attribute<'a, types::U32>),
    U40(Attribute<'a, types::U40>),
    U48(Attribute<'a, types::U48>),
    U56(Attribute<'a, types::U56>),
    U64(Attribute<'a, types::U64>),
    I8(Attribute<'a, types::I8>),
    I16(Attribute<'a, types::I16>),
    I24(Attribute<'a, types::I24>),
    I32(Attribute<'a, types::I32>),
    I40(Attribute<'a, types::I40>),
    I48(Attribute<'a, types::I48>),
    I56(Attribute<'a, types::I56>),
    I64(Attribute<'a, types::I64>),
    Enum8(Attribute<'a, types::Enum8>),
    Enum16(Attribute<'a, types::Enum16>),
    F16(Attribute<'a, types::F16>),
    F32(Attribute<'a, types::F32>),
    F64(Attribute<'a, types::F64>),
    OctetString(Attribute<'a, types::OctetString>),
    CharacterString(Attribute<'a, types::CharacterString>),
    LongOctetString(Attribute<'a, types::LongOctetString>),
    LongCharacterString(Attribute<'a, types::LongCharacterString>),
    Array(Attribute<'a, types::Array>),
    Structure(Attribute<'a, types::Structure>),
    TimeOfDay(Attribute<'a, types::TimeOfDay>),
    Date(Attribute<'a, types::Date>),
    UtcTime(Attribute<'a, types::UtcTime>),
    ClusterId(Attribute<'a, types::ClusterId>),
    AttributeId(Attribute<'a, types::AttributeId>),
    BacnetOid(Attribute<'a, types::BacnetOid>),
    IeeeAddress(Attribute<'a, types::IeeeAddress>),
    SecurityKey(Attribute<'a, types::SecurityKey>),
    Unknown(Attribute<'a, types::Unknown>),
}

pub enum AttributeSide {
    Server,
    Client,
    Either,
}

#[derive(Error, Debug)]
pub enum ZCLError {
    #[error("failed to serialize value")]
    Serialization,
    #[error("value is out of range")]
    ValueOutOfRange,
}

trait ZCLCompatibleType {
    fn len(&self) -> usize;
    //fn to_bytes(self, data: &mut [u8]) -> Result<(), ZCLError>;
    //fn from_bytes(data: &[u8]) -> Result<Self, ZCLError>;
}

trait ZCLType: ZCLCompatibleType {
    type T;
    const NON_VALUE: Option<Self::T>;
    const ID: u8;
}

mod globals {
    use crate::types;
    const CLUSTER_REVISION: super::Attribute<types::U16> = super::Attribute {
        code: todo!(),
        side: todo!(),
        writable: todo!(),
        readable: todo!(),
        reportable: todo!(),
        scene: todo!(),
        optional: todo!(),
        default: todo!(),
        min: todo!(),
        max: todo!(),
        name: todo!(),
    };
    #[repr(u8)]
    pub enum ReportingStatus {
        Pending = 0,
        Complete = 1,
        None = 0xff,
    }

    impl types::ZCLEnum for ReportingStatus {
        const NON_VALUE: Self = Self::None;
    }
    const ATTRIBUTE_REPORTING_STATUS: super::Attribute<types::Enum8<ReportingStatus>> =
        super::Attribute {
            code: todo!(),
            side: todo!(),
            writable: todo!(),
            readable: todo!(),
            reportable: todo!(),
            scene: todo!(),
            optional: todo!(),
            default: todo!(),
            min: todo!(),
            max: todo!(),
            name: todo!(),
        };
}

#[repr(u8)]
pub enum Command {
    PlaceholderTodo,
}

#[repr(u8)]
pub enum Status {
    PlaceholderTodo,
}

pub struct Cluster<'a> {
    code: u16,
    name: &'a str,
    #[cfg(feature = "std")]
    attributes: Vec<&'a AnyAttribute<'a>>,
    #[cfg(not(feature = "std"))]
    attributes: &'a [AnyAttribute<'a>],
}

pub mod clusters {
    #[cfg(feature = "std")]
    macro_rules! attrs {
        ($attrs: tt) => {
        vec! $attrs
    }}
    #[cfg(not(feature = "std"))]
    macro_rules! attrs {
        ($attrs: tt) => {
            &$attrs
        };
    }
    pub mod general {
        const BASIC: crate::Cluster = crate::Cluster {
            code: 0x0000,
            name: "Basic",
            attributes: attrs!([]),
        };

        const POWER_CONFIGURATION: crate::Cluster = crate::Cluster {
            code: 0x0001,
            name: "Power Configuration",
            attributes: attrs!([]),
        };

        const DEVICE_TEMPERATURE: crate::Cluster = crate::Cluster {
            code: 0x0002,
            name: "Device Temperature",
            attributes: attrs!([]),
        };

        const IDENTIFY: crate::Cluster = crate::Cluster {
            code: 0x0003,
            name: "Identify",
            attributes: attrs!([]),
        };
    }
}
