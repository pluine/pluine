use std::marker::PhantomData;

use crate::*;

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
    exactness: Option<Exactness>,
}

pub enum Exactness {
    /// #i | #I
    Inexact,
    /// #e | #E
    Exact,
    // TODO: deprecate with upstream Option<T>
    /// <empty>
    Unknown,
}
