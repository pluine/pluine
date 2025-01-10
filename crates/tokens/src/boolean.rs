// TEMP: until the tokenizer trait exists to implement directly onto boolean
/// EBNF: `#t | #true | #f | #false`
pub struct Boolean(bool);
