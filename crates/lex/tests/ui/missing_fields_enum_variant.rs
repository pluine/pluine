#[derive(pluine_lex_macros::Spanned)]
enum Foo<'src> {
    A(),
    B(&'src str),
}

fn main() {}
