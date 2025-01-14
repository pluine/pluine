use crate::*;

#[derive(Debug, PartialEq)]
pub enum Atmosphere<'src> {
    Comment(Comment<'src>),
    Directive(Directive),
}

#[derive(Debug, PartialEq)]
pub struct Directive {
    inner: DirectiveVariant,
    span: Span,
}

// TODO: case-insensitive directives? spec seems to imply that they should be
#[derive(Debug, PartialEq)]
pub enum DirectiveVariant {
    /// EBNF: `#!fold-case`
    FoldCase,
    /// EBNF: `#!no-fold-case`
    NoFoldCase,
}
