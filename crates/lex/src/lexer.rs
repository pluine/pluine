use alloc::vec::Vec;

use crate::*;

/// Entrypoint for using `pluine_lex`.
/// ```
/// # use pluine_lex::Lexer;
/// let src = "\"abc\"";
/// let tokens = Lexer::new(src)
///     .tokenize_all()
///     .expect("valid source code string");
/// ```
pub struct Lexer<'src> {
    scanner: Scanner<'src>,
    token_buffer: Vec<TokenAll<'src>>,
}

impl<'src> Lexer<'src> {
    /// Construct a new `Lexer`.
    pub fn new(src: &'src str) -> Self {
        Self { scanner: Scanner::new(src), token_buffer: Vec::new() }
    }

    /// A result is returned because tokens are validated to some degree. No
    /// error recovery is applied. Meaning, no tokenization is performed on the remaining source
    /// string once an invalid token is encountered.
    //
    // NOTE: Avoid using recursion here. Tail call optimization can't be guaranteed by the rust
    // compiler, and the `tailcall` crate does not perform well for mutual recursion. Makes it also
    // hard to reason about potential origins of UTF-8 sequence boundary errors.
    pub fn tokenize_all(mut self) -> Result<Vec<TokenAll<'src>>, TokenizeError> {
        while let Some((start_index, char)) = self.scanner.next() {
            match char {
                // Atmosphere Whitespace
                ' ' | '\t' | '\r' | '\n' => continue,
                ';' => {
                    self.scan_semicolon_comment(start_index);
                }
                '"' => {
                    self.scan_string(start_index)?;
                }
                // XXX:
                _ => todo!(),
            }
        }

        Ok(self.token_buffer)
    }

    /// `;` scanned
    fn scan_semicolon_comment(&mut self, start_index: usize) {
        let (end_index, comment_str) = self.scanner.scan_until_line_ending();

        let span = self.scanner.span(start_index, end_index);

        let semicolon_comment = SemicolonComment { inner: comment_str, span };
        let comment_token = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(semicolon_comment)));

        self.token_buffer.push(comment_token);
    }

    /// `"` scanned
    fn scan_string(&mut self, start_index: usize) -> Result<(), StringLiteralScanError> {
        let mut string_elements = StringElementCollector::new(self.scanner.src());

        loop {
            let Some((char_index, char)) = self.scanner.next() else {
                let eof_span = self.scanner.span_to_end_of_file(start_index);
                return Err(StringLiteralScanError::EndOfFile(eof_span));
            };

            match char {
                '"' => {
                    let string_elements = string_elements.finalize(char_index);

                    let token = TokenAll::Token(Token::String(StringLiteral {
                        inner: string_elements,
                        // + 1 to include `"` in span
                        span: self.scanner.span(start_index, char_index + 1),
                    }));

                    self.token_buffer.push(token);

                    return Ok(());
                }
                '\\' => {
                    let Some((char_nested_index, char_nested)) = self.scanner.next() else {
                        let eof_span = self.scanner.span_to_end_of_file(start_index);
                        return Err(StringLiteralScanError::EndOfFile(eof_span));
                    };

                    match char_nested {
                        '"' => string_elements.push_string_escape(char_index, StringEscape::DoubleQuote),
                        '\\' => string_elements.push_string_escape(char_index, StringEscape::Backslash),
                        '|' => string_elements.push_string_escape(char_index, StringEscape::VerticalLine),
                        'x' | 'X' => {
                            let inline_code_point = self.scan_inline_hex(char_index)?;
                            string_elements.push_inline_code_point(char_index, inline_code_point);
                        }
                        'a' => string_elements.push_mnemonic_escape(char_index, MnemonicEscape::Alarm),
                        'b' => string_elements.push_mnemonic_escape(char_index, MnemonicEscape::Backspace),
                        't' => string_elements.push_mnemonic_escape(char_index, MnemonicEscape::Tab),
                        'n' => string_elements.push_mnemonic_escape(char_index, MnemonicEscape::Newline),
                        'r' => string_elements.push_mnemonic_escape(char_index, MnemonicEscape::Return),
                        '\r' => {
                            // \r\n case handled by `maybe_update_line_ending` call in next loop iteration
                            string_elements.push_newline_escape(char_index, LineEnding::Return, Vec::new());
                        }
                        '\n' => {
                            string_elements.push_newline_escape(char_index, LineEnding::Newline, Vec::new());
                        }
                        ' ' => {
                            self.scan_string_newline_escape(start_index, char_index, &mut string_elements, IntralineWhitespace::Space)?
                        }
                        '\t' => self.scan_string_newline_escape(start_index, char_index, &mut string_elements, IntralineWhitespace::Tab)?,
                        _ => {
                            // + 1 to include the unknown escape character
                            let span = self.scanner.span(char_index, char_nested_index + 1);
                            return Err(StringLiteralScanError::UnknownEscape(span));
                        }
                    }
                }
                '\n' => {
                    string_elements.maybe_update_line_ending(char_index);
                }
                _ => {
                    string_elements.maybe_begin_chars(char_index);
                }
            }
        }
    }

    fn scan_string_newline_escape(
        &mut self,
        string_start_index: usize,
        backslash_index: usize,
        string_elements: &mut StringElementCollector,
        first_whitespace_char: IntralineWhitespace,
    ) -> Result<(), StringLiteralScanError> {
        let mut leading_whitespace = alloc::vec![first_whitespace_char];

        loop {
            let Some((newline_escape_char_index, newline_escape_char)) = self.scanner.next() else {
                let eof_span = self.scanner.span_to_end_of_file(string_start_index);
                return Err(StringLiteralScanError::EndOfFile(eof_span));
            };

            match newline_escape_char {
                ' ' => leading_whitespace.push(IntralineWhitespace::Space),
                '\t' => leading_whitespace.push(IntralineWhitespace::Tab),
                '\r' => {
                    string_elements.push_newline_escape(backslash_index, LineEnding::Return, leading_whitespace);
                    break;
                }
                '\n' => {
                    string_elements.push_newline_escape(backslash_index, LineEnding::Newline, leading_whitespace);
                    break;
                }
                _ => {
                    let span = self.scanner.span_char(newline_escape_char_index);
                    return Err(StringLiteralScanError::UnknownWhitespace(span));
                }
            }
        }

        Ok(())
    }

    /// `\x` or `\X` have already been scanned
    ///
    /// `start_index` points to the `\` in `\x<HexDigit>+`
    fn scan_inline_hex(&mut self, start_index: usize) -> Result<InlineCodePoint, InlineCodePointScanError> {
        let Some((char_index, char)) = self.scanner.next() else {
            let span = self.scanner.span_to_end_of_file(start_index);
            return Err(InlineCodePointScanError::EndOfFile(span));
        };

        if char == InlineCodePoint::TERIMINATOR {
            let span = self.scanner.span(start_index, char_index + 1);
            return Err(InlineCodePointScanError::MissingDigit(span));
        }

        let Some(hex_value) = char.to_digit(HexadecimalDigit::RADIX) else {
            let span = self.scanner.span_char(char_index);
            return Err(InlineCodePointScanError::InvalidHexDigit(span));
        };

        let mut current_code_point = hex_value;

        loop {
            let Some((next_char_index, next_char)) = self.scanner.next() else {
                let span = self.scanner.span_to_end_of_file(start_index);
                return Err(InlineCodePointScanError::EndOfFile(span));
            };

            if next_char == InlineCodePoint::TERIMINATOR {
                let span = self.scanner.span(start_index, next_char_index + 1);
                return InlineCodePoint::new(span, current_code_point);
            }

            let Some(next_code_point) = next_char.to_digit(HexadecimalDigit::RADIX) else {
                let span = self.scanner.span_char(next_char_index);
                return Err(InlineCodePointScanError::InvalidSequenceChar(span));
            };

            let Some(shifted_prev_code_point) = current_code_point.checked_mul(HexadecimalDigit::RADIX) else {
                let span = self.scanner.span(start_index, next_char_index + 1);
                return Err(InlineCodePointScanError::OutOfBounds(span));
            };

            // Within bounds guaranteed by `checked_mul`.
            current_code_point = shifted_prev_code_point + next_code_point;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_atmosphere_whitespace() {
        let src = " \t\n\r";
        let tokens = Lexer::new(src).tokenize_all().unwrap();
        assert!(tokens.is_empty())
    }

    #[test]
    fn semicolon_comment() {
        let src = " ;\t\n ";
        let tokens = Lexer::new(src).tokenize_all().unwrap();

        let comment = &tokens[0];
        let expected = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(SemicolonComment {
            inner: "\t",
            span: Span::new(src, 1, 3),
        })));

        assert_eq!(&expected, comment);
    }

    mod inline_hex {
        use super::*;

        #[test]
        fn ascii() {
            assert_valid('💯', "1F4AF");
        }

        #[test]
        fn emoji() {
            assert_valid('a', "61");
        }

        #[test]
        fn leading_zeroes() {
            assert_valid('a', "0061");
        }

        #[test]
        fn upper_and_lower_case() {
            // (tests both first and subsequent chars)
            assert_valid('אּ', "fb30");
            assert_valid('אּ', "FB30");
        }

        #[test]
        fn out_of_bounds_error() {
            // Error span up until the character creating the out of bounds
            assert_err!(InlineCodePointScanError::OutOfBounds, 0, 11, "FFFFFFFFF");
        }

        #[test]
        fn invalid_hexadecimal_digit_error() {
            // first character
            assert_err!(InlineCodePointScanError::InvalidHexDigit, 2, 3, "x");
        }

        #[test]
        fn invalid_sequence_character_error() {
            // subsequent character
            assert_err!(InlineCodePointScanError::InvalidSequenceChar, 3, 4, "1x");
        }

        #[test]
        fn invalid_codepoint_error() {
            assert_err!(InlineCodePointScanError::InvalidCodePoint, "D800");
        }

        #[test]
        fn at_least_one_digit_error() {
            // ; added in assert_err
            assert_err!(InlineCodePointScanError::MissingDigit, "");
        }

        #[test]
        fn end_of_file_error() {
            // at first
            assert_eof("");
            // after first
            assert_eof("00");

            fn assert_eof(src: &str) {
                // omit semicolon
                let src = alloc::format!("\\x{}", src);

                let mut lexer = Lexer::new(&src);
                lexer.scanner.next();
                lexer.scanner.next();

                let start = 0;

                let err = lexer.scan_inline_hex(start).unwrap_err();

                let expected_span = Span::new(&src, start, src.len());
                let expected_error = InlineCodePointScanError::EndOfFile(expected_span);
                assert_eq!(expected_error, err);
            }
        }

        fn assert_valid(expected_char: char, hex_str: &str) {
            let hex_str = alloc::format!("\\x{};", hex_str);
            let mut lexer = Lexer::new(&hex_str);
            lexer.scanner.next();
            lexer.scanner.next();

            let actual_char = lexer.scan_inline_hex(0).expect("invalid inline hex character").inner();
            assert_eq!(expected_char, actual_char);
        }

        macro_rules! assert_err {
            ($err_type:tt::$err_variant:tt, $hex_str:literal) => {
                // Add 3 because of the inserted "\x" and ";"
                assert_err!($err_type::$err_variant, 0, $hex_str.len() + 3, $hex_str)
            };
            ($err_type:tt::$err_variant:tt, $start:literal, $end:expr, $hex_str:literal) => {
                let hex_str = alloc::format!("\\x{};", $hex_str);

                let expected_error = $err_type::$err_variant($crate::Span::new(&hex_str, $start, $end));

                let mut lexer = Lexer::new(&hex_str);
                lexer.scanner.next();
                lexer.scanner.next();

                let actual_scan_error = lexer.scan_inline_hex(0).expect_err("expected invalid hex string");

                assert_eq!(expected_error, actual_scan_error);
            };
        }
        use assert_err;
    }

    mod string {
        use super::*;

        fn expected_string_token<'src>(src: &'src str, start: usize, end: usize, element: StringElement<'src>) -> TokenAll<'src> {
            expected_string_tokens(src, start, end, [element])
        }

        fn expected_string_tokens<'src>(
            src: &'src str,
            start: usize,
            end: usize,
            elements: impl IntoIterator<Item = StringElement<'src>>,
        ) -> TokenAll<'src> {
            TokenAll::Token(Token::String(StringLiteral {
                inner: elements.into_iter().collect(),
                span: Span::new(src, start, end),
            }))
        }

        #[test]
        fn valid_scan() {
            let src = "\"abc\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 5, StringElement::Chars("abc"));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn mixed_string_elements() {
            let src = "\"abc\\x64;e\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let string_elements = [
                StringElement::Chars("abc"),
                StringElement::InlineCodePoint(InlineCodePoint('d', Span::new(src, 4, 9))),
                StringElement::Chars("e"),
            ];
            let expected = expected_string_tokens(src, 0, 11, string_elements);
            assert_eq!(&expected, comment);
        }

        #[test]
        fn backslash_escape() {
            let src = r#""\\""#;
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::StringEscape(StringEscape::Backslash));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn vertical_line_escape() {
            let src = r#""\|""#;
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::StringEscape(StringEscape::VerticalLine));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn double_quote_escape() {
            let src = r#""\"""#;
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::StringEscape(StringEscape::DoubleQuote));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn mnemonic_escape() {
            let src = "\"\\a\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::MnemonicEscape(MnemonicEscape::Alarm));
            assert_eq!(&expected, comment);

            let src = "\"\\b\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::MnemonicEscape(MnemonicEscape::Backspace));
            assert_eq!(&expected, comment);

            let src = "\"\\n\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::MnemonicEscape(MnemonicEscape::Newline));
            assert_eq!(&expected, comment);

            let src = "\"\\r\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::MnemonicEscape(MnemonicEscape::Return));
            assert_eq!(&expected, comment);

            let src = "\"\\t\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 4, StringElement::MnemonicEscape(MnemonicEscape::Tab));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn inline_hex_escape() {
            // lower x
            let src = "\"\\x61;\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let inlined_code_point = InlineCodePoint('a', Span::new(src, 1, 6));
            let expected = expected_string_token(src, 0, 7, StringElement::InlineCodePoint(inlined_code_point));
            assert_eq!(&expected, comment);

            // upper X
            let src = "\"\\X61;\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let inlined_code_point = InlineCodePoint('a', Span::new(src, 1, 6));
            let expected = expected_string_token(src, 0, 7, StringElement::InlineCodePoint(inlined_code_point));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn unclosed_eof_error() {
            // at first char
            let src = r#"""#;
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 0, 1);
            let expected_error = TokenizeError::String(StringLiteralScanError::EndOfFile(expected_span));
            assert_eq!(expected_error, actual_error);

            // at subsequent chars
            let src = r#""abc"#;
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 0, 4);
            let expected_error = TokenizeError::String(StringLiteralScanError::EndOfFile(expected_span));
            assert_eq!(expected_error, actual_error);
        }

        #[test]
        fn newline_escape() {
            // keeps leading and trailing whitespace

            // supports both \t and ' ' as first and subsequent chars
            let src = "\"abc  \\ \t\n  def\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();
            let comment = &tokens[0];
            let expected = expected_string_tokens(
                src,
                0,
                16,
                [
                    StringElement::Chars("abc  "),
                    StringElement::NewlineEscape(StringNewlineEscape {
                        line_ending: LineEnding::Newline,
                        leading_whitespace: alloc::vec![IntralineWhitespace::Space, IntralineWhitespace::Tab],
                    }),
                    StringElement::Chars("  def"),
                ],
            );
            assert_eq!(&expected, comment);

            // supports both \t and ' ' as first and subsequent chars
            let src = "\"abc  \\ \t\n  def\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();
            let comment = &tokens[0];
            let expected = expected_string_tokens(
                src,
                0,
                16,
                [
                    StringElement::Chars("abc  "),
                    StringElement::NewlineEscape(StringNewlineEscape {
                        line_ending: LineEnding::Newline,
                        leading_whitespace: alloc::vec![IntralineWhitespace::Tab, IntralineWhitespace::Space],
                    }),
                    StringElement::Chars("  def"),
                ],
            );
            assert_eq!(&expected, comment);

            // without leading whitespace
            // TODO: test \r and \r\n return too
            let src = "\"a\\\nb\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();
            let comment = &tokens[0];
            let expected = expected_string_tokens(
                src,
                0,
                6,
                [
                    StringElement::Chars("a"),
                    StringElement::NewlineEscape(StringNewlineEscape {
                        // TODO: test \r and \r\n return too
                        line_ending: LineEnding::Newline,
                        leading_whitespace: Default::default(),
                    }),
                    StringElement::Chars("b"),
                ],
            );
            assert_eq!(&expected, comment);
        }

        // Make sure that the newline isn't simply checked for a possible
        // CRLF of a newline escape and then discarded if not.
        #[test]
        fn newline_as_first_char() {
            let src = "\"\n\"";
            let tokens = Lexer::new(src).tokenize_all().unwrap();

            let comment = &tokens[0];
            let expected = expected_string_token(src, 0, 3, StringElement::Chars("\n"));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn newline_escape_eof_error() {
            let src = r#""\ "#;
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 0, 3);
            let expected_error = TokenizeError::String(StringLiteralScanError::EndOfFile(expected_span));
            assert_eq!(expected_error, actual_error);
        }

        #[test]
        fn newline_escape_unknown_whitespace_error() {
            let src = r#""\ a""#;
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 3, 4);
            let expected_error = TokenizeError::String(StringLiteralScanError::UnknownWhitespace(expected_span));
            assert_eq!(expected_error, actual_error);
        }

        #[test]
        fn newline_escape_without_line_ending_error() {
            // prematurely closing is an error when following the specs grammar
            // (newline isn't optional)
            let src = "\"\\ \"";
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 3, 4);
            let expected_error = TokenizeError::String(StringLiteralScanError::UnknownWhitespace(expected_span));
            assert_eq!(expected_error, actual_error);
        }

        #[test]
        fn unknown_escape_error() {
            let src = r#""\y""#;
            let actual_error = Lexer::new(src).tokenize_all().unwrap_err();

            let expected_span = Span::new(src, 1, 3);
            let expected_error = TokenizeError::String(StringLiteralScanError::UnknownEscape(expected_span));
            assert_eq!(expected_error, actual_error);
        }
    }
}
