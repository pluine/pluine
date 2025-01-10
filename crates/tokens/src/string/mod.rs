use crate::*;

pub enum StringElement {
    Newline(StringNewline)
    HexEscape(InlineHexEscape),
}

/// Note: final intraline whitespace
/// EBNF: `\ <IntralineWhitespace>* <LineEnding> <IntralineWhitespace>*`
pub struct StringNewline {
    leading_whitespace: Vec<IntralineWhitespace>,
    line_ending: LineEnding,
}
