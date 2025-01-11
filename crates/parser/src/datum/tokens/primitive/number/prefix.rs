use std::marker::PhantomData;

/// EBNF:
/// ```
/// <Radix Binary> = #b | #B
/// <Radix Octal> = #o | #O
/// <Radix Decimal> = <empty> | #d | #D
/// <Radix Hexadecimal> = #x | #X`
/// ```
struct RadixMarker<R>(PhantomData<R>);

/// <Radix R> <Exactness> | <Exactness> <Radix R>
pub struct Prefix<R> {
    radix: PhantomData<R>,
    // NOTE: exactness can not be made public, it can only be determined by
    // looking at the entire number. 4/2 is for example an exact number, whilst
    // 4.0/2 is not
    exactness: Option<Exactness>,
}

enum Exactness {
    /// #i | #I
    Inexact,
    /// #e | #E
    Exact,
}
