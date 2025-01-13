use crate::*;

#[derive(Debug, PartialEq)]
pub enum NumberLiteral<'src> {
    Binary(Number<'src, BinaryDigit>),
    Octal(Number<'src, OctalDigit>),
    Decimal(Number<'src, DecimalDigit>),
    Hexadecimal(Number<'src, HexadecimalDigit>),
}

impl Spanned for NumberLiteral<'_> {
    fn span(&self) -> Span<'_> {
        match self {
            NumberLiteral::Binary(number) => number.span(),
            NumberLiteral::Octal(number) => number.span(),
            NumberLiteral::Decimal(number) => number.span(),
            NumberLiteral::Hexadecimal(number) => number.span(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Number<'src, R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
    span: Span<'src>,
}

impl<R: Radix> Spanned for Number<'_, R> {
    fn span(&self) -> Span<'_> {
        self.span
    }
}
