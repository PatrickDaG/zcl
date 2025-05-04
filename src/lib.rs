use thiserror::Error;
pub mod attribute;
pub mod types;

fn main() {}

#[derive(Error, Debug)]
pub enum ZclError {
    #[error("failed to serialize value")]
    Serialization,
    #[error("value is out of range")]
    ValueOutOfRange,
}

#[crabtime::function]
fn define_attribute_raw(
    pattern!(
        $code:literal,
        $name:ident,
        $typ:ty,
        $min:expr,
        $max:expr,
        $flags:tt,
        $default:expr,
        $optional:ident
    ): _,
) {
    #![dependency(convert_case = "0.8")]

    use convert_case::{Case, Casing};
    let name_constant_case = stringify!($name).to_case(Case::Constant);
    let flags = stringify!($flags);
    let readable = flags.contains('R');
    let writable = flags.contains('W');
    let reportable = flags.contains('P');
    let scene = flags.contains('S');
    let mandatory = stringify!($optional) == "M";

    crabtime::output! {
        #[doc = "Value:"]
        #[doc = ""]
        #[doc = "```rust"]
        #[doc = concat!(
            "Attribute<'static, ",
            stringify!($typ),
            "> {"
        )]
        #[doc = concat!("    code: ", stringify!($code), ",")]
        #[doc = concat!("    name: \"", stringify!($name), "\",")]
        #[doc = "    side: AttributeSide::Server,"]
        #[doc = concat!("    writable: ", stringify!({{writable}}), ",")]
        #[doc = concat!("    readable: ", stringify!({{readable}}), ",")]
        #[doc = concat!("    reportable: ", stringify!({{reportable}}), ",")]
        #[doc = concat!("    scene: ", stringify!({{scene}}), ",")]
        #[doc = concat!("    mandatory: ", stringify!({{mandatory}}), ",")]
        #[doc = concat!("    default: ", stringify!($default), ",")]
        #[doc = concat!("    min: ", stringify!($min), ",")]
        #[doc = concat!("    max: ", stringify!($max), ",")]
        #[doc = "}"]
        #[doc = "```"]
        const {{name_constant_case}}: crate::attribute::Attribute<'static, $typ> = crate::attribute::Attribute {
            code: $code,
            name: stringify!($name),
            side: crate::attribute::AttributeSide::Server,
            writable: {{writable}},
            readable: {{readable}},
            reportable: {{reportable}},
            scene: {{scene}},
            mandatory: {{mandatory}},
            default: $default,
            min: $min,
            max: $max,
        };
    }
}

macro_rules! define_attr {
    ($code:literal $name:ident $typ:ident $min:literal $max:literal $flags:tt None $optional:ident) => {
        define_attribute_raw!(
            $code,
            $name,
            crate::types::$typ,
            crate::attribute::AttributeRange::Value(crate::types::$typ($min)),
            crate::attribute::AttributeRange::Value(crate::types::$typ($max)),
            $flags,
            None,
            $optional
        );
    };

    ($code:literal $name:ident $typ:ident $min:literal $max:literal $flags:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $code,
            $name,
            crate::types::$typ,
            crate::attribute::AttributeRange::Value(crate::types::$typ($min)),
            crate::attribute::AttributeRange::Value(crate::types::$typ($max)),
            $flags,
            Some(crate::types::$typ($default)),
            $optional
        );
    };
}

macro_rules! define_attr_enum {
    ($code:literal $name:ident $typ:ident $enum:ident $flags:tt None $optional:ident) => {
        define_attribute_raw!(
            $code,
            $name,
            crate::types::$typ<$enum>,
            crate::attribute::AttributeRange::Ignore,
            crate::attribute::AttributeRange::Ignore,
            $flags,
            None,
            $optional
        );
    };

    ($code:literal $name:ident $typ:ident $enum:ident $flags:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $code,
            $name,
            crate::types::$typ<$enum>,
            crate::attribute::AttributeRange::Ignore,
            crate::attribute::AttributeRange::Ignore,
            $flags,
            Some(crate::types::$typ($enum::$default)),
            $optional
        );
    };
}

macro_rules! define_enum8 {
    ($name:ident, {
        $($variant:ident = $value:expr),* $(,)?
    }) => {
        #[repr(u8)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum $name {
            $($variant = $value),*,
            None = 0xff,
        }

        impl crate::types::ZclEnum for $name {
            const NON_VALUE: Self = Self::None;
        }
    };
}

mod globals {
    define_attr!(0xfffd ClusterRevision U16 0x0001 0xfffe R 0x0000 M);

    define_enum8!(ReportingStatus, { Pending = 0, Complete = 1, });
    define_attr_enum!(0xfffe AttributeReportingStatus Enum8 ReportingStatus R None O);
}

#[repr(u8)]
pub enum Command {
    PlaceholderTodo,
}

#[repr(u8)]
pub enum Status {
    PlaceholderTodo,
}

// struct PressureClusterMeta {
//     pub measured_value: Attribute<U16>,
// }
//
// impl PressureClusterMeta {
//     fn attrs(&self) -> [(&'static str, &'static str); 1] {
//         [("measured_value", "U16")]
//     }
// }
//
// impl PressureClusterMeta {}

pub struct Cluster<'a, Ts> {
    code: u16,
    name: &'a str,
    meta: Ts,
}

// pub mod clusters {
//     #[cfg(feature = "std")]
//     // macro_rules! attrs {
//     //     ($attrs: tt) => {
//     //     vec! $attrs
//     // }}
//     #[cfg(not(feature = "std"))]
//     macro_rules! attrs {
//         ($attrs: tt) => {
//             &$attrs
//         };
//     }
//     // pub mod general {
//     //     const BASIC: crate::Cluster = crate::Cluster {
//     //         code: 0x0000,
//     //         name: "Basic",
//     //         attributes: attrs!([]),
//     //     };
//     //
//     //     const POWER_CONFIGURATION: crate::Cluster = crate::Cluster {
//     //         code: 0x0001,
//     //         name: "Power Configuration",
//     //         attributes: attrs!([]),
//     //     };
//     //
//     //     const DEVICE_TEMPERATURE: crate::Cluster = crate::Cluster {
//     //         code: 0x0002,
//     //         name: "Device Temperature",
//     //         attributes: attrs!([]),
//     //     };
//     //
//     //     const IDENTIFY: crate::Cluster = crate::Cluster {
//     //         code: 0x0003,
//     //         name: "Identify",
//     //         attributes: attrs!([]),
//     //     };
//     // }
// }
