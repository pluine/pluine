use pluine_lex::span::Span;

#[derive(pluine_lex_macros::Spanned)]
struct Foo {
    #[span]
    a: Span,
    #[span]
    b: Span,
}

fn main() {}
