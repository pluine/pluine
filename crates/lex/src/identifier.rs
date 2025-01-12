mod core {
    use crate::*;

    /// Known in some contexts as "Symbol".
    #[derive(Debug, PartialEq)]
    pub enum Identifier {
        Simple(SimpleIdentifier),
        Vertical(VerticalIdentifier),
        Peculiar(PeculiarIdentifier),
    }
}
pub(crate) use core::Identifier;

mod simple {
    use alloc::vec::Vec;

    // TODO: if not(unicode_identifiers):
    // ascii_initial =  |char| .is_ascii_alphabetic() || is_ascii_non_letter
    // TODO: if (unicode_identifiers):
    // char = ascii_initial || (!.is_ascii() && in unicode category)
    //
    // Unicode category: Ll, Lt, Lm, Lo, Nd, Nl, No, Pd, Pc, Po, Sc, Sm, Sk, So, or Co categories or is
    // U+200C or U+200D
    //
    //  ASCII Non letter: `! | $ | % | & | * | / | : | < | = | > | ? | @ | ^ | _ | ~`
    #[derive(Debug, PartialEq)]
    pub struct SimpleInitial(char);

    // TODO: if not(unicode_identifiers):
    // ascii_subsequent =  |char| .is_ascii_alphanumberic() || is_ascii_non_letter
    // TODO: if (unicode_identifiers):
    // char = ascii_initial || (!.is_ascii() && in unicode category)
    //
    // Unicode category: Lu, Ll, Lt, Lm, Lo, Mn, Mc, Me, Nd, Nl, No, Pd, Pc, Po, Sc, Sm, Sk, So, or
    // Co, or is U+200C or U+200DG
    //
    // ASCII Non Letter: `! | $ | % | & | * | / | : | < | = | > | ? | @ | ^ | _ | ~ | @ | + | - | .`
    //
    // Digit: 0..9
    #[derive(Debug, PartialEq)]
    pub struct SimpleSubsequent(char);

    #[derive(Debug, PartialEq)]
    pub struct SimpleIdentifier(SimpleInitial, Vec<SimpleSubsequent>);
}
pub(crate) use simple::{SimpleIdentifier, SimpleInitial, SimpleSubsequent};

mod vertical {
    use crate::*;

    // TODO: use &str
    #[derive(Debug, PartialEq)]
    pub struct VerticalIdentifier(alloc::string::String);

    /// EBNF: `<inline hex escape>` | `<mnemonic escape>` | `<any character except '|' or '\'>`
    pub enum SymbolElement {
        MnemonicEscape(MnemonicEscape),
        InlineCodePoint(InlineCodePoint),
        Character(SymbolElementCharacter),
    }

    // TODO: any other character must still be ascii, or part a given unicode group
    pub struct SymbolElementCharacter(char);
}
pub(crate) use vertical::VerticalIdentifier;

mod peculiar {
    use alloc::vec::Vec;

    use crate::*;

    /// Invalid exceptions: +i and -i and ifnan
    #[derive(Debug, PartialEq)]
    pub enum PeculiarIdentifier {
        /// EBNF: `<Sign>`
        SingleSign(Sign),
        /// EBNF: `<Sign> <SignSubsequent> <Subsequent>*`
        SignInitial(Sign, SignSubsequent, Vec<SimpleSubsequent>),
        /// EBNF `[<Sign>] . <DotSubsequent> <Subsequent>*`
        SignDot(Option<Sign>, DotSubsequent, Vec<SimpleSubsequent>),
    }

    /// EBNF: `<SimpleInitial> | <Sign> | @`
    #[derive(Debug, PartialEq)]
    pub struct SignSubsequent(char);

    /// EBNF: <SignSubsequent> | .
    #[derive(Debug, PartialEq)]
    pub struct DotSubsequent(char);
}
pub(crate) use peculiar::PeculiarIdentifier;
