use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Comment<'src> {
    /// EBNF-ish: `<all characters up to a line ending>`
    ///
    /// String does not include the leading semi-colon, nor the line ending.
    Semicolon(&'src str),
    Nested(NestedComment<'src>),
    /// The lexer won't parse inner datum, even if it is part of the formal grammar.
    /// This choice makes it a lot easier to decouple a lexer from the parser,
    /// and in turn making the tokenization step orders of magnitude simpler.
    /// Instead, only the `#;` is registered. The rest is instead placed in separate token stream
    /// elements.
    ///
    /// EBNF: `#; <Atmosphere>* <Datum>`
    Section,
}

/// EBNF: `#| <NestedCommentText> <NestedComment>* <NestedCommentText> |#`
#[derive(Debug, PartialEq)]
pub struct NestedComment<'src> {
    leading_text: NestedCommentText<'src>,
    nested_comment: Vec<NestedComment<'src>>,
    trailing_text: NestedCommentText<'src>,
}

/// EBNF-ish: `<all characters except CommentOpen and CommentClose>`
/// (may be empty)
#[derive(Debug, PartialEq)]
pub struct NestedCommentText<'src>(&'src str);
