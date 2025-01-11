/// EBNF-ish: `<Byte> = <any exact integer between 0 and 255>`
///
/// TODO: parse inner bytes as Number and make sure that it can be reduced to an u8, checking that
/// it is not inexact  too. for example, 4/2 is a valid byte, but not 4.0/2
pub struct Byte(u8);

/// EBNF: `#u8(<Byte>)`
pub struct ByteVector(Vec<Byte>);
