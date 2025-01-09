//! Pluine language token declarations
//!
//! Models tokens from [Revised7 Scheme Standard ch. 7.1 - Formal syntax](https://standards.scheme.org/corrected-r7rs/r7rs-Z-H-9.html#TAG:__tex2page_chap_7) into Rust types by applying the applying the following adaptations:
//!
//! - Token names and composition declarations may in some cases be altered to facilitate a more
//!   ergonomic Rust API. However, this should never be at the cost of changing the Scheme R7RS
//!   syntax.
//!
//! - EBNF: `...` comments should denote ISO/IEC 14977 EBNF and not a pseudoform thereof.
//!
//! - Combines token rules by marking some terminals as optional. Ex. `<a> = <b> <c> | <b>` becomes
//!   `<a> = <b> [<c>]`.
//!
//! - Generalizes the `<empty>` token in grammar rules by declaring downstream usage as optional.
//!   Ex. `<decimal digit> = <empty> | #d` becomes `<downstream> = [<decimal digit>]`
//!
//! - Transformations are also done to disambiguate which terminals resolve to which non-terminal
//!   alternative. For example, the specification defines in a denormalized form the following
//!   non-terminals: `<uint 10> = <digit 10>+`, `<decimal> = <uint 10> [<Suffix>]`, and then `<ureal
//!   10>` = <uint 10> | <decimal>. But which alternative should should the tokenizer then resolve
//!   "10" to? `<ureal>` from `<uint>` or `<ureal>` from `<decimal>`?

mod number;
pub(crate) use number::*;

mod bytes;
pub(crate) use bytes::{Byte, ByteVector};

mod identifier;
pub(crate) use identifier::Identifier;

mod containers;
pub(crate) use containers::*;

mod private;
