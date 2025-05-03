#![feature(f16)]

use thiserror::Error;
pub mod types;

fn main() {}

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
    min: T,
    max: T,
}

pub enum AnyAttribute<'a> {
    I16(Attribute<'a, types::I16>),
    U16(Attribute<'a, types::U16>),
}

pub enum AttributeSide {
    Server,
    Client,
    Either,
}

#[derive(Error, Debug)]
pub enum ZCLError {
    Serialization,
    ValueOutOfRange,
}

trait ZCLType {
    type T;
    const NON_VALUE: Option<Self::T>;
    const ID: u8;
    fn len(&self) -> usize;
    //fn to_bytes(self, data: &mut [u8]) -> Result<(), ZCLError>;
    //fn from_bytes(data: &[u8]) -> Result<Self, ZCLError>;
}

mod types {}

mod globals {
    use crate::types;
    const ClusterRevision: super::Attribute<types::U16> = super::Attribute {
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
    const AttributeReportingStatus: super::Attribute<types::Enum8<ReportingStatus>> =
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
    // #[cfg(feature = "std")]
    attributes: Vec<&'a AnyAttribute<'a>>,
    // #[cfg(not(feature = "std"))]
    // attributes: &'a [AnyAttribute<'a>],
}

pub mod clusters {
    pub mod general {
        const BASIC: crate::Cluster = crate::Cluster {
            code: 0x0000,
            name: "Basic",
            attributes: vec![],
        };

        const POWER_CONFIGURATION: crate::Cluster = crate::Cluster {
            code: 0x0001,
            name: "Power Configuration",
            attributes: vec![],
        };

        const DEVICE_TEMPERATURE: crate::Cluster = crate::Cluster {
            code: 0x0002,
            name: "Device Temperature",
            attributes: vec![],
        };

        const IDENTIFY: crate::Cluster = crate::Cluster {
            code: 0x0003,
            name: "Identify",
            attributes: vec![],
        };
    }
}
