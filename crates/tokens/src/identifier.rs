mod core {
    use crate::*;

    pub enum Identifier {
        Simple(SimpleIdentifier),
        Vertical(VerticalIdentifier),
        Peculiar(PeculiarIdentifier),
    }
}

mod peculiar {
    // Invalid
    // +i and -i and ifnan
    pub struct PeculiarIdentifier(String);
}
pub(crate) use peculiar::PeculiarIdentifier;

mod vertical {
    pub struct VerticalIdentifier(String);
}
pub(crate) use vertical::VerticalIdentifier;

mod simple {
    pub struct SimpleIdentifier(String);
}
pub(crate) use simple::SimpleIdentifier;

mod character {
    //! "Scheme implementations may permit any additional repertoire of non-ASCII
    //! Unicode characters ... of Lu, Ll, Lt, Lm, Lo, Mn, Mc, Me, Nd, Nl,
    //! No, Pd, Pc, Po, Sc, Sm, Sk, So, or Co, or is U+200C or U+200D ... However, it is an error
    //! for the first character to have a general category of Nd, Mc, or Me."
    //!
    //! R7RS short spec - 7.1.1

    pub struct IdentifierCharacter(char);

    pub struct FirstIdenfitierCharacter(char);

    impl FirstIdenfitierCharacter {
        /// Consumes a first identifier candidate character, returning `Some(Self)` it is valid.
        /// If no additional feature flags are enabled, then thef
        pub fn from_char(identifier_character: char) -> Option<Self> {
            todo!()
        }
    }
}
