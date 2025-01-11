use crate::*;

pub enum Token {
    Identifier(Identifier),
    Boolean(Boolean),
    Number(NumberLiteral),
    Character(CharacterLiteral),
    String(StringLiteral),
}

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

pub enum TokenAll<'a> {
    Token(Token),
    InterToken(Atmosphere<'a>),
}
