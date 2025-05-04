pub mod attribute;

pub trait ZclCompatibleType {
    fn len(&self) -> usize;
    //fn to_bytes(self, data: &mut [u8]) -> Result<(), ZclError>;
    //fn from_bytes(data: &[u8]) -> Result<Self, ZclError>;
}

pub trait ZclType: ZclCompatibleType {
    type T;
    const NON_VALUE: Option<Self::T>;
    const ID: u8;
}

macro_rules! basic_type {
    ($name: ident, $type: ty, $non: expr, $id: literal, $len: literal) => {
        #[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
        pub struct $name(pub $type);
        impl ZclCompatibleType for $name {
            fn len(&self) -> usize {
                $len
            }
        }

        impl ZclType for $name {
            type T = $type;
            const NON_VALUE: Option<Self::T> = $non;
            const ID: u8 = $id;
        }
    };
}

pub trait ZclEnum {
    const NON_VALUE: Self;
}

basic_type!(NoData, (), None, 0x00, 0x0);

basic_type!(Data8, [u8; 1], None, 0x08, 0x1);
basic_type!(Data16, [u8; 2], None, 0x09, 0x2);
basic_type!(Data24, [u8; 3], None, 0x0a, 0x3);
basic_type!(Data32, [u8; 4], None, 0x0b, 0x4);
basic_type!(Data40, [u8; 5], None, 0x0c, 0x5);
basic_type!(Data48, [u8; 6], None, 0x0d, 0x6);
basic_type!(Data56, [u8; 7], None, 0x0e, 0x7);
basic_type!(Data64, [u8; 8], None, 0x0f, 0x8);

basic_type!(Bool, Option<bool>, Some(None), 0x10, 0x1);

basic_type!(Bitmap8, u8, None, 0x18, 0x1);
basic_type!(Bitmap16, u16, None, 0x19, 0x2);
basic_type!(Bitmap24, u32, None, 0x1a, 0x3);
basic_type!(Bitmap32, u32, None, 0x1b, 0x4);
basic_type!(Bitmap40, u64, None, 0x1c, 0x5);
basic_type!(Bitmap48, u64, None, 0x1d, 0x6);
basic_type!(Bitmap56, u64, None, 0x1e, 0x7);
basic_type!(Bitmap64, u64, None, 0x1f, 0x8);

basic_type!(U8, u8, Some(0xff), 0x20, 0x1);
basic_type!(U16, u16, Some(0xffff), 0x21, 0x2);
basic_type!(U24, u32, Some(0xffffff), 0x22, 0x3);
basic_type!(U32, u32, Some(0xffffffff), 0x23, 0x4);
basic_type!(U40, u64, Some(0xffffffffff), 0x24, 0x5);
basic_type!(U48, u64, Some(0xffffffffffff), 0x25, 0x6);
basic_type!(U56, u64, Some(0xffffffffffffff), 0x26, 0x7);
basic_type!(U64, u64, Some(0xffffffffffffffff), 0x27, 0x8);

basic_type!(I8, i8, Some(0x80u8 as i8), 0x28, 0x1);
basic_type!(I16, i16, Some(0x8000u16 as i16), 0x29, 0x2);
basic_type!(I24, i32, Some(0xff800000u32 as i32), 0x2a, 0x3);
basic_type!(I32, i32, Some(0x80000000u32 as i32), 0x2b, 0x4);
basic_type!(I40, i64, Some(0xffffff8000000000u64 as i64), 0x2c, 0x5);
basic_type!(I48, i64, Some(0xffff800000000000u64 as i64), 0x2d, 0x6);
basic_type!(I56, i64, Some(0xff80000000000000u64 as i64), 0x2e, 0x7);
basic_type!(I64, i64, Some(0x8000000000000000u64 as i64), 0x2f, 0x8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Enum8<T: ZclEnum>(pub T);
impl<T: ZclEnum> ZclType for Enum8<T> {
    type T = T;
    const NON_VALUE: Option<Self::T> = Some(T::NON_VALUE);
    const ID: u8 = 0x30;
}
impl<T: ZclEnum> ZclCompatibleType for Enum8<T> {
    fn len(&self) -> usize {
        0x1
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Enum16<T: ZclEnum>(pub T);
impl<T: ZclEnum> ZclType for Enum16<T> {
    type T = T;
    const NON_VALUE: Option<Self::T> = Some(T::NON_VALUE);
    const ID: u8 = 0x31;
}
impl<T: ZclEnum> ZclCompatibleType for Enum16<T> {
    fn len(&self) -> usize {
        0x2
    }
}

// basic_type!(F16, f16, Some(f16::NAN), 0x38, 0x2);
basic_type!(F32, f32, Some(f32::NAN), 0x39, 0x4);
basic_type!(F64, f64, Some(f64::NAN), 0x3a, 0x8);

pub struct OctetString<'a>(pub Option<&'a [u8]>);
impl<'a> ZclType for OctetString<'a> {
    type T = Option<&'a [u8]>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x41;
}
impl ZclCompatibleType for OctetString<'_> {
    fn len(&self) -> usize {
        match self.0 {
            Some(x) => x.len() + 1,
            None => 1,
        }
    }
}

pub struct CharacterString<'a>(pub Option<&'a str>);
impl<'a> ZclType for CharacterString<'a> {
    type T = Option<&'a str>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x42;
}
impl ZclCompatibleType for CharacterString<'_> {
    fn len(&self) -> usize {
        match self.0 {
            Some(x) => x.len() + 1,
            None => 1,
        }
    }
}

pub struct LongOctetString<'a>(pub Option<&'a [u8]>);
impl<'a> ZclType for LongOctetString<'a> {
    type T = Option<&'a [u8]>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x43;
}
impl ZclCompatibleType for LongOctetString<'_> {
    fn len(&self) -> usize {
        match self.0 {
            Some(x) => x.len() + 2,
            None => 2,
        }
    }
}

pub struct LongCharacterString<'a>(pub Option<&'a str>);
impl<'a> ZclType for LongCharacterString<'a> {
    type T = Option<&'a str>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x44;
}
impl ZclCompatibleType for LongCharacterString<'_> {
    fn len(&self) -> usize {
        match self.0 {
            Some(x) => x.len() + 2,
            None => 2,
        }
    }
}

pub struct Array<'a, T>(pub Option<&'a [T]>);
impl<'a, T: ZclCompatibleType> ZclType for Array<'a, T> {
    type T = Option<&'a [T]>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x48;
}
impl<T: ZclCompatibleType> ZclCompatibleType for Array<'_, T> {
    fn len(&self) -> usize {
        match self.0 {
            Some(xs) => 2 + xs.iter().map(|x| x.len()).sum::<usize>(),
            None => 2,
        }
    }
}

pub trait StructureCompatibleType: ZclCompatibleType {
    // u16 is required for Structures by the Zcl spec
    fn num_members(&self) -> u16;
}

pub struct Structure<T>(Option<T>);
impl<T: StructureCompatibleType> ZclType for Structure<T> {
    type T = Option<T>;
    const NON_VALUE: Option<Self::T> = Some(None);
    const ID: u8 = 0x4c;
}
impl<T: StructureCompatibleType> ZclCompatibleType for Structure<T> {
    fn len(&self) -> usize {
        match self.0 {
            Some(ref x) => x.num_members() as usize + 2,
            None => 2,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TimeOfDay {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub hundredths: u8,
}
// TODO impl ord for TimeOfDay, ignore 0xff values
impl TimeOfDay {
    const INVALID: Self = Self {
        hours: 0xff,
        minutes: 0xff,
        seconds: 0xff,
        hundredths: 0xff,
    };
}
impl ZclType for TimeOfDay {
    type T = TimeOfDay;
    const NON_VALUE: Option<Self::T> = Some(TimeOfDay::INVALID);
    const ID: u8 = 0xe0;
}
impl ZclCompatibleType for TimeOfDay {
    fn len(&self) -> usize {
        0x4
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Date {
    /// year (starting at 1900)
    pub year: u8,
    /// month 1-12
    pub month: u8,
    /// day 1-31
    pub day_of_month: u8,
    /// weekday 1-7
    pub day_of_week: u8,
}
// TODO impl ord for Date, ignore 0xff values
impl Date {
    const INVALID: Self = Self {
        year: 0xff,
        month: 0xff,
        day_of_month: 0xff,
        day_of_week: 0xff,
    };
}
impl ZclType for Date {
    type T = Date;
    const NON_VALUE: Option<Self::T> = Some(Date::INVALID);
    const ID: u8 = 0xe1;
}
impl ZclCompatibleType for Date {
    fn len(&self) -> usize {
        0x4
    }
}

basic_type!(UtcTime, u32, Some(u32::MAX), 0xe2, 0x4);
basic_type!(ClusterId, u16, Some(u16::MAX), 0xe8, 0x2);
basic_type!(AttributeId, u16, Some(u16::MAX), 0xe9, 0x2);
basic_type!(BacnetOid, u32, Some(u32::MAX), 0xea, 0x4);
basic_type!(IeeeAddress, u64, Some(u64::MAX), 0xf0, 0x8);
basic_type!(SecurityKey, [u8; 16], None, 0xf1, 0x10);
basic_type!(Unknown, (), None, 0xff, 0x0);
