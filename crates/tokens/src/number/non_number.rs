use crate::*;

/// Infinities (inf) and Not a Number (nan). Renamed from the standard's <infnan>
pub struct NonNumber {
    sign: Sign,
    variant: NonNumberVariant,
}

pub enum NonNumberVariant {
    /// +inf.0 | -inf.0
    Infinity,
    /// +nan.0 | -nan.0
    Invalid,
}
