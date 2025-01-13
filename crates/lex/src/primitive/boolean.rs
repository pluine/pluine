use crate::*;

/// EBNF: `#t | #T | #true | #TRUE | #f | #F | #false | #FALSE`
// XXX: #tRuE is also a valid representation
#[derive(Debug, PartialEq)]
pub struct Boolean<'src>(bool, Span<'src>);

impl Spanned for Boolean<'_> {
    fn span(&self) -> Span<'_> {
        self.1
    }
}
