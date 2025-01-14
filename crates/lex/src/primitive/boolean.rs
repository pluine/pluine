use crate::*;

/// EBNF: `#t | #T | #true | #TRUE | #f | #F | #false | #FALSE`
// XXX: #tRuE is also a valid representation
#[derive(Debug, PartialEq, Spanned)]
pub struct Boolean {
    inner: bool,
    #[span]
    span: Span,
}
