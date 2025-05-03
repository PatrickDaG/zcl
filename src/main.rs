#![no_std]
fn main() {
    println!("Hello, world!");
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
    min: T,
    max: T,
}

pub enum AttributeSide {
    Server,
    Client,
    Either,
}

trait ZCLType {
    type T;
    const NON_VALUE: Option<Self::T>;
    const ID: u8;
    const LENGTH: u8;
}

mod types {
    pub struct I16(i16);
    impl super::ZCLType for I16 {
        type T = i16;

        const NON_VALUE: Option<Self::T> = Some(0x800);

        const ID: u8 = 0x29;

        const LENGTH: u8 = 0x2;
    }
    pub struct U16(u16);
    impl super::ZCLType for U16 {
        type T;

        const NON_VALUE: Option<Self::T>;

        const ID: u8;

        const LENGTH: u8;
    }
    pub trait ZCLEnum {
        const NON_VALUE: Self;
    }
    pub struct ENUM8<T: ZCLEnum>(T);
    impl<T: ZCLEnum> super::ZCLType for ENUM8<T> {
        type T = T;

        const NON_VALUE: Option<Self::T> = Some(T::NON_VALUE);

        const ID: u8 = 0x30;

        const LENGTH: u8 = 0x1;
    }
}

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
    const AttributeReportingStatus: super::Attribute<types::ENUM8<ReportingStatus>> =
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
pub enum commands {}

#[repr(u8)]
pub enum status {}

pub struct Cluster {
    code: u16,
}
