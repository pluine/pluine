mod mnemonic {
    #[derive(Debug, PartialEq)]
    pub enum MnemonicEscape {
        /// EPNF: `\a`
        Alarm,
        /// EPNF: `\b`
        Backspace,
        /// EPNF: `\t`
        Tabulation,
        /// EPNF: `\n`
        Newline,
        /// EPNF: `\r`
        Return,
    }
}
pub(crate) use mnemonic::MnemonicEscape;

mod inline_code_point {
    /// EBNF: `\ x <HexadecimalDigit>+ ;` | \ X <HexadecimalDigit>+ ;`
    #[derive(Debug, PartialEq)]
    pub struct InlineCodePoint(char);
}
pub(crate) use inline_code_point::InlineCodePoint;
