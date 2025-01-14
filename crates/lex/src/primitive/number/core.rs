use crate::*;

#[derive(Debug, PartialEq, Spanned)]
pub enum NumberLiteral {
    Binary(Number<BinaryDigit>),
    Octal(Number<OctalDigit>),
    Decimal(Number<DecimalDigit>),
    Hexadecimal(Number<HexadecimalDigit>),
}

#[derive(Debug, PartialEq, Spanned)]
pub struct Number<R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
    #[span]
    span: Span,
}
