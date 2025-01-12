/// EBNF: <Space> | <Tab>
pub enum IntralineWhitespace {
    Space,
    Tab,
}

#[derive(Debug, PartialEq)]
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

impl LineEnding {
    pub(crate) fn is_line_ending(char: char) -> bool {
        char == '\n' || char == '\r'
    }
}
