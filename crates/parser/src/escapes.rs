mod mnemonic {
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

mod inline_hex {
    use crate::*;

    /// EBNF: `\ x <HexadecimalDigit>+ ;` | \ X <HexadecimalDigit>+ ;`
    pub struct InlineHexEscape(NonEmptyVec<HexadecimalDigit>);
}
pub(crate) use inline_hex::InlineHexEscape;
