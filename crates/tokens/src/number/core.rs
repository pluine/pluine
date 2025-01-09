use crate::*;

pub struct Number<R: Radix> {
    prefix: Prefix<R>,
    inner: ComplexNumber<R>,
}
