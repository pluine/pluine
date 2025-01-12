use crate::*;

#[derive(Debug, PartialEq)]
pub enum RealNumber<R: Radix> {
    Number { sign: Option<Sign>, variant: RealNumberVariant<R> },
    NonNumber(NonNumber),
}

#[derive(Debug, PartialEq)]
pub enum RealNumberVariant<R: Radix> {
    /// Simple fraction representation
    ///
    /// EBNF: `<Digit R>+ / <Digit R>+`
    Fraction {
        numerator: NonEmptyVec<R>,
        denominator: NonEmptyVec<R>,
    },
    /// Number representation when not a simple fraction
    ///
    /// Note that decimal radix may still represent a fraction with a non-simple
    /// fraction form, say "1.5".
    ///
    /// EBNF: `<BinaryDigit>+ | <OctalDigit>+ | <HexadecimalDigit>+` | <Decimal>
    Number(R::Number),
}
