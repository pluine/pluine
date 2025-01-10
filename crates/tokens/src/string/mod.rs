mod mnemonic_escape {
    pub enum MnemonicEscape {
        /// EPNF: `\a`
        Alert,
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
pub(crate) use mnemonic_escape::MnemonicEscape;

mod inline_hex_escape {
    use crate::*;

    /// EBNF: `\ x <HexadecimalDigit>+ ;`
    pub struct InlineHexEscape(NonEmptyVec<HexadecimalDigit>);
}

mod whitespace;
pub(crate) use whitespace::*;
