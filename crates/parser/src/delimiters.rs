use crate::*;

/// EBNF: `|`
pub struct VerticalLine;

/// EBNF: `<Whitespace> | <VerticalLine> | ( | ) | " | ;`
pub enum Delimiter {
    Whitespace(Whitespace),
    VerticalLine(VerticalLine),
    RoundOpenBracket,
    RoundCloseBracket,
    DoubleQuotes,
    Semicolon,
}
