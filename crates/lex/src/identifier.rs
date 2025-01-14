mod core {
    use crate::*;

    /// Known in some contexts as "Symbol".
    #[derive(Debug, PartialEq, Spanned)]
    pub enum Identifier<'src> {
        Simple(SimpleIdentifier<'src>),
        Vertical(VerticalIdentifier<'src>),
        Peculiar(PeculiarIdentifier<'src>),
    }
}
pub(crate) use core::Identifier;

mod simple {
    use crate::*;

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

    /// EBNF: `<SimpleInitial> <SimpleSubsequent>*`
    #[derive(Debug, PartialEq, Spanned)]
    pub struct SimpleIdentifier<'src> {
        inner: &'src str,
        #[span]
        span: Span<'src>,
    }
}
pub(crate) use simple::{SimpleIdentifier, SimpleInitial, SimpleSubsequent};

mod vertical {
    use alloc::vec::Vec;

    use crate::*;

    #[derive(Debug, PartialEq, Spanned)]
    pub struct VerticalIdentifier<'src> {
        inner: Vec<SymbolElement<'src>>,
        #[span]
        span: Span<'src>,
    }

    /// EBNF: `<inline hex escape>` | `<mnemonic escape>` | `<any character except '|' or '\'>`
    // TODO: any other character must still be valid ascii, or part a allowed unicode group, `SimpleSubsequent`
    #[derive(Debug, PartialEq)]
    pub enum SymbolElement<'src> {
        MnemonicEscape(MnemonicEscape),
        InlineCodePoint(InlineCodePoint<'src>),
        Str(&'src str),
    }

    pub struct SymbolElementCharacter(char);
}
pub(crate) use vertical::VerticalIdentifier;

mod peculiar {
    use crate::*;

    /// Invalid exceptions: +i and -i and ifnan
    /// EBNF: `<Sign>`
    /// EBNF: `<Sign> <SignSubsequent> <Subsequent>*`
    /// EBNF `[<Sign>] . <DotSubsequent> <Subsequent>*`
    #[derive(Debug, PartialEq, Spanned)]
    pub struct PeculiarIdentifier<'src> {
        /// Span can't be reused trivially for str
        /// as it includes surrounding quotes
        inner: &'src str,
        #[span]
        span: Span<'src>,
    }

    /// EBNF: `<SimpleInitial> | <Sign> | @`
    #[derive(Debug, PartialEq)]
    pub struct SignSubsequent(char);

    /// EBNF: <SignSubsequent> | .
    #[derive(Debug, PartialEq)]
    pub struct DotSubsequent(char);
}
pub(crate) use peculiar::PeculiarIdentifier;
