mod mnemonic {
    #[derive(Debug, PartialEq)]
    pub enum MnemonicEscape {
        /// EPNF: `\a`
        Alarm,
        /// EPNF: `\b`
        Backspace,
        /// EPNF: `\n`
        Newline,
        /// EPNF: `\r`
        Return,
        /// EPNF: `\t`
        Tab,
    }
}
pub(crate) use mnemonic::MnemonicEscape;

mod inline_code_point {
    use crate::*;

    /// EBNF: `\ x <HexadecimalDigit>+ ;` | \ X <HexadecimalDigit>+ ;`
    #[derive(Debug, PartialEq)]
    pub struct InlineCodePoint(char, Span);

    impl InlineCodePoint {
        pub(crate) const TERIMINATOR: char = ';';

        /// Span should point to the entire range from `\x` to the terminator `;` included.
        pub(crate) fn new(span: Span, code_point: u32) -> Result<Self, InlineCodePointScanError> {
            match char::from_u32(code_point) {
                Some(inner_char) => Ok(Self(inner_char, span)),
                None => Err(InlineCodePointScanError::InvalidCodePoint(span)),
            }
        }

        pub fn inner(&self) -> char {
            self.0
        }
    }

    #[derive(Debug, PartialEq, Spanned)]
    pub enum InlineCodePointScanError {
        /// Provided hex value is too large to fit inside an u32
        ///
        /// Inner span points to the entire inline hex
        OutOfBounds(Span),
        /// Provided hex value is not a valid unicode code point
        ///
        /// Inner span points to the entire inline hex
        InvalidCodePoint(Span),
        /// Invalid hexadecimal digit, only 0..=9, a..=f, and A..=F are allowed
        ///
        /// Inner span points to the invalid character
        InvalidHexDigit(Span),
        /// Invalid character. expected a digit (0..=9, a..=f, and A..=F) or a semicolon terminator
        /// (;)
        ///
        /// Inner span points to the invalid character
        InvalidSequenceChar(Span),
        /// No hexadecimal digit provided, at least is required
        ///
        /// Inner span points to the entire inline hex
        MissingDigit(Span),
        /// Reached EOF, inline hex values need to be terminated with a semicolon
        ///
        /// Inner span points to the entire inline hex
        EndOfFile(Span),
    }
}
pub(crate) use inline_code_point::{InlineCodePoint, InlineCodePointScanError};
