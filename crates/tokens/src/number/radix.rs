use crate::*;

pub trait Radix: private::Sealed {
    /// Radix specific number representation in [`RealNumberVariant::Number`]
    type Number;
}

private::impl_sealed_marker!(Radix, BinaryDigit, OctalDigit, DecimalDigit, HexadecimalDigit);

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
