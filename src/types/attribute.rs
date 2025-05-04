use crate::types::ZclType;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AttributeRange<T: ZclType> {
    /// This range has no meaning, e.g. min/max of an enumeration
    Ignore,
    /// Another attribute with the given attribute id specifies this value
    Attribute(u16),
    /// A size limit in bytes
    Size(usize),
    /// A specific value
    Value(T),
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
    pub writable: bool,
    pub readable: bool,
    pub reportable: bool,
    pub scene: bool,
    pub mandatory: bool,
    pub default: Option<T>,
    pub min: AttributeRange<T>,
    pub max: AttributeRange<T>,
}

// impl Attribute<> {
//     fn ty(&self) -> &'static str {
//         T::ty() // e.g. crate::types::U16
//     }
// }
