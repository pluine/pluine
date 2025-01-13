use pluine_lex::span::Span;

#[derive(pluine_lex_macros::Spanned)]
union Foo<'src> {
    a: Span<'src>,
}

fn main() {}
