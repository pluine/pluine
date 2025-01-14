use crate::*;

/// Error returned by [`Lexer::tokenize_all`]
// NOTE: thiserror currently not being used because it requires inner errors
// to be `dyn Error + 'static`. No `'src` lifetime allowed that is.
#[derive(Debug)]
pub enum TokenizeError<'src> {
    String(TokenizeStringError<'src>),
}

#[derive(Debug)]
pub enum TokenizeStringError<'src> {
    /// invalid inline code point (inline hex escape)
    InlineHex(InlineCodePointScanError<'src>),
}
