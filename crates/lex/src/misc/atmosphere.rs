use crate::*;

#[derive(Debug, PartialEq)]
pub enum Atmosphere<'src> {
    Comment(Comment<'src>),
    Directive(Directive<'src>),
}

#[derive(Debug, PartialEq)]
pub struct Directive<'src> {
    inner: DirectiveVariant,
    span: Span<'src>,
}

// TODO: case-insensitive directives? spec seems to imply that they should be
#[derive(Debug, PartialEq)]
pub enum DirectiveVariant {
    /// EBNF: `#!fold-case`
    FoldCase,
    /// EBNF: `#!no-fold-case`
    NoFoldCase,
}
