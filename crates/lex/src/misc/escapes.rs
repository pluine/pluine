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
    /// EBNF: `\ x <HexadecimalDigit>+ ;` | \ X <HexadecimalDigit>+ ;`
    #[derive(Debug, PartialEq)]
    pub struct InlineCodePoint(char);

    impl InlineCodePoint {
        pub(crate) const TERIMINATOR: char = ';';

        pub(crate) fn from_u32(hex_value: u32) -> Result<Self, InlineCodePointScanError> {
            char::from_u32(hex_value)
                .map(Self)
                .ok_or(InlineCodePointScanError::InvalidCodePoint)
        }

        pub fn inner(&self) -> char {
            self.0
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum InlineCodePointScanError {
        /// The provided hex value is too large to fit inside an u32
        OutOfBounds,
        /// The provided hex value is not a valid unicode code point
        InvalidCodePoint,
        /// Invalid hexadecimal digit, only 0..=9, a..=f, and A..=F are allowed
        InvalidHexDigit,
        /// Invalid character. Expected semicolon terminator (`;`), or
        /// hexadecimal digit (0..=9, a..=f, and A..=F)
        InvalidSequenceChar,
        /// At least one hex value needs to be provided
        MissingDigit,
        /// Reached EOF, inline hex value need to be terminated with a semicolon (`;`)
        EndOfFile,
    }
}
pub(crate) use inline_code_point::{InlineCodePoint, InlineCodePointScanError};
