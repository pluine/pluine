use crate::*;

pub enum Atmosphere {
    Whitespace(Whitespace),
    Comment(Comment),
    Directive(Directive),
}

// TODO: case-insensitive directives? spec seems to imply that they should be
pub enum Directive {
    /// EBNF: `#!fold-case`
    FoldCase,
    /// EBNF: `#!no-fold-case`
    NoFoldCase,
}
