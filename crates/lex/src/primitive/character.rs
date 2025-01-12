mod character {
    use crate::*;

    #[derive(Debug, PartialEq)]
    pub enum CharacterLiteral {
        Simple(CharacterSimple),
        CodePoint(CharacterCodePoint),
        Name(CharacterName),
    }
}
pub(crate) use character::CharacterLiteral;

mod literal {
    /// EBNF-ish: `#\<any char>`
    #[derive(Debug, PartialEq)]
    pub struct CharacterSimple(char);
}
pub(crate) use literal::CharacterSimple;

mod code_point {
    /// Unicode code point character representation.
    ///
    /// EBNF: `#\x <HexadecimalDigit>+ | #\X <HexadecimalDigit>+`
    #[derive(Debug, PartialEq)]
    pub struct CharacterCodePoint(char);
}
pub(crate) use code_point::CharacterCodePoint;

mod name {
    #[derive(Debug, PartialEq)]
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
