/// EBNF-ish: `<Byte> = <any integer between 0 and 255>`
pub struct Byte(u8);

/// EBNF: `#u8(<Byte>)`
pub struct ByteVector(Vec<Byte>);
