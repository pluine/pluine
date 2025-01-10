/// EBNF: <Space> | <Tab>
pub enum IntralineWhitespace {
    Space,
    Tab,
}

pub enum LineEnding {
    /// LF
    ///
    /// EBNF: `<return>`
    Return,
    /// LF
    ///
    /// EBNF: `<newline>`
    Newline,
    /// CRLF
    ///
    /// EBNF: `<return> <newline>`
    ReturnNewline,
}

/// EBNF: `<IntralineWhitespace> | <LineEnding>`
pub enum Whitespace {
    Intraline(IntralineWhitespace),
    Interline(LineEnding),
}
