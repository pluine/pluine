use crate::*;

/// Error returned by [`Lexer::tokenize_all`]
// NOTE: thiserror currently not being used because it requires inner errors
// to be `dyn Error + 'static`. No `'src` lifetime allowed that is.
#[derive(Debug)]
pub enum TokenizeError {
    String(TokenizeStringError),
}

#[derive(Debug)]
pub enum TokenizeStringError {
    /// invalid inline code point (inline hex escape)
    InlineHex(InlineCodePointScanError),
}
