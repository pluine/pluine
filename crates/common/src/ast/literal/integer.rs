use chumsky::prelude::*;

use crate::*;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Integer {
    // NOTE: Bounds checks part of semantic analysis
    Signed(u64),
    Unsigned(u64),
    // TODO:
    // Float(f64)
}

impl RuineParser for Integer {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        let number = || text::digits::<char, Simple<char>>(10).map(|str| str.parse::<u64>().unwrap());

        choice((
            just('-').ignore_then(number().map(Integer::Signed)),
            number().map(Integer::Unsigned),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_integer_parsing() {
        assert_unsigned(1, "1");
        assert_unsigned(1, "01");
        assert_unsigned(10, "10");
        assert_unsigned(0, "00");

        fn assert_unsigned(expected_integer: u64, integer_str: &str) {
            assert_eq!(Integer::Unsigned(expected_integer), Integer::parser().parse(integer_str).unwrap());
        }
    }

    #[test]
    fn signed_integer_parsing() {
        assert_signed(1, "-1");
        assert_signed(1, "-01");
        assert_signed(10, "-10");
        assert_signed(0, "-0");
        assert_signed(0, "-00");

        fn assert_signed(expected_integer: i64, integer_str: &str) {
            assert_eq!(
                Integer::Signed(expected_integer as u64),
                Integer::parser().parse(integer_str).unwrap()
            );
        }
    }

    #[test]
    fn minus_character_not_repeatable() {
        assert!(Integer::parser().parse("--1").is_err());
    }
}
