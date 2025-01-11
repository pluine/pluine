//! Pluine Parser.

mod bytes;
pub(crate) use bytes::{Byte, ByteVector};

mod datum {
    use crate::*;

    pub enum Datum {
        Simple(),
        Compound(),
        // /// EBNF: `# <DecimalDigit>+ = <Datum>`
        // Labeled(NonEmptyVec<Datum>, Box<Datum>),
        // /// EBNF: `# <DecimalDigit>+ #`
        // Reference(NonEmptyVec<DecimalDigit>),
    }

    pub enum SimpleDatum {
        Boolean,
        Number,
        Character,
        String,
        Identifier,
        ByteVector,
    }
}
pub(crate) use datum::Datum;
