//! pluine commons.

pub(crate) use ast::{Atom, Expression, Literal, Value};
mod ast {
    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub struct Expression {
        pub symbol: Box<str>,
        pub list: Box<[Value]>,
    }

    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum Value {
        Atom(Atom),
        Expression(Expression),
    }

    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum Atom {
        Literal(Literal),
        // TODO:
        // VariableReference(()),
    }

    #[cfg_attr(test, derive(Debug, PartialEq))]
    pub enum Literal {
        Integer(i64),
        String(Box<str>),
        // TODO: boolean?
    }
}

mod parser {
    use std::str::FromStr;

    use crate::*;

    impl FromStr for Expression {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // remove padding
            // expect it to start with ( )
            // split by whitespace,
            //
            // parse symbol
            // match parse expression or atom
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod value {
            use super::*;

            #[test]
            fn integer_literals() {
                assert_integer_literals([1, 2], "(+ 1 2)");
                assert_integer_literals([1, 2], "(+ -1 -2)");
                assert_integer_literals([1, 2], "(+ 1 -2)");
                assert_integer_literals([1, 2], "(+ -1 2)");

                fn assert_integer_literals(integers: impl IntoIterator<Item = i64>, expression_str: &str) {
                    assert_expression(integer_expression(integers), expression_str);
                }
            }

            #[test]
            fn string_literals() {
                assert_expression(string_expression(["1", "2"]), r#"(+ "1" "2")"#);
            }

            #[test]
            fn mixed_literals() {
                assert_expression(
                    literal_expression([Literal::String("a".into()), Literal::Integer(1)]),
                    r#"(+ "a" 1)"#,
                );
            }

            #[test]
            fn nested_expressions() {
                assert_expression(
                    nested_expression([integer_expression([1, 2]), integer_expression([3, 4])]),
                    "(+ (+ 1 2) (+ 3 4))",
                );
            }

            #[test]
            fn expression_and_literal() {
                assert_expression(
                    simple_expression([
                        Value::Expression(integer_expression([1, 2])),
                        Value::Atom(Atom::Literal(Literal::Integer(3))),
                    ]),
                    "(+ (+ 1 2) 3)",
                );
            }
        }

        mod padding {
            use super::*;

            #[test]
            fn expression() {
                assert_padding(" (+ 1 2)");
                assert_padding("(+ 1 2) ");
                assert_padding(" (+ 1 2) ");
            }

            #[test]
            fn symbol() {
                assert_padding("(  + 1 2)");
                assert_padding("(+  1 2)");
            }

            #[test]
            fn value() {
                assert_padding("(+ 1  2)");
                assert_padding("(+ 1 2 )");
            }

            fn assert_padding(expression_string: &str) {
                assert_expression(integer_expression([1, 2]), expression_string);
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

        fn string_expression(strs: impl IntoIterator<Item = &'static str>) -> Expression {
            literal_expression(strs.into_iter().map(Into::into).map(Literal::String))
        }

        fn integer_expression(numbers: impl IntoIterator<Item = i64>) -> Expression {
            literal_expression(numbers.into_iter().map(Literal::Integer))
        }
    }
}
