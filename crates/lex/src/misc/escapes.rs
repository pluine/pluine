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
    pub struct InlineCodePoint<'src>(char, Span<'src>);

    impl<'src> InlineCodePoint<'src> {
        pub(crate) const TERIMINATOR: char = ';';

        /// Span should point to the entire range from `\x` to the terminator `;` included.
        pub(crate) fn new(span: Span<'src>, code_point: u32) -> Result<Self, InlineCodePointScanError<'src>> {
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
    pub enum InlineCodePointScanError<'src> {
        /// The provided hex value is too large to fit inside an u32
        ///
        /// Inner span points to the entire inline hex.
        OutOfBounds(Span<'src>),
        /// The provided hex value is not a valid unicode code point
        ///
        /// Inner span points to the entire inline hex.
        InvalidCodePoint(Span<'src>),
        /// Invalid hexadecimal digit, only 0..=9, a..=f, and A..=F are allowed
        ///
        /// Inner span points to the invalid character.
        InvalidHexDigit(Span<'src>),
        /// Invalid character. Expected semicolon terminator (`;`), or
        /// hexadecimal digit (0..=9, a..=f, and A..=F)
        ///
        /// Inner span points to the invalid character.
        InvalidSequenceChar(Span<'src>),
        /// At least one hex value needs to be provided
        ///
        /// Inner span points to the entire inline hex.
        MissingDigit(Span<'src>),
        /// Reached EOF, inline hex value need to be terminated with a semicolon (`;`)
        ///
        /// Inner span points to the entire inline hex.
        EndOfFile(Span<'src>),
    }
}
pub(crate) use inline_code_point::{InlineCodePoint, InlineCodePointScanError};
