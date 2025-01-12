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

    pub fn tokenize_all(mut self) -> Vec<TokenAll<'src>> {
        let mut scanner = self.scanner;

        while let Some((char_index, char)) = scanner.next() {
            match char {
                // Atmosphere Whitespace
                ' ' | '\t' | '\r' | '\n' => continue,
                ';' => {
                    let comment_str = scanner.scan_until_line_ending();
                    let comment_token = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(comment_str)));
                    self.token_buffer.push(comment_token);
                }
                // TODO: do not call call this match recursively, might lead to str on utf8 sequenc boundary errors
                _ => todo!(),
            }
        }

        self.token_buffer
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
        let src = " ; ";
        let tokens = Lexer::new(src).tokenize_all();

        let comment = &tokens[0];
        let expected = TokenAll::InterToken(Atmosphere::Comment(Comment::Semicolon(" ")));
        assert_eq!(&expected, comment);
    }
}
