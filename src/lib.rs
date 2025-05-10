#![no_std]
use thiserror::Error;

pub mod types;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Error, PartialEq, Debug, Copy, Clone)]
pub enum ZclError {
    #[error("failed to serialize value")]
    Serialization,
    #[error("value is out of range")]
    ValueOutOfRange,
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    PlaceholderTodo,
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Status {
    /// Operation was successful
    Success = 0x00,
    /// Operation was not successful
    Failure = 0x01,
    /// The sender of the command does not have authorization to carry out this command
    NotAuthorized = 0x7e,
    /// Unknown purpose
    Reserved7f = 0x7f,
    /// The command appears to contain the wrong fields, as detected either by the presence of one or more invalid field entries or by there being missing fields. Command not carried out. Implementer has discretion as to whether to return this error or INVALID_FIELD
    MalformedCommand = 0x80,
    /// The specified command is not supported on the device. Command not carried out
    UnsupCommand = 0x81,
    /// At least one field of the command contains an incorrect value, according to the specification the device is implemented to
    InvalidField = 0x85,
    /// The specified attribute does not exist on the device
    UnsupportedAttribute = 0x86,
    /// Out of range error or set to a reserved value. Attribute keeps its old value. Note that an attribute value may be out of range if an attribute is related to another, e.g., with minimum and maximum attributes. See the individual attribute descriptions for specific details
    InvalidValue = 0x87,
    /// Attempt to write a read-only attribute
    ReadOnly = 0x88,
    /// An operation failed due to an insufficient amount of free space available
    InsufficientSpace = 0x89,
    /// The requested information (e.g., table entry) could not be found
    NotFound = 0x8b,
    /// Periodic reports cannot be issued for this attribute
    UnreportableAttribute = 0x8c,
    /// The data type given for an attribute is incorrect. Command not carried out
    InvalidDataType = 0x8d,
    /// The selector for an attribute is incorrect
    InvalidSelector = 0x8e,
    /// The supplied values (e.g., contents of table cells) are inconsistent
    Reserved92 = 0x92,
    /// The exchange was aborted due to excessive response time
    Timeout = 0x94,
    /// Failed case when a client or a server decides to abort the upgrade process
    Abort = 0x95,
    /// Invalid OTA upgrade image (ex. failed signature validation or signer information check or CRC check)
    InvalidImage = 0x96,
    /// Server does not have data block available yet
    WaitForData = 0x97,
    /// No OTA upgrade image available for the client
    NoImageAvailable = 0x98,
    /// The client still requires more OTA upgrade image files to successfully upgrade
    RequireMoreImage = 0x99,
    /// The command has been received and is being processed
    NotificationPending = 0x9a,
    /// An error occurred during calibration
    ReservedC2 = 0xc2,
    /// The cluster is not supported
    UnsupportedCluster = 0xc3,
    /// Something else
    Invalid(u8),
}

pub struct Cluster<'a, Ts> {
    code: u16,
    name: &'a str,
    meta: Ts,
}
