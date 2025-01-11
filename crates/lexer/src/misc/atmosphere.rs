use crate::*;

pub enum Atmosphere<'src> {
    Whitespace(Whitespace),
    Comment(Comment<'src>),
    Directive(Directive),
}

// TODO: case-insensitive directives? spec seems to imply that they should be
pub enum Directive {
    /// EBNF: `#!fold-case`
    FoldCase,
    /// EBNF: `#!no-fold-case`
    NoFoldCase,
}
