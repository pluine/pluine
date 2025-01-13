use crate::*;

#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Identifier(Identifier<'src>),
    Boolean(Boolean<'src>),
    Number(NumberLiteral<'src>),
    Character(CharacterLiteral),
    String(StringLiteral<'src>),
    Other(TokenChar),
}

#[derive(Debug, PartialEq)]
pub enum TokenChar {
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
