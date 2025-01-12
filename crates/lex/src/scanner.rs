use ::core::str::CharIndices;

use crate::*;

pub struct Scanner<'src> {
    src: &'src str,
    char_iter: CharIndices<'src>,
}

impl Iterator for Scanner<'_> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.char_iter.next()
    }
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src, char_iter: src.char_indices() }
    }

    /// Shorthand for calling [`Self::next`] and omitting the returned index
    pub fn next_char(&mut self) -> Option<char> {
        self.next().map(|(_, char)| char)
    }

    /// See [`Span::new`]
    pub fn span(&self, start: usize, end: usize) -> Span<'_> {
        Span::new(self.src, start, end)
    }

    /// Span from `start` to `source.len()`
    pub fn span_to_end_of_file(&self, start: usize) -> Span<'_> {
        Span::new(self.src, start, self.src.len())
    }

    /// Span from `start` to `start + 1`
    pub fn span_char(&self, start: usize) -> Span<'_> {
        Span::new(self.src, start, start + 1)
    }

    /// Forwards the internal iterator until it reaches a [`LineEnding`]
    ///
    /// Returned string does not include the line ending.
    /// Returns an empty string if internal iterator is at EOF.
    ///
    /// # Panics
    ///
    /// If inner iterator is not on an UTF-8 sequence boundary. This should
    /// never happen if after matching `.next()` on an ASCII character.
    pub fn scan_until_line_ending(&mut self) -> &'src str {
        match self.next() {
            Some((start, start_char)) => {
                if LineEnding::is_line_ending(start_char) {
                    return "";
                }

                loop {
                    match self.next() {
                        Some((current_index, current_char)) => {
                            if LineEnding::is_line_ending(current_char) {
                                return &self.src[start..current_index];
                            }
                        }
                        // Reached EOF
                        None => return &self.src[start..self.char_iter.offset()],
                    }
                }
            }
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scan_until_line_ending {
        use super::*;

        #[test]
        fn finds_return() {
            let mut scanner = Scanner::new("abc\n123");

            let scanned = scanner.scan_until_line_ending();
            assert_eq!("abc", scanned);
        }

        #[test]
        fn finds_line_feed() {
            let mut scanner = Scanner::new("abc\r123");

            let scanned = scanner.scan_until_line_ending();
            assert_eq!("abc", scanned);
        }

        #[test]
        fn empty_string_for_eof() {
            let mut scanner = Scanner::new("a");
            scanner.next();

            let scanned = scanner.scan_until_line_ending();
            assert_eq!("", scanned);
        }

        #[test]
        fn no_newline_returns_remaining() {
            let mut scanner = Scanner::new("123");

            let scanned = scanner.scan_until_line_ending();
            assert_eq!("123", scanned);
        }
    }
}
