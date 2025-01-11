use crate::*;

pub enum NumberLiteral {
    Binary(Number<BinaryDigit>),
    Octal(Number<OctalDigit>),
    Decimal(Number<DecimalDigit>),
    Hexadecimal(Number<HexadecimalDigit>),
}

pub struct Number<R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
}
