use crate::*;

#[derive(Debug, PartialEq)]
pub enum Atmosphere<'src> {
    Comment(Comment<'src>),
    Directive(Directive),
}

// TODO: case-insensitive directives? spec seems to imply that they should be
#[derive(Debug, PartialEq)]
pub enum Directive {
    /// EBNF: `#!fold-case`
    FoldCase,
    /// EBNF: `#!no-fold-case`
    NoFoldCase,
}
