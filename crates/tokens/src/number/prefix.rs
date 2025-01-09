use std::marker::PhantomData;

use crate::*;

/// <Radix> <Exactness> | <Exactness> <Radix>
pub struct Prefix<R: Digit> {
    radix: Radix<R>,
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

/// Binary - EBNF `#b | #B`,
/// Octal - EBNF `#o | #O`,
/// Decimal - EBNF `<empty> | #d | #D`,
/// Hexadecimal - EBNF `#x | #X`,
pub struct Radix<R: Digit>(PhantomData<R>);
