use chumsky::prelude::*;

use crate::*;

// IMPROVEMENT: merge with literal?
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Atom {
    Literal(Literal),
    // TODO:
    // VariableReference(()),
}

impl RuineParser for Atom {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        Literal::parser().map(Atom::Literal)
    }
}
