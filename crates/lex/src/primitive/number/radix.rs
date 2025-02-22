use crate::*;

pub trait Radix: core::fmt::Debug + PartialEq + private::Sealed {
    /// Radix specific number representation in [`RealNumberVariant::Number`]
    type Number: core::fmt::Debug + PartialEq;
}

private::impl_sealed_marker!(BinaryDigit, OctalDigit, DecimalDigit, HexadecimalDigit);

impl Radix for DecimalDigit {
    type Number = Decimal;
}

simple_radix_number!(BinaryDigit, OctalDigit, HexadecimalDigit);

macro_rules! simple_radix_number {
    ($($digit:ty),* $(,)?) => {
        $(impl Radix for $digit { type Number = NonEmptyVec<$digit>; })*
    };
}
use simple_radix_number;
