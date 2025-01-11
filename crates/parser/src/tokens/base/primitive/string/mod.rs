use crate::*;

/// EBNF: `" <StringElement>* "`
pub struct StringLiteral(Vec<StringElement>);

pub enum StringElement {
    InlineCodePoint(InlineCodePoint),
    NewlineEscape(LineEnding),
    MnemonicEscape(MnemonicEscape),
    StringEscape(StringEscape),
    /// Any character other than `"` and `\`
    Other(char),
}

/// EBNF: `\ <IntralineWhitespace>* <LineEnding>`
// NOTE: trailing intraline whitespace captured by [`StringElement::Other`]
pub struct StringNewlineEscape {
    leading_whitespace: Vec<IntralineWhitespace>,
    line_ending: LineEnding,
}

pub enum StringEscape {
    /// EBNF: `\"`
    Quote,
    /// EBNF: `\\`
    Backslash,
    /// EBNF: `\|`
    VerticalLine,
}
