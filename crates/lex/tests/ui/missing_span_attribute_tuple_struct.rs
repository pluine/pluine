use pluine_lex::span::Span;

#[derive(pluine_lex_macros::Spanned)]
struct Foo<'src>(Span<'src>, usize);

fn main() {}
