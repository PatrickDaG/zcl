#[crabtime::function]
fn define_attribute_raw(
    pattern!(
        $code:literal,
        $name:ident,
        $type:expr,
        $min:expr,
        $max:expr,
        $access:tt,
        $default:expr,
        $optional:ident
    ): _,
) {
    #![dependency(convert_case = "0.8")]

    use convert_case::{Case, Casing};
    let name_constant_case = stringify!($name).to_case(Case::Constant);
    let access = stringify!($access);
    let readable = access.contains('R');
    let writable = access.contains('W');
    let reportable = access.contains('P');
    let scene = access.contains('S');
    let mandatory = stringify!($optional) == "M";

    crabtime::output! {
        #[doc = "```rust"]
        #[doc = "Attribute {"]
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
        pub const {{name_constant_case}}: crate::types::attribute::Attribute<'static, $type> = crate::types::attribute::Attribute {
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

macro_rules! define_enum {
    ($type:path, $name:ident, {
        $($variant:ident = $value:expr),* $(,)?
    }) => {
        #[repr($type)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum $name {
            $($variant = $value),*,
            None = <$type>::MAX,
        }

        impl crate::types::ZclEnum for $name {
            const NON_VALUE: Self = Self::None;
        }
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! map_kind_to_type {
    (nodata) => {
        $crate::types::NoData
    };
    (data8) => {
        $crate::types::Data8
    };
    (data16) => {
        $crate::types::Data16
    };
    (data24) => {
        $crate::types::Data24
    };
    (data32) => {
        $crate::types::Data32
    };
    (data40) => {
        $crate::types::Data40
    };
    (data48) => {
        $crate::types::Data48
    };
    (data56) => {
        $crate::types::Data56
    };
    (data64) => {
        $crate::types::Data64
    };
    (bool) => {
        $crate::types::Bool
    };
    (map8) => {
        $crate::types::Bitmap8
    };
    (map16) => {
        $crate::types::Bitmap16
    };
    (map24) => {
        $crate::types::Bitmap24
    };
    (map32) => {
        $crate::types::Bitmap32
    };
    (map40) => {
        $crate::types::Bitmap40
    };
    (map48) => {
        $crate::types::Bitmap48
    };
    (map56) => {
        $crate::types::Bitmap56
    };
    (map64) => {
        $crate::types::Bitmap64
    };
    (uint8) => {
        $crate::types::U8
    };
    (uint16) => {
        $crate::types::U16
    };
    (uint24) => {
        $crate::types::U24
    };
    (uint32) => {
        $crate::types::U32
    };
    (uint40) => {
        $crate::types::U40
    };
    (uint48) => {
        $crate::types::U48
    };
    (uint56) => {
        $crate::types::U56
    };
    (uint64) => {
        $crate::types::U64
    };
    (int8) => {
        $crate::types::I8
    };
    (int16) => {
        $crate::types::I16
    };
    (int24) => {
        $crate::types::I24
    };
    (int32) => {
        $crate::types::I32
    };
    (int40) => {
        $crate::types::I40
    };
    (int48) => {
        $crate::types::I48
    };
    (int56) => {
        $crate::types::I56
    };
    (int64) => {
        $crate::types::I64
    };
    // (enum8)     => { $crate::types::Enum8 };
    // (enum16)    => { $crate::types::Enum16 };
    // (semi)      => { $crate::types:: };
    (single) => {
        $crate::types::F32
    };
    (double) => {
        $crate::types::F64
    };
    (octstr) => {
        $crate::types::OctetString::<'static>
    };
    (string) => {
        $crate::types::CharacterString::<'static>
    };
    (octstr16) => {
        $crate::types::LongOctetString::<'static>
    };
    (string16) => {
        $crate::types::LongCharacterString::<'static>
    };
    // (ASCII)     => { $crate::types:: };
    // (array)     => { $crate::types::Array };
    // (struct)    => { $crate::types::Structure };
    // (set)       => { $crate::types:: };
    // (bag)       => { $crate::types:: };
    (ToD) => {
        $crate::types::TimeOfDay
    };
    (date) => {
        $crate::types::Date
    };
    (UTC) => {
        $crate::types::UtcTime
    };
    (clusterId) => {
        $crate::types::ClusterId
    };
    (attribId) => {
        $crate::types::AttributeId
    };
    (bacOID) => {
        $crate::types::BacnetOid
    };
    (EUI64) => {
        $crate::types::IeeeAddress
    };
    (key128) => {
        $crate::types::SecurityKey
    };
    (unk) => {
        $crate::types::Unknown
    };
    ($other:tt) => {
        $other
    };
}

macro_rules! define_attr {
    ($id:literal $name:ident enum8, $enum:ty, $access:tt None $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            crate::types::Enum8::<$enum>,
            crate::types::attribute::AttributeRange::Ignore,
            crate::types::attribute::AttributeRange::Ignore,
            $access,
            None,
            $optional
        );
    };
    ($id:literal $name:ident octstr, $max_bytes:literal $access:tt $default:expr, $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!(octstr),
            crate::types::attribute::AttributeRange::Size(0),
            crate::types::attribute::AttributeRange::Size($max_bytes),
            $access,
            Some(map_kind_to_type!(octstr)(Some($default))),
            $optional
        );
    };
    ($id:literal $name:ident octstr, $access:tt $default:expr, $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!(octstr),
            crate::types::attribute::AttributeRange::Ignore,
            crate::types::attribute::AttributeRange::Ignore,
            $access,
            Some(map_kind_to_type!(octstr)(Some($default))),
            $optional
        );
    };
    ($id:literal $name:ident string, $max_bytes:literal $access:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!(string),
            crate::types::attribute::AttributeRange::Size(0),
            crate::types::attribute::AttributeRange::Size($max_bytes),
            $access,
            Some(map_kind_to_type!(string)(Some($default))),
            $optional
        );
    };
    ($id:literal $name:ident string, $access:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!(string),
            crate::types::attribute::AttributeRange::Ignore,
            crate::types::attribute::AttributeRange::Ignore,
            $access,
            Some(map_kind_to_type!(string)(Some($default))),
            $optional
        );
    };
    ($id:literal $name:ident bool, $access:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!(bool),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!(bool)(Some(false))),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!(bool)(Some(true))),
            $access,
            Some(map_kind_to_type!(bool)(Some($default))),
            $optional
        );
    };
    ($id:literal $name:ident $kind:path, $min:literal $max:literal $access:tt None $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!($kind),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!($kind)($min)),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!($kind)($max)),
            $access,
            None,
            $optional
        );
    };
    ($id:literal $name:ident $kind:path, $min:literal $max:literal $access:tt $default:literal $optional:ident) => {
        define_attribute_raw!(
            $id,
            $name,
            map_kind_to_type!($kind),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!($kind)($min)),
            crate::types::attribute::AttributeRange::Value(map_kind_to_type!($kind)($max)),
            $access,
            Some(map_kind_to_type!($kind)($default)),
            $optional
        );
    };
}

macro_rules! define_cluster {
    ($cluster_name:ident $cluster_id:literal [
        $( ($id:literal $name:ident $kind:path, $($args:tt)+)),* $(,)?
    ]) => { paste::paste! {
        pub mod [<$cluster_name:snake:lower>] {
            pub struct [< $cluster_name Attrs >] {
                $(pub [<$name:snake:lower>]: $crate::types::attribute::Attribute<'static, map_kind_to_type!($kind)>,)+
            }

            impl [< $cluster_name Attrs >] {
                pub fn attrs(&self) -> [(&'static str, &'static str); count!($($name)+)] {
                    [
                        $((stringify!($name), stringify!(map_kind_to_type!($kind))),)+
                    ]
                }
            }

            $(define_attr!($id $name $kind, $($args)+);)*
        }

        #[doc = "```rust"]
        #[doc = "Cluster {"]
        #[doc = concat!("    code: ", stringify!($cluster_id), ",")]
        #[doc = concat!("    name: \"", stringify!($cluster_name), "\",")]
        #[doc = concat!("    meta: ", stringify!([<$cluster_name:snake:lower>]), "::", stringify!([< $cluster_name Attrs >]), " {")]
        $(
        #[doc = concat!("        ", stringify!([<$name:snake:lower>]), ": ", stringify!([<$cluster_name:snake:lower>]), "::", stringify!([<$name:snake:upper>]), ",")]
        )*
        #[doc = "    }"]
        #[doc = "}"]
        #[doc = "```"]
        #[doc = ""]
        $(
        #[doc = concat!("- ", stringify!([<$name:snake:lower>]), ": [`", stringify!([<$cluster_name:snake:lower>]), "::", stringify!([<$name:snake:upper>]), "`]")]
        )*
        pub const [< $cluster_name:snake:upper _CLUSTER >]: $crate::Cluster< [<$cluster_name:snake:lower>]::[< $cluster_name Attrs >]> = $crate::Cluster {
            code: $cluster_id,
            name: stringify!($cluster_name),
            meta: [<$cluster_name:snake:lower>]::[< $cluster_name Attrs >] {
            $([<$name:snake:lower>]: [<$cluster_name:snake:lower>]::[<$name:snake:upper>],)*
            }
        };
    }};
}

pub mod global {
    define_attr!(0xfffd ClusterRevision uint16, 0x0001 0xfffe R 0x0000 M);

    define_enum!(u8, ReportingStatus, { Pending = 0, AttributeReportingComplete = 1, });
    define_attr!(0xfffe AttributeReportingStatus enum8, ReportingStatus, R None O);
}

pub mod general {
    define_cluster! {
        Basic 0x0000 [
            (0x0000 ZclVersion                 uint8 ,  0x00 0xff  R  8  M),
            // (0x0001 ApplicationVersion         uint8 ,  0x00 0xff  R  0  O),
            // (0x0002 StackVersion               uint8 ,  0x00 0xff  R  0  O),
            // (0x0003 HwVersion                  uint8 ,  0x00 0xff  R  0  O),
            // (0x0004 ManufacturerName           string,  32         R  "" O),
            // (0x0005 ModelIdentifier            string,       32    R  "" O),
            // (0x0006 DateCode                   string,       16    R  "" O),
            // // (0x0007 PowerSource                enum8 ,  0x00 0xff  R  0x00         M),
            // // (0x0008 GenericDeviceClass         enum8 ,  0x00 0xff  R  0xff         O),
            // // (0x0009 GenericDeviceType          enum8 ,  0x00 0xff  R  0xff         O),
            // (0x000a ProductCode                octstr,             R  &[], O),
            // (0x000b ProductUrl                 string,             R  "" O),
            // (0x000c ManufacturerVersionDetails string,             R  "" O),
            // (0x000d SerialNumber               string,             R  "" O),
            // (0x000e ProductLabel               string,             R  "" O),
            // (0x0010 LocationDescription        string,       16    RW "" O),
            // // (0x0011 PhysicalEnvironment        enum8 ,  desc       RW 0            O),
            // (0x0012 DeviceEnabled              bool  ,             RW true            O),
            // // (0x0013 AlarmMask                  map8  ,  000000xx   RW 0            O),
            // // (0x0014 DisableLocalConfig         map8  ,  000000xx   RW 0            O),
            // (0x4000 SwBuildId                  string,       16    R  "" O),
        ]
    }
}
