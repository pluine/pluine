use crate::*;

/// Infinities (inf) and Not a Number (nan). Renamed from the standard's <infnan>
#[derive(Debug, PartialEq)]
pub struct NonNumber {
    sign: Sign,
    variant: NonNumberVariant,
}

#[derive(Debug, PartialEq)]
pub enum NonNumberVariant {
    /// +inf.0 | -inf.0 | +INF.0 | -INF.0
    Infinity,
    /// +nan.0 | -nan.0 | +NAN.0 | -NAN.0
    Invalid,
}
