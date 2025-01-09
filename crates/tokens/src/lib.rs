//! Pluine language token declarations
//!
//! Implements [Revised7 Scheme Standard ch. 7.1 - Formal syntax](https://standards.scheme.org/corrected-r7rs/r7rs-Z-H-9.html#TAG:__tex2page_chap_7) by applying the following crate-wide adaptations:
//!
//! - Token names and composition declarations may in some cases be reworked to facilitate an
//!   ergonomic Rust API.
//!
//! - EBNF: `...` comments should denote a proper EBNF and not a pseudoform thereof.
//!
//! - Generalizes the `<empty>` token in grammar rules by declaring downstream usage as optional.
//!   Ex. `<decimal digit> = <empty> | #d` becomes `<downstream> = [<decimal digit>]`

mod identifier;
pub(crate) use identifier::Identifier;

mod number;
pub(crate) use number::*;

mod containers;
pub(crate) use containers::*;

mod private;
