mod character {
    use crate::*;

    pub enum Character {
        Literal(CharacterLiteral),
        CodePoint(CharacterCodePoint),
        Name(CharacterName),
    }
}

mod literal {
    // TODO: feature flag unicode support
    /// EBNF-ish: `#\<any char>`
    pub struct CharacterLiteral(char);
}
pub(crate) use literal::CharacterLiteral;

mod code_point {
    // TODO: feature flag unicode support
    /// Unicode point code character representation point.
    /// EBNF: `#\x <HexadecimalDigit>+ | #\X <HexadecimalDigit>+`
    pub struct CharacterCodePoint(char);
}
pub(crate) use code_point::CharacterCodePoint;

mod name {
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
}
pub(crate) use name::CharacterName;
