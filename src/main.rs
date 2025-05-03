#![no_std]
#![feature(f16)]

use thiserror::Error;
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

mod types {
    macro_rules! basic_type {
        ($name: ident, $type: ty, $non: expr, $id: literal, $len: literal) => {
            pub struct $name($type);
            impl super::ZCLType for $name {
                type T = $type;

                const NON_VALUE: Option<Self::T> = $non;

                const ID: u8 = $id;

                fn len(&self) -> usize {
                    $len
                }
            }
        };
    }
    basic_type!(Unknown, (), None, 0xff, 0x0);
    basic_type!(NoData, (), None, 0x00, 0x0);

    basic_type!(Data8, [u8; 1], None, 0x08, 0x1);
    basic_type!(Data16, [u8; 2], None, 0x09, 0x2);
    basic_type!(Data24, [u8; 3], None, 0x0a, 0x3);
    basic_type!(Data32, [u8; 4], None, 0x0b, 0x4);
    basic_type!(Data40, [u8; 5], None, 0x0c, 0x5);
    basic_type!(Data48, [u8; 6], None, 0x0d, 0x6);
    basic_type!(Data56, [u8; 7], None, 0x0e, 0x7);
    basic_type!(Data64, [u8; 8], None, 0x0f, 0x8);

    basic_type!(Boolean, Option<bool>, Some(None), 0x10, 0x1);

    basic_type!(Bitmap8, u8, None, 0x18, 0x1);

    basic_type!(U8, u8, Some(0xff), 0x20, 0x1);
    basic_type!(U16, u16, Some(0xffff), 0x21, 0x2);
    basic_type!(U24, u32, Some(0xffffff), 0x22, 0x3);

    basic_type!(I8, i8, Some(0x80), 0x28, 0x1);
    basic_type!(I16, i16, Some(0x8000), 0x29, 0x2);

    pub trait ZCLEnum {
        const NON_VALUE: Self;
    }
    pub struct ENUM8<T: ZCLEnum>(T);
    impl<T: ZCLEnum> super::ZCLType for ENUM8<T> {
        type T = T;

        const NON_VALUE: Option<Self::T> = Some(T::NON_VALUE);

        const ID: u8 = 0x30;

        fn len(&self) -> usize {
            0x1
        }
    }

    basic_type!(F16, f16, Some(f16::NAN), 0x38, 0x2);
    basic_type!(F32, f32, Some(f32::NAN), 0x38, 0x2);

    pub struct CharacterString<'a>(Option<&'a str>);
    impl<'a> super::ZCLType for CharacterString<'a> {
        type T = Option<&'a str>;

        const NON_VALUE: Option<Self::T> = Some(None);

        const ID: u8 = 0x42;

        fn len(&self) -> usize {
            match self.0 {
                Some(x) => x.len() + 1,
                None => 1,
            }
        }
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

pub struct Cluster<'a> {
    code: u16,
    name: &'a str,
    attributes: [&'a Attribute<'a>],
}
