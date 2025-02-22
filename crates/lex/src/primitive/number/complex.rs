use crate::*;

#[derive(Debug, PartialEq)]
pub enum ComplexNumber<R: Radix> {
    /// EBNF: `<RealNumber>`
    Real(RealNumber<R>),
    /// EBNF: `<RealNumber> @ <RealNumber>`
    Polar { magnitude: RealNumber<R>, phase: RealNumber<R> },
    /// Number in the rectangular complex form where the imaginary component
    /// is a valid number (neither NaN nor Infinity).
    ///
    /// EBNF: `[<RealNumber>] <Sign> [<RealNumberVariant>] i`
    RectangularValid {
        real: Option<RealNumber<R>>,
        sign: Sign,
        imaginary: RealNumberVariant<R>,
    },
    /// Number in the rectangular complex form where the imaginary component
    /// is not a valid number.
    ///
    /// EBNF: `[<RealNumber>] <NonNumber> i`
    RectangularInvalid { real: Option<RealNumber<R>>, imaginary: NonNumber },
}
