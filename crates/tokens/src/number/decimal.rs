use crate::*;

/// Standard calls this token <decimal 10>
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
