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
    NewlineEscape(StringNewlineEscape),
    MnemonicEscape(MnemonicEscape),
    StringEscape(StringEscape),
    /// Any character other than `"` and `\`
    ///
    /// Stores all characters as one continuous string slice until the scanner
    /// encounters another `StringElement` variant. This should significantly
    /// reduce the number of elements pushed into [`StringLiteral`].
    Chars(&'src str),
}

/// EBNF: `\ <IntralineWhitespace>* <LineEnding>`
// NOTE: trailing intraline whitespace captured by [`StringElement::Chars`]
#[derive(Debug, PartialEq)]
pub struct StringNewlineEscape {
    pub(crate) line_ending: LineEnding,
    pub(crate) leading_whitespace: Vec<IntralineWhitespace>,
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

#[derive(Debug, PartialEq, Error)]
pub enum StringLiteralScanError {
    #[error("invalid inline code point (inline hex escape)")]
    InlineHex(#[from] InlineCodePointScanError),
    #[error("end of file reached, no closing '\"' found")]
    EndOfFile(Span),
    #[error("unknown escape character, expected one of `\"', '\\', '|', 'x', 'X', 'a', 'b', 't', 'n', '<tab>' or '<space>'")]
    UnknownEscape(Span),
    #[error("invalid whitespace character following a newline escape ('\\'), only space and tab are allowed before the mandatory newline")]
    UnknownWhitespace(Span),
}

pub(crate) struct StringElementCollector<'src> {
    string_elements: Vec<StringElement<'src>>,
    // keeping track of start index for `StringElement::Chars`
    chars_state: Option<usize>,
    src: &'src str,
}

impl<'src> StringElementCollector<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            string_elements: Default::default(),
            chars_state: Default::default(),
            src,
        }
    }

    pub fn push_inline_code_point(&mut self, start: usize, inline_code_point: InlineCodePoint) {
        self.maybe_end_chars(start);
        self.string_elements.push(StringElement::InlineCodePoint(inline_code_point));
    }

    /// `start` is index pointing at the backslash
    pub fn push_string_escape(&mut self, start: usize, escape: StringEscape) {
        self.maybe_end_chars(start);
        self.string_elements.push(StringElement::StringEscape(escape));
    }

    /// `start` is index pointing at the backslash
    pub fn push_mnemonic_escape(&mut self, start: usize, escape: MnemonicEscape) {
        self.maybe_end_chars(start);
        self.string_elements.push(StringElement::MnemonicEscape(escape));
    }

    /// `start` is index pointingb at the backslash
    pub fn push_newline_escape(&mut self, start: usize, line_ending: LineEnding, leading_whitespace: Vec<IntralineWhitespace>) {
        self.maybe_end_chars(start);

        self.string_elements.push(StringElement::NewlineEscape(StringNewlineEscape {
            leading_whitespace,
            line_ending,
        }));
    }

    pub fn finalize(mut self, end: usize) -> Vec<StringElement<'src>> {
        self.maybe_end_chars(end);

        self.string_elements
    }

    pub fn maybe_begin_chars(&mut self, start: usize) {
        if self.chars_state.is_none() {
            self.chars_state = Some(start)
        }
    }

    /// Trailing LF in a CRLF line ending possibly encoundered from a a newline escape
    pub fn maybe_update_line_ending(&mut self, char_index: usize) {
        if let Some(StringElement::NewlineEscape(StringNewlineEscape { line_ending, .. })) = self.string_elements.last_mut() {
            if line_ending == &LineEnding::Return {
                *line_ending = LineEnding::ReturnNewline;
            }
        } else {
            self.maybe_begin_chars(char_index)
        }
    }

    fn maybe_end_chars(&mut self, end: usize) {
        if let Some(char_start) = self.chars_state.take() {
            let chars = &self.src[char_start..end];
            self.string_elements.push(StringElement::Chars(chars));
        }
    }
}
