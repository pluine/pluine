use chumsky::prelude::*;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(Integer),
    String(Box<str>),
    // TODO: boolean?
}

impl RuineParser for Literal {
    fn parser() -> impl Parser<char, Self, Error = chumsky::prelude::Simple<char>> {
        // IMPROVEMENT: separate to a `StringLiteral`?
        let escape = just('\\').ignore_then(one_of("\\\""));

        let string_literal = none_of("\\\"")
            .or(escape)
            .repeated()
            .collect()
            .delimited_by(just('"'), just('"'))
            .map(Literal::String);

        Integer::parser().map(Literal::Integer).or(string_literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_literal_parsing() {
        assert_string_literal("a", "\"a\"");
    }

    #[test]
    fn escapes_string_literal() {
        assert_string_literal("\"test\"", r#""\"test\"""#);
        assert_string_literal("\\", r#""\\""#);
        assert_string_literal("\"", r#""\"""#);
    }

    #[test]
    fn escape_character_must_be_known() {
        assert!(Literal::parser().parse("\\a").is_err());
    }

    fn assert_string_literal(expected_str: &str, string_literal: &str) {
        assert_eq!(
            Literal::String(expected_str.into()),
            Literal::parser().parse(string_literal).unwrap()
        );
    }
}
