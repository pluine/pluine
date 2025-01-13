use pluine_lex::span::Span;

#[derive(pluine_lex_macros::Spanned)]
struct Foo<'src> {
    #[span]
    a: Span<'src>,
    #[span]
    b: Span<'src>,
}

fn main() {}
