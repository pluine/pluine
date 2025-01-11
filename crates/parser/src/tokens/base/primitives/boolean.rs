// TEMP: until the tokenizer trait exists to implement directly onto boolean
/// EBNF: `#t | #T | #true | #TRUE | #f | #F | #false | #FALSE`
// XXX: #tRuE is also a valid representation
pub struct Boolean(bool);
