mod character {
    use crate::*;

    #[derive(Debug, PartialEq, Spanned)]
    pub enum CharacterLiteral {
        Simple(CharacterSimple),
        CodePoint(CharacterCodePoint),
        Name(CharacterName),
    }
}
pub(crate) use character::CharacterLiteral;

mod literal {
    use crate::*;

    /// EBNF-ish: `#\<any char>`
    #[derive(Debug, PartialEq, Spanned)]
    pub struct CharacterSimple {
        inner: char,
        #[span]
        span: Span,
    }
}
pub(crate) use literal::CharacterSimple;

mod code_point {
    use crate::*;

    /// Unicode code point character representation.
    ///
    /// EBNF: `#\x <HexadecimalDigit>+ | #\X <HexadecimalDigit>+`
    #[derive(Debug, PartialEq, Spanned)]
    pub struct CharacterCodePoint {
        inner: char,
        #[span]
        span: Span,
    }
}
pub(crate) use code_point::CharacterCodePoint;

mod name {
    use crate::*;

    #[derive(Debug, PartialEq)]
    pub enum CharacterNameVariant {
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

    #[derive(Debug, PartialEq, Spanned)]
    pub struct CharacterName {
        inner: CharacterNameVariant,
        #[span]
        span: Span,
    }
}
pub(crate) use name::{CharacterName, CharacterNameVariant};
