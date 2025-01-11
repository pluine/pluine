mod core {
    use crate::*;

    pub enum Identifier {
        Simple(SimpleIdentifier),
        Vertical(VerticalIdentifier),
        Peculiar(PeculiarIdentifier),
    }
}

mod simple {
    // TODO: if not(unicode_identifiers):
    // ascii_initial =  |char| .is_ascii_alphabetic() || is_ascii_non_letter
    // TODO: if (unicode_identifiers):
    // char = ascii_initial || (!.is_ascii() && in unicode category)
    //
    // Unicode category: Ll, Lt, Lm, Lo, Nd, Nl, No, Pd, Pc, Po, Sc, Sm, Sk, So, or Co categories or is
    // U+200C or U+200D
    //
    //  ASCII Non letter: `! | $ | % | & | * | / | : | < | = | > | ? | @ | ^ | _ | ~`
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
    // Digit: DecimalDigit
    pub struct SimpleSubsequent(char);

    pub struct SimpleIdentifier(SimpleInitial, Vec<SimpleSubsequent>);
}
pub(crate) use simple::SimpleIdentifier;

mod vertical {
    use crate::*;

    pub struct VerticalIdentifier(String);

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

// TODO:
mod peculiar {
    // Invalid
    // +i and -i and ifnan
    pub struct PeculiarIdentifier(String);

    pub struct SignSubsequent();
}
pub(crate) use peculiar::PeculiarIdentifier;
