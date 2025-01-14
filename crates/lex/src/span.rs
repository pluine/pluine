//! Types referencing source code spans.

/// Implemented on types containing a [`Span`]
pub trait Spanned: crate::private::Sealed {
    /// Retrieve the span for a type
    fn span(&self) -> Span<'_>;
}

/// Span pointing to a range within the source storing both start (inclusive)
/// and end (exclusive) indexes in order to later provide diagnostics feedback.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span<'src> {
    start: usize,
    end: usize,
    /// Storing the source string slice over some trick like `PhantomData<&'src
    /// ()>` to later avoid reading the span on another source string.
    src: &'src str,
}

impl Spanned for Span<'_> {
    fn span(&self) -> Span<'_> {
        *self
    }
}

crate::private::impl_sealed_marker!(Span<'_>);

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

/// Primarily end-to-end tests for `pluine_lex_macros`
#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use pluine_lex_macros::Spanned as SpannedMacro;

    use super::Span;
    use crate::Spanned;

    #[test]
    fn error_ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ui/*.rs"));
    }

    #[test]
    fn newtype_struct() {
        let src = "a".to_string();
        let span = Span::new(&src, 0, 1);

        let spanned = NewtypeStruct(span);

        assert_eq!(span, spanned.span())
    }

    #[test]
    fn tuple_struct() {
        let src = "a".to_string();
        let span = Span::new(&src, 0, 1);

        let spanned = TupleStruct(0, span);

        assert_eq!(span, spanned.span())
    }

    #[test]
    fn r#struct() {
        let src = "a".to_string();
        let span = Span::new(&src, 0, 1);

        let spanned = GenericStruct { inner: 0, span };

        assert_eq!(span, spanned.span())
    }

    #[test]
    fn r#enum() {
        let src = "a".to_string();
        let span = Span::new(&src, 0, 1);

        let spanned_a = Enum::A(NewtypeStruct(span));
        assert_eq!(span, spanned_a.span());

        let spanned_b = Enum::B(TupleStruct(0, span));
        assert_eq!(span, spanned_b.span());
    }

    #[derive(SpannedMacro)]
    struct NewtypeStruct<'src>(super::Span<'src>);

    #[derive(SpannedMacro)]
    struct TupleStruct<'src>(#[allow(unused)] usize, #[span] super::Span<'src>);

    #[derive(SpannedMacro)]
    struct GenericStruct<'src, T> {
        #[allow(unused)]
        inner: T,
        #[span]
        span: super::Span<'src>,
    }

    #[derive(SpannedMacro)]
    enum Enum<'src> {
        A(NewtypeStruct<'src>),
        B(TupleStruct<'src>),
    }
}
