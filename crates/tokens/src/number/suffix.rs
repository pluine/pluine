use crate::*;

/// e | E
struct ExponentMarker;

/// Used denote exponentiation
///
/// EBNF: `<ExponentMarker> [<Sign>] <DecimalDigit>+`
pub struct Suffix {
    sign: Option<Sign>,
    digits: NonEmptyVec<DecimalDigit>,
}

pub enum Sign {
    /// +
    Plus,
    /// -
    Minus,
}
