mod character {
    use crate::*;

    #[derive(Debug, PartialEq)]
    pub enum CharacterLiteral<'src> {
        Simple(CharacterSimple<'src>),
        CodePoint(CharacterCodePoint<'src>),
        Name(CharacterName<'src>),
    }

    impl Spanned for CharacterLiteral<'_> {
        fn span(&self) -> Span<'_> {
            match self {
                CharacterLiteral::Simple(character_simple) => character_simple.span(),
                CharacterLiteral::CodePoint(character_code_point) => character_code_point.span(),
                CharacterLiteral::Name(character_name) => character_name.span(),
            }
        }
    }
}
pub(crate) use character::CharacterLiteral;

mod literal {
    use crate::*;

    /// EBNF-ish: `#\<any char>`
    #[derive(Debug, PartialEq)]
    pub struct CharacterSimple<'src> {
        inner: char,
        span: Span<'src>,
    }

    impl Spanned for CharacterSimple<'_> {
        fn span(&self) -> Span<'_> {
            self.span
        }
    }
}
pub(crate) use literal::CharacterSimple;

mod code_point {
    use crate::*;

    /// Unicode code point character representation.
    ///
    /// EBNF: `#\x <HexadecimalDigit>+ | #\X <HexadecimalDigit>+`
    #[derive(Debug, PartialEq)]
    pub struct CharacterCodePoint<'src> {
        inner: char,
        span: Span<'src>,
    }

    impl Spanned for CharacterCodePoint<'_> {
        fn span(&self) -> Span<'_> {
            self.span
        }
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

    #[derive(Debug, PartialEq)]
    pub struct CharacterName<'src> {
        inner: CharacterNameVariant,
        span: Span<'src>,
    }

    impl Spanned for CharacterName<'_> {
        fn span(&self) -> Span<'_> {
            self.span
        }
    }
}
pub(crate) use name::{CharacterName, CharacterNameVariant};
