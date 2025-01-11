use crate::*;

pub enum Comment {
    /// EBNF-ish: `<all characters up to a line ending>`
    Semicolon(String),
    Nested(NestedComment),
    /// The lexer won't parse inner datum, even if it is part of the formal grammar.
    /// This choice makes it a lot easier to decouple a lexer from the parser,
    /// and in turn making the tokenization step orders of magnitude simpler.
    /// Instead, only the `#;` is registered. The rest is instead placed in separate token stream
    /// elements.
    ///
    /// EBNF: `#; <Atmosphere>* <Datum>`
    Section,
}

mod nested {
    /// EBNF: `#|`
    pub struct NestedCommentOpen;

    /// EBNF: `|#`
    pub struct NestedCommentClose;

    /// EBNF-ish: `<all characters except CommentOpen and CommentClose>`
    /// (may be empty)
    pub struct CommentText(String);

    /// EBNF: `<CommentOpen> <CommentText> <NestedComment>* <CommentText> <CommentClose>`
    pub struct NestedComment {
        leading_text: CommentText,
        nested_comment: Vec<NestedComment>,
        trailing_text: CommentText,
    }
}
pub(crate) use nested::{CommentText, NestedComment, NestedCommentClose, NestedCommentOpen};
