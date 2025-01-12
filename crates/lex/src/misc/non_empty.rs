use alloc::vec::Vec;

/// Used to tokenize <T>+, a list with at least one element.
#[derive(Debug, PartialEq)]
pub struct NonEmptyVec<T>(Vec<T>);
