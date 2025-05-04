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
        pub const {{name_constant_case}}: crate::types::attribute::Attribute<'static, $typ> = crate::types::attribute::Attribute {
            code: $code,
            name: stringify!($name),
            side: crate::types::attribute::AttributeSide::Server,
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
            crate::types::attribute::AttributeRange::Value(crate::types::$typ($min)),
            crate::types::attribute::AttributeRange::Value(crate::types::$typ($max)),
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
            crate::types::attribute::AttributeRange::Value(crate::types::$typ($min)),
            crate::types::attribute::AttributeRange::Value(crate::types::$typ($max)),
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
            crate::types::attribute::AttributeRange::Ignore,
            crate::types::attribute::AttributeRange::Ignore,
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
            crate::types::attribute::AttributeRange::Ignore,
            crate::types::attribute::AttributeRange::Ignore,
            $flags,
            Some(crate::types::$typ($enum::$default)),
            $optional
        );
    };
}

macro_rules! define_enum {
    ($typ:ty, $name:ident, {
        $($variant:ident = $value:expr),* $(,)?
    }) => {
        #[repr($typ)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum $name {
            $($variant = $value),*,
            None = <$typ>::MAX,
        }

        impl crate::types::ZclEnum for $name {
            const NON_VALUE: Self = Self::None;
        }
    };
}

pub mod global {
    define_attr!(0xfffd ClusterRevision U16 0x0001 0xfffe R 0x0000 M);

    define_enum!(u8, ReportingStatus, { Pending = 0, Complete = 1, });
    define_attr_enum!(0xfffe AttributeReportingStatus Enum8 ReportingStatus R None O);
}
