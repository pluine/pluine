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

/// From the standard's <decimal 10>.
///
/// EBNF: `<DecimalVariant> [<Suffix>]`
pub struct Decimal {
    variant: DecimalVariant,
    suffix: Option<Suffix>,
}

pub enum DecimalVariant {
    /// Integer digits only
    ///
    /// EBNF: `<DecimalDigit>+`
    Integer(NonEmptyVec<DecimalDigit>),
    /// Fraction digits only
    ///
    /// EBNF: `. <DecimalDigit>+`
    Fraction { fraction_digits: NonEmptyVec<DecimalDigit> },
    /// Both integer and fraction digits present
    ///
    /// EBNF: `<DecimalDigit>+ . <DecimalDigit>*`
    Both {
        digits: NonEmptyVec<DecimalDigit>,
        fractional_digits: Vec<DecimalDigit>,
    },
}
