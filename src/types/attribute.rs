use crate::types::ZclType;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AttributeRange<T: ZclType> {
    // Any value but explicitly excluding the the NON_VALUE
    Value,
    // Any value. If a NON_VALUE is defined for the data type, it explicitly is interpreted as a
    // normal value instead.
    Full,
    // Any value. If a NON_VALUE is defined for the data type, it will explicitly represent None.
    FullWithNone,
    // A range [min, max] where both boundary values are included
    InclusiveRange(T, T),
    // A range [min, max] where both boundary values are included, but the specific values are
    // defined by other attributes in the same cluster for which the attribute ids are recorded
    // here.
    InclusiveRangeReference(u16, u16),
    // A maximum size in bytes.
    Size(usize),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AttributeSide {
    Server,
    Client,
    Either,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Attribute<'a, T: ZclType> {
    pub code: u16,
    pub name: &'a str,
    pub side: AttributeSide,
    pub readable: bool,
    pub writable: bool,
    pub reportable: bool,
    pub scene: bool,
    pub mandatory: bool,
    pub default: Option<T>,
    pub range: AttributeRange<T>,
}
