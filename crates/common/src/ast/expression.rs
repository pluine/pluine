use std::str::FromStr;

use chumsky::prelude::*;
use text::TextParser;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum Value {
    Atom(Atom),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub symbol: Box<str>,
    pub list: Box<[Value]>,
}

// TODO: deprecate?
impl FromStr for Expression {
    type Err = Vec<Simple<char>>;

    fn from_str(source_code_string: &str) -> Result<Self, Self::Err> {
        Expression::parser().parse(source_code_string)
    }
}

impl RuineParser for Expression {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        let expression = recursive(|expression| {
            // XXX: check scheme spec for which symbol values to allow
            let symbol = text::ident().or(just('+').to('+'.to_string())).padded();

            let nested_expression = expression.delimited_by(just('('), just(')')).map(Value::Expression);
            let atom = || Atom::parser().map(Value::Atom);

            let list = nested_expression.or(atom()).padded().repeated().at_least(1);

            symbol
                .then(list)
                .map(|(symbol, list)| Expression { symbol: symbol.into(), list: list.into_boxed_slice() })
        })
        .delimited_by(just('('), just(')'));

        expression.padded().then_ignore(end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod value {
        use super::*;

        #[test]
        fn mixed_literals() {
            assert_expression(
                literal_expression([Literal::String("a".into()), Literal::Integer(Integer::Unsigned(1))]),
                r#"(+ "a" 1)"#,
            );
        }

        #[test]
        fn nested_expressions() {
            assert_expression(
                nested_expression([unsigned_expression([1, 2]), unsigned_expression([3, 4])]),
                "(+ (+ 1 2) (+ 3 4))",
            );
        }

        #[test]
        fn expression_and_literal() {
            assert_expression(
                simple_expression([
                    Value::Expression(unsigned_expression([1, 2])),
                    Value::Atom(Atom::Literal(Literal::Integer(Integer::Unsigned(3)))),
                ]),
                "(+ (+ 1 2) 3)",
            );
        }
    }

    mod whitespace {
        use super::*;

        #[test]
        fn expression() {
            assert_normalized(" (+ 1 2)");
            assert_normalized("  (+ 1 2)");
            assert_normalized("(+ 1 2) ");
            assert_normalized(" (+ 1 2) ");
        }

        #[test]
        fn symbol() {
            assert_normalized("(  + 1 2)");
            assert_normalized("(+  1 2)");
        }

        #[test]
        fn value() {
            assert_normalized("(+ 1  2)");
            assert_normalized("(+ 1 2 )");
        }

        #[test]
        fn newline() {
            assert_normalized("\n(+ 1  2)");
            assert_normalized("(+\n1 2)");
            assert_normalized("(+ 1\n2)");
            assert_normalized("(+ 1 2\n)");
            assert_normalized("(+ 1 2)\n");
        }

        fn assert_normalized(expression_string: &str) {
            assert_expression(unsigned_expression([1, 2]), expression_string);
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn trailing_non_whitespace_character() {
            assert_error("(+ 1 2) a");
        }

        #[test]
        fn missing_list() {
            assert_error("(+)");
        }

        fn assert_error(expression_string: &str) {
            assert!(expression_string.parse::<Expression>().is_err())
        }
    }

    fn assert_expression(expected_expression: Expression, expression_string: &str) {
        let expression = expression_string.parse().unwrap();
        assert_eq!(expected_expression, expression);
    }

    fn simple_expression(values: impl IntoIterator<Item = Value>) -> Expression {
        Expression {
            symbol: "+".into(),
            list: values.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    fn nested_expression(expressions: impl IntoIterator<Item = Expression>) -> Expression {
        simple_expression(expressions.into_iter().map(Value::Expression))
    }

    fn atomic_expression(atoms: impl IntoIterator<Item = Atom>) -> Expression {
        simple_expression(atoms.into_iter().map(Value::Atom))
    }

    fn literal_expression(literals: impl IntoIterator<Item = Literal>) -> Expression {
        atomic_expression(literals.into_iter().map(Atom::Literal))
    }

    fn integer_expression(numbers: impl IntoIterator<Item = Integer>) -> Expression {
        literal_expression(numbers.into_iter().map(Literal::Integer))
    }

    fn unsigned_expression(numbers: impl IntoIterator<Item = u64>) -> Expression {
        integer_expression(numbers.into_iter().map(Integer::Unsigned))
    }
}
