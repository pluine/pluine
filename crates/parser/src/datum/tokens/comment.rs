use crate::*;

pub enum Comment {
    /// EBNF-ish: `<all characters up to a line ending>`
    Semicolon(String),
    Nested(NestedComment),
    Section(SectionComment),
}

mod nested {
    /// EBNF: `#|`
    pub struct CommentOpen;

    /// EBNF: `|#`
    pub struct CommentClose;

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
pub(crate) use nested::{CommentClose, CommentOpen, CommentText, NestedComment};

mod section {
    use crate::*;

    /// EBNF: `<Atmosphere>* <Datum>`
    pub struct SectionComment(Vec<Atmosphere>, Datum);
}
pub(crate) use section::SectionComment;
