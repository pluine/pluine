use alloc::vec::Vec;

use thiserror::Error;

use crate::*;

/// EBNF: `" <StringElement>* "`
#[derive(Debug, PartialEq, Spanned)]
pub struct StringLiteral<'src> {
    pub(crate) inner: Vec<StringElement<'src>>,
    #[span]
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq)]
pub enum StringElement<'src> {
    InlineCodePoint(InlineCodePoint),
    NewlineEscape(LineEnding),
    MnemonicEscape(MnemonicEscape),
    StringEscape(StringEscape),
    /// Any character other than `"` and `\`
    ///
    /// Stores all characters as one continuous string slice until the scanner
    /// encounters another `StringElement` variant. This should significantly
    /// reduce the number of elements pushed into [`StringLiteral`].
    Other(&'src str),
}

/// EBNF: `\ <IntralineWhitespace>* <LineEnding>`
// NOTE: trailing intraline whitespace captured by [`StringElement::Other`]
pub struct StringNewlineEscape {
    leading_whitespace: Vec<IntralineWhitespace>,
    line_ending: LineEnding,
}

#[derive(Debug, PartialEq)]
pub enum StringEscape {
    /// EBNF: `\"`
    DoubleQuote,
    /// EBNF: `\\`
    Backslash,
    /// EBNF: `\|`
    VerticalLine,
}

#[derive(Debug, Error)]
pub enum StringLiteralScanError {
    #[error("invalid inline code point (inline hex escape)")]
    InlineHex(#[from] InlineCodePointScanError),
    #[error("end of file reached, no closing '\"' found")]
    EndOfFile(Span),
}
