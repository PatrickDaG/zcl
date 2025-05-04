use crate::types::ZclType;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AttributeRange<T: ZclType> {
    Attr(u16),
    Size(usize),
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
    pub default: T,
    pub min: AttributeRange<T>,
    pub max: AttributeRange<T>,
}
