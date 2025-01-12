/// Implemented on types containing a [`Span`]
pub trait Spanned {
    fn span(&self) -> Span<'_>;
}

/// Span pointing to a range within the source  storing both start (inclusive)
/// and end (exclusive) indexes in order to later provide diagnostics feedback.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span<'src> {
    start: usize,
    end: usize,
    /// Storing the source string slice over some trick like `PhantomData<&'src
    /// ()>` to later avoid reading the span on another source string.
    src: &'src str,
}

impl<'src> Span<'src> {
    /// Create a span from `start` (inclusive) and `end` (exclusive) indexes.
    ///
    /// Panics in non-optimized builds if start is greater than end, and end not
    /// within the bounds of str.
    pub(crate) fn new(src: &'src str, start: usize, end: usize) -> Self {
        debug_assert!(start < end);
        debug_assert!(end <= src.len());

        Span { start, end, src }
    }
}
