use crate::*;

#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Identifier(Identifier<'src>),
    Boolean(Boolean<'src>),
    Number(NumberLiteral<'src>),
    Character(CharacterLiteral<'src>),
    String(StringLiteral<'src>),
    Other(TokenChar<'src>),
}

#[derive(Debug, PartialEq, Spanned)]
pub struct TokenChar<'src> {
    inner: TokenCharVariant,
    #[span]
    span: Span<'src>,
}

#[derive(Debug, PartialEq)]
pub enum TokenCharVariant {
    /// `(`
    OpenParenthesis,
    /// `)`
    CloseParenthesis,
    /// `#(`
    PoundOpenParenthesis,
    /// `.`
    Dot,
    /// `'`
    Apostophe,
    /// `
    GraveAccent,
    /// `,`
    Comma,
    /// `,@`
    CommaAt,
}

#[derive(Debug, PartialEq)]
pub enum TokenAll<'src> {
    InterToken(Atmosphere<'src>),
    Token(Token<'src>),
}
