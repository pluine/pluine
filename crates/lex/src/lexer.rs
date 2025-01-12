use alloc::vec::Vec;

use crate::*;

pub struct Lexer<'src> {
    scanner: Scanner<'src>,
    token_buffer: Vec<TokenAll<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { scanner: Scanner::new(src), token_buffer: Vec::new() }
    }

    /// A result is returned because tokens are validated to some degree. No
    /// error recovery is applied. Meaning, no tokenization is performed on the remaining source
    /// string once an invalid token is encountered.
    // NOTE: Avoid using recursion here. Tail call optimization can't be guaranteed by the rust
    // compiler, and the `tailcall` crate does not perform well for mutual recursion. Makes it also
    // hard to reason about potential origins of UTF-8 sequence boundary errors.
    pub fn tokenize_all(mut self) -> Vec<TokenAll<'src>> {
        while let Some((start_index, char)) = self.scanner.next() {
            match char {
                // Atmosphere Whitespace
                ' ' | '\t' | '\r' | '\n' => continue,
                ';' => {
                    self.scan_semicolon_comment(start_index);
                }
                '"' => {
                    // TODO:
                    self.scan_string();
                }
                // XXX:
                _ => todo!(),
            }
        }

        self.token_buffer
    }

    fn scan_semicolon_comment(&mut self, start_index: usize) {
        let comment_str = self.scanner.scan_until_line_ending();
        let comment_token = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(comment_str)));
        self.token_buffer.push(comment_token);
    }

    fn scan_string(&mut self) {
        loop {
            match self.scanner.next() {
                Some((char_index, char)) => match char {
                    '"' => {
                        todo!()
                    }
                    '\\' => {
                        match self.scanner.next() {
                            Some((char_nested_index, char_nested)) => {
                                match char_nested {
                                    '"' => todo!(),
                                    '\\' => todo!(),
                                    '|' => todo!(),
                                    // inline hex escape
                                    'x' | 'X' => {
                                        let inline_hex_result = self.scan_inline_hex(char_nested_index);
                                    }
                                    // mnemonic escape
                                    'a' => todo!(),
                                    'b' => todo!(),
                                    't' => todo!(),
                                    'n' => todo!(),
                                    'r' => todo!(),
                                    // intraline whitespace
                                    '\t' | ' ' => todo!(),
                                    _ => {
                                        // TODO: error
                                        todo!();
                                    }
                                }
                                todo!()
                            }
                            // TODO: handle EOF:
                            None => todo!(),
                        }
                    }
                    _ => todo!(),
                },
                // TODO: handle EOF:
                None => todo!(),
            }
        }
    }

    /// `\x` or `\X` have already been scanned
    ///
    /// `start_index` points to the `\` in `\x<HexDigit>+`
    fn scan_inline_hex(&mut self, start_index: usize) -> Result<InlineCodePoint, InlineCodePointScanError> {
        match self.scanner.next() {
            Some((char_index, char)) => {
                if char == InlineCodePoint::TERIMINATOR {
                    let span = self.scanner.span(start_index, char_index + 1);
                    return Err(InlineCodePointScanError::MissingDigit(span));
                }

                match char.to_digit(HexadecimalDigit::RADIX) {
                    Some(hex_value) => {
                        let mut current_code_point = hex_value;
                        loop {
                            match self.scanner.next() {
                                Some((next_char_index, next_char)) => {
                                    if next_char == InlineCodePoint::TERIMINATOR {
                                        let span = self.scanner.span(start_index, next_char_index + 1);
                                        return InlineCodePoint::new(span, current_code_point);
                                    }

                                    match next_char.to_digit(HexadecimalDigit::RADIX) {
                                        Some(next_code_point) => {
                                            let Some(shifted_prev_code_point) = current_code_point.checked_mul(HexadecimalDigit::RADIX)
                                            else {
                                                let span = self.scanner.span(start_index, next_char_index + 1);
                                                return Err(InlineCodePointScanError::OutOfBounds(span));
                                            };

                                            // Within bounds guaranteed by `checked_mul`.
                                            current_code_point = shifted_prev_code_point + next_code_point;
                                        }
                                        None => {
                                            let span = self.scanner.span_char(next_char_index);
                                            return Err(InlineCodePointScanError::InvalidSequenceChar(span));
                                        }
                                    }
                                }
                                None => {
                                    let span = self.scanner.span_to_end_of_file(start_index);
                                    return Err(InlineCodePointScanError::EndOfFile(span));
                                }
                            }
                        }
                    }
                    None => {
                        let span = self.scanner.span_char(char_index);
                        Err(InlineCodePointScanError::InvalidHexDigit(span))
                    }
                }
            }
            None => {
                let span = self.scanner.span_to_end_of_file(start_index);
                Err(InlineCodePointScanError::EndOfFile(span))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_atmosphere_whitespace() {
        let src = " \t\n\r";
        let tokens = Lexer::new(src).tokenize_all();
        assert!(tokens.is_empty())
    }

    #[test]
    fn semicolon_comment() {
        let src = " ;\t\n ";
        let tokens = Lexer::new(src).tokenize_all();

        let comment = &tokens[0];
        let expected = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon("\t")));
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

        #[test]
        fn valid_scan() {
            let src = "\"abc\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::Other("abc")])));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn backslash_escape() {
            let src = "\"\\\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::StringEscape(
                StringEscape::Backslash
            )])));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn vertical_line_escape() {
            let src = "\"|\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::StringEscape(
                StringEscape::VerticalLine
            )])));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn double_quote_escape() {
            let src = "\"\"\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::StringEscape(
                StringEscape::DoubleQuote
            )])));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn mnemonic_escape() {
            let src = "\"\\a\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::MnemonicEscape(
                MnemonicEscape::Alarm
            )])));
            assert_eq!(&expected, comment);

            let src = "\"\\b\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::MnemonicEscape(
                MnemonicEscape::Backspace
            )])));
            assert_eq!(&expected, comment);

            let src = "\"\\n\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::MnemonicEscape(
                MnemonicEscape::Newline
            )])));
            assert_eq!(&expected, comment);

            let src = "\"\\r\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::MnemonicEscape(
                MnemonicEscape::Return
            )])));
            assert_eq!(&expected, comment);

            let src = "\"\\t\"";
            let tokens = Lexer::new(src).tokenize_all();

            let comment = &tokens[0];
            let expected = TokenAll::Token(Token::String(StringLiteral(alloc::vec![StringElement::MnemonicEscape(
                MnemonicEscape::Tab
            )])));
            assert_eq!(&expected, comment);
        }

        #[test]
        fn inline_hex_escape() {
            // both upper and lowercase
            // assert span range and contents
            todo!()
        }

        #[test]
        fn unclosed_eof_error() {
            todo!()
        }

        #[test]
        fn newline_escape_with_line_ending() {
            todo!()
        }

        #[test]
        fn newline_escape_without_line_ending() {
            todo!()
        }

        #[test]
        fn newline_escape_eof_error() {
            todo!()
        }

        #[test]
        fn newline_escaped_newline_error() {
            todo!()
        }

        #[test]
        fn newline_non_newline_error() {
            todo!()
        }

        #[test]
        fn unknown_escape_error() {
            todo!()
        }
    }
}
