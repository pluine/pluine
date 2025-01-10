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
//!
//! ## Features
//!
//! (None are turned on by default.)
//!
//! ### `unicode`
//!
//! Adds unicode support. Normally, only ASCII-characters are permitted  identifiers, strings and
//! characters etc.
//!
//! ### Characters
//!
//! "Normally" typed character literals won't accept non-ASCII characters in neither normal nor hex
//! form if this feature is disabled. For `#\😎` and `#\x1F60E` respectively will at the parse stage
//! return an error, unless the unicode feature is enabled. (Characters written with in their hex
//! form use the value of their corresponding unicode code point.)
//!
//! ### Identifiers
//!
//! For identifiers, only a subset unicode general categories are allowed. Enabling unicode support
//! means because of this that some KiB of lookup tables need to be added to to the final binary
//! size. The parser performance difference should on the other hand be practically negligible.
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
//! will still be permitted if it was an allowed ASCII character.

mod bytes;
pub(crate) use bytes::{Byte, ByteVector};

mod boolean;
pub(crate) use boolean::Boolean;

mod character;
pub(crate) use character::*;

mod number;
pub(crate) use number::*;

mod whitespace;
pub(crate) use whitespace::*;

mod escapes;
pub(crate) use escapes::*;

mod string;
pub(crate) use string::*;

mod identifier;
pub(crate) use identifier::*;

mod delimiters;

mod containers;
pub(crate) use containers::*;

mod private;
