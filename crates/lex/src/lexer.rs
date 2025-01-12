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
        while let Some(char) = self.scanner.next() {
            match char {
                // Atmosphere Whitespace
                ' ' | '\t' | '\r' | '\n' => continue,
                ';' => {
                    self.scan_semicolon_comment();
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

    fn scan_semicolon_comment(&mut self) {
        let comment_str = self.scanner.scan_until_line_ending();
        let comment_token = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(comment_str)));
        self.token_buffer.push(comment_token);
    }

    fn scan_string(&mut self) {
        loop {
            match self.scanner.next() {
                Some(char) => match char {
                    '"' => {
                        todo!()
                    }
                    '\\' => {
                        match self.scanner.next() {
                            Some(char_nested) => {
                                match char_nested {
                                    '"' => todo!(),
                                    '\\' => todo!(),
                                    '|' => todo!(),
                                    // inline hex escape
                                    'x' | 'X' => todo!(),
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
    fn scan_inline_hex(&mut self) -> Result<InlineCodePoint, InlineCodePointScanError> {
        match self.scanner.next() {
            Some(char) => {
                if char == InlineCodePoint::TERIMINATOR {
                    return Err(InlineCodePointScanError::MissingDigit);
                }

                match char.to_digit(HexadecimalDigit::RADIX) {
                    Some(hex_value) => {
                        let mut current_value = hex_value;
                        loop {
                            match self.scanner.next() {
                                Some(next_char) => {
                                    if next_char == InlineCodePoint::TERIMINATOR {
                                        return InlineCodePoint::from_u32(current_value);
                                    }

                                    match next_char.to_digit(HexadecimalDigit::RADIX) {
                                        Some(next_hex) => {
                                            current_value = current_value
                                                .checked_mul(HexadecimalDigit::RADIX)
                                                .ok_or(InlineCodePointScanError::OutOfBounds)?;

                                            // Within bounds guaranteed by `checked_mul`.
                                            current_value += next_hex;
                                        }
                                        None => return Err(InlineCodePointScanError::InvalidSequenceChar),
                                    }
                                }
                                None => return Err(InlineCodePointScanError::EndOfFile),
                            }
                        }
                    }
                    None => Err(InlineCodePointScanError::InvalidHexDigit),
                }
            }
            None => Err(InlineCodePointScanError::EndOfFile),
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
            assert_err(InlineCodePointScanError::OutOfBounds, "FFFFFFFFF");
        }

        #[test]
        fn invalid_hexadecimal_digit_error() {
            // first character
            assert_err(InlineCodePointScanError::InvalidHexDigit, "x");
        }

        #[test]
        fn invalid_sequence_character_error() {
            // subsequent character
            assert_err(InlineCodePointScanError::InvalidSequenceChar, "1x");
        }

        #[test]
        fn invalid_codepoint_error() {
            assert_err(InlineCodePointScanError::InvalidCodePoint, "D800");
        }

        #[test]
        fn at_least_one_digit_error() {
            assert_err(InlineCodePointScanError::MissingDigit, ";");
        }

        #[test]
        fn end_of_file_error() {
            // at first
            let err = Lexer::new("").scan_inline_hex().unwrap_err();
            assert_eq!(InlineCodePointScanError::EndOfFile, err);

            // after first
            let err = Lexer::new("00").scan_inline_hex().unwrap_err();
            assert_eq!(InlineCodePointScanError::EndOfFile, err);
        }

        fn assert_valid(expected_char: char, hex_str: &str) {
            let actual_char = scan_inline_hex(hex_str).expect("invalid inline hex character").inner();
            assert_eq!(expected_char, actual_char);
        }

        fn assert_err(expected_scan_error: InlineCodePointScanError, hex_str: &str) {
            let actual_scan_error = scan_inline_hex(hex_str).expect_err("expected invalid hex string");
            assert_eq!(expected_scan_error, actual_scan_error);
        }

        fn scan_inline_hex(hex_str: &str) -> Result<InlineCodePoint, InlineCodePointScanError> {
            let hex_str = alloc::format!("{};", hex_str);
            Lexer::new(&hex_str).scan_inline_hex()
        }
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
