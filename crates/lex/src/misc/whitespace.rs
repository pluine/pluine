/// EBNF: <Space> | <Tab>
#[derive(Debug, PartialEq)]
pub enum IntralineWhitespace {
    Space,
    Tab,
}

#[derive(Debug, PartialEq)]
pub enum LineEnding {
    /// LF (\r)
    ///
    /// EBNF: `<return>`
    Return,
    /// LF (\n)
    ///
    /// EBNF: `<newline>`
    Newline,
    /// CRLF (\r\n)
    ///
    /// EBNF: `<return> <newline>`
    ReturnNewline,
}

impl LineEnding {
    pub(crate) fn is_line_ending(char: char) -> bool {
        char == '\n' || char == '\r'
    }
}
