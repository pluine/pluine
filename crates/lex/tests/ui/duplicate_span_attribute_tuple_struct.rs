use pluine_lex::span::Span;

#[derive(pluine_lex_macros::Spanned)]
struct Foo<'src>(#[span] Span<'src>, #[span] Span<'src>);

fn main() {}
