use crate::*;

#[derive(Debug, PartialEq)]
pub enum NumberLiteral<'src> {
    Binary(Number<BinaryDigit>, Span<'src>),
    Octal(Number<OctalDigit>, Span<'src>),
    Decimal(Number<DecimalDigit>, Span<'src>),
    Hexadecimal(Number<HexadecimalDigit>, Span<'src>),
}

impl Spanned for NumberLiteral<'_> {
    fn span(&self) -> Span<'_> {
        *match self {
            NumberLiteral::Binary(_, span) => span,
            NumberLiteral::Octal(_, span) => span,
            NumberLiteral::Decimal(_, span) => span,
            NumberLiteral::Hexadecimal(_, span) => span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Number<R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
}
