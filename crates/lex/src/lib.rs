//! Pluine Lexer
//!
//! Converts a UTF-8 string to a [`TokenStream`].
//! The output should still be high-level enough for a simple formatter.
//!
//!
//! ## Features
//!
//! (None are turned on by default.)
//!
//! ### `unicode_identifiers`
//!
//! Unicode characters are supported by default in comments, string and character literals.
//!
//! For identifiers, however, only a subset unicode general categories are allowed in the
//! specification. Some KiB of lookup tables need to be added to to the final binary size for this,
//! hence the feature flag. Enabling this feature should, however, not affect parser performance
//! by much.
//!
//! The specification is pretty ambiguous as to how unicode identifiers should be supported.
//! Lots of non-terminals forbid a certain set of ASCII characters, but then
//! proceed to allow any characters part of a given Unicode general category,
//! which may in turn include the previously forbidden character. `\` (U+005C)
//! belonging to the Po category being one such example.
//!
//! Pluine implements, therefore, unicode idedentifier support by allowing any character in the
//! allowed Unicode general categories, *unless* it is also an ASCII character which was
//! previously not permitted. Conversely, a character not part of the allowed unicode categories
//! will still be permitted if it was an allowed ASCII character./!
//!
//! ## Token modelling
//!
//! Models and parses tokens from [Revised7 Scheme Standard ch. 7.1 - Formal syntax](https://standards.scheme.org/corrected-r7rs/r7rs-Z-H-9.html#TAG:__tex2page_chap_7) into Rust types. Crate declaration are in many cases more terse by applying the following principles:
//!
//! - Token names and composition declarations may in some cases be altered to facilitate a more
//!   ergonomic Rust API. However, this should never be at the cost of changing the Scheme R7RS
//!   syntax.
//!
//! - Generalizes the `<empty>` token in grammar rules by declaring downstream usage as optional.
//!   Ex. `<decimal digit> = <empty> | #d` becomes `<downstream> = [<decimal digit>]`
//!
//! - Combines token rules by marking some terminals as optional. Ex. `<a> = <b> <c> | <b>` becomes
//!   `<a> = <b> [<c>]`.
//!
//! - Transformations are also done to disambiguate which terminals resolve to which non-terminal
//!   alternative. For example, the specification defines in a denormalized form the following
//!   non-terminals: `<uint 10> = <digit 10>+`, `<decimal> = <uint 10> [<Suffix>]`, and then `<ureal
//!   10>` = <uint 10> | <decimal>. But which alternative should should the tokenizer then resolve
//!   "10" to? `<ureal>` from `<uint>` or `<ureal>` from `<decimal>`?

#![no_std]
extern crate alloc;

mod span;
pub(crate) use span::{Span, Spanned};

mod lexer;
pub(crate) use lexer::Lexer;

mod scanner;
pub(crate) use scanner::Scanner;

mod token;
pub(crate) use token::{Token, TokenAll, TokenChar};

mod comment;
pub(crate) use comment::Comment;

mod identifier;
pub(crate) use identifier::*;

mod primitive;
pub(crate) use primitive::*;

mod misc;
pub(crate) use misc::*;

mod private;
