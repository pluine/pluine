use thiserror::Error;

use crate::*;

/// Error returned by [`Lexer::tokenize_all`]
// NOTE: thiserror currently not being used because it requires inner errors
// to be `dyn Error + 'static`. No `'src` lifetime allowed that is.
#[derive(Debug, PartialEq, Error)]
pub enum TokenizeError {
    #[error("failed to tokenize string")]
    String(#[from] StringLiteralScanError),
}
