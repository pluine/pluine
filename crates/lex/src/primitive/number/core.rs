use crate::*;

#[derive(Debug, PartialEq)]
pub enum NumberLiteral {
    Binary(Number<BinaryDigit>),
    Octal(Number<OctalDigit>),
    Decimal(Number<DecimalDigit>),
    Hexadecimal(Number<HexadecimalDigit>),
}

#[derive(Debug, PartialEq)]
pub struct Number<R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
}
