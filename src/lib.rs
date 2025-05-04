use thiserror::Error;

pub mod attributes;
pub mod types;

#[derive(Error, Debug)]
pub enum ZclError {
    #[error("failed to serialize value")]
    Serialization,
    #[error("value is out of range")]
    ValueOutOfRange,
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
