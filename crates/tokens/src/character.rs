pub enum Character {
    Literal(CharacterLiteral),
    CodePoint(CharacterCodePoint),
    Name(CharacterName),
}

// TODO: feature flag unicode support
/// EBNF-ish: `#\<any char>`
pub struct CharacterLiteral(char);

// TODO: feature flag unicode support
/// Unicode point code character representation point.
/// EBNF: `#\x <HexadecimalDigit>+ | #\X <HexadecimalDigit>+`
pub struct CharacterCodePoint(char);

pub enum CharacterName {
    /// EPNF: `#\alarm`
    Alarm,
    /// EPNF: `#\backspace`
    Backspace,
    /// EPNF: `#\delete`
    Delete,
    /// EPNF: `#\escape`
    Escape,
    /// EPNF: `#\newline`
    Newline,
    /// EPNF: `#\null`
    Null,
    /// EPNF: `#\return`
    Return,
    /// EPNF: `#\space`
    Space,
    /// EPNF: `#\tab`
    Tab,
}
