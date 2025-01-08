use chumsky::prelude::*;

pub trait RuineParser: Sized {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>>;
}
