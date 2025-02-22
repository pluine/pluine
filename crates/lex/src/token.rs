use crate::*;

#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Identifier(Identifier<'src>),
    Boolean(Boolean),
    Number(NumberLiteral),
    Character(CharacterLiteral),
    String(StringLiteral<'src>),
    Other(TokenChar),
}

#[derive(Debug, PartialEq, Spanned)]
pub struct TokenChar {
    inner: TokenCharVariant,
    #[span]
    span: Span,
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
