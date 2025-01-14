use crate::*;

#[derive(Debug, PartialEq, Spanned)]
pub enum NumberLiteral<'src> {
    Binary(Number<'src, BinaryDigit>),
    Octal(Number<'src, OctalDigit>),
    Decimal(Number<'src, DecimalDigit>),
    Hexadecimal(Number<'src, HexadecimalDigit>),
}

#[derive(Debug, PartialEq, Spanned)]
pub struct Number<'src, R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
    #[span]
    span: Span<'src>,
}
