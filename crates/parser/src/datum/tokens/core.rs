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

/// Intertoken space can only occur on either side of a token, and not within.
pub enum TokenStream {
    Token(Token),
    InterToken(Atmosphere),
}
