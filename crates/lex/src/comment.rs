use alloc::vec::Vec;

use crate::*;

#[derive(Debug, PartialEq, Spanned)]
pub enum Comment<'src> {
    /// EBNF-ish: `<all characters up to a line ending>`
    ///
    /// String does not include the leading semi-colon, nor the line ending.
    Semicolon(SemicolonComment<'src>),
    Nested(NestedComment<'src>),
    Section(SectionComment),
}

#[derive(Debug, PartialEq, Spanned)]
pub struct SemicolonComment<'src> {
    pub(crate) inner: &'src str,
    #[span]
    pub(crate) span: Span,
}

/// EBNF: `#| <NestedCommentText> <NestedComment>* <NestedCommentText> |#`
#[derive(Debug, PartialEq, Spanned)]
pub struct NestedComment<'src> {
    leading_text: NestedCommentText<'src>,
    nested_comment: Vec<NestedComment<'src>>,
    trailing_text: NestedCommentText<'src>,
    #[span]
    span: Span,
}

/// EBNF-ish: `<all characters except CommentOpen and CommentClose>`
/// (may be empty)
#[derive(Debug, PartialEq)]
pub struct NestedCommentText<'src>(&'src str);

/// EBNF: `#; <Atmosphere>* <Datum>`
///
/// The lexer won't parse inner datum, even if it is part of the formal grammar.
/// This choice makes it a lot easier to decouple a lexer from the parser, and
/// in turn making the tokenization step orders of magnitude simpler. Instead,
/// only the `#;` is registered. The rest is instead placed in separate token
/// stream elements. Span points to only the `#;` comment token.
#[derive(Debug, PartialEq, Spanned)]
pub struct SectionComment(Span);
