//! Pluine Language parsing to an AST.
//!
//! Currently done by using the [scannerless](https://en.wikipedia.org/wiki/Scannerless_parsing) `chumsky` parsing library.
//! Chumsky being a recursive decent parser over and LR parser should be fine given the unambiguity
//! ! of S-expressions.

mod parser;
pub(crate) use parser::RuineParser;

mod ast;
pub(crate) use ast::*;
