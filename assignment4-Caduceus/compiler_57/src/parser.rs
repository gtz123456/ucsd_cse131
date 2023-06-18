use std::collections::HashSet;

use sexp::Sexp;

use crate::{
    types::{
        ArithmeticBinaryOp, ArithmeticUnaryOp, CompEqualityOp, CompInequalityOp,
        ComparisonBinaryOp, ComparisonUnaryOp, Expr, Op1, Op2, ParseError, RESERVED_KEYWORDS,
    },
    util::is_valid_identifier,
};

use sexp::Atom::*;

/// Parses the given s-expression into (what is effectively) a Rust enum representation
/// (i.e., an abstract syntax tree).
///
/// # Parameters
/// - `s`: The s-expression.
///
/// # Returns
/// The expression. If the s-expression is invalid, this will instead return a `ParseError`.
pub fn parse_expr(s: &Sexp) -> Result<Expr, ParseError> {
    match s {
        // <number>
        // <identifier>
        // false
        // true
        Sexp::Atom(atom) => match atom {
            S(s) => Ok(if s == "true" || s == "false" {
                Expr::Bool(s == "true")
            } else if s == "input" {
                Expr::Input
            } else {
                Expr::Id(s)
            }),
            I(i) => {
                // Make sure there's no overflow
                if *i > (i64::MAX >> 1) {
                    Err(ParseError::OverflowNumber(atom))
                } else if *i < (i64::MIN >> 1) {
                    Err(ParseError::UnderflowNumber(atom))
                } else {
                    Ok(Expr::Number(*i))
                }
            }
            _ => Err(ParseError::UnsupportedAtomType(atom)),
        },
        Sexp::List(list) => match list.as_slice() {
            // (block <expr>+)
            [Sexp::Atom(S(op)), expressions @ ..] if op == "block" => {
                let mut parsed_statements = vec![];
                for statement in expressions {
                    parsed_statements.push(parse_expr(statement)?);
                }

                if parsed_statements.is_empty() {
                    Err(ParseError::BlockEmpty)
                } else {
                    Ok(Expr::Block(parsed_statements))
                }
            }
            // (set! <identifier> <expr>)
            all @ [Sexp::Atom(S(op)), Sexp::Atom(S(id)), expr] if op == "set!" => {
                if !is_valid_identifier(id) {
                    Err(ParseError::InvalidIdentifier(id, all))
                } else if RESERVED_KEYWORDS.contains(&id.as_str()) {
                    Err(ParseError::ReservedIdentifierUsed(id, all))
                } else {
                    Ok(Expr::Set(id, Box::new(parse_expr(expr)?)))
                }
            }
            // (if <expr> <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Ok(Expr::If(
                Box::new(parse_expr(e1)?),
                Box::new(parse_expr(e2)?),
                Box::new(parse_expr(e3)?),
            )),
            // (let (<bindings> +) <expr>)
            // where each <binding> has the form (String <expr>)
            [Sexp::Atom(S(op)), Sexp::List(bindings), e] if op == "let" => {
                let mut parsed_bindings = vec![];

                // We want to make sure we don't have any duplicate identifiers.
                let mut seen_variables = HashSet::new();
                for b in bindings {
                    let binding_result = parse_bind(b)?;

                    // If we've seen this identifier before, that means we have a duplicate!
                    if seen_variables.contains(binding_result.0) {
                        return Err(ParseError::DuplicateIdentifier(binding_result.0));
                    }

                    // Otherwise, we haven't seen it yet, so we can mark it as seen.
                    seen_variables.insert(binding_result.0);
                    parsed_bindings.push(binding_result);
                }

                if parsed_bindings.is_empty() {
                    return Err(ParseError::NoLetBindings);
                }

                Ok(Expr::Let(parsed_bindings, Box::new(parse_expr(e)?)))
            }
            // (add1 <expr>)
            // (sub1 <expr>)
            all @ [Sexp::Atom(S(op)), e] => match op.as_str() {
                "loop" => Ok(Expr::Loop(Box::new(parse_expr(e)?))),
                "break" => Ok(Expr::Break(Box::new(parse_expr(e)?))),
                "add1" => Ok(Expr::UnOp(
                    Op1::Arith(ArithmeticUnaryOp::Add1),
                    Box::new(parse_expr(e)?),
                )),
                "sub1" => Ok(Expr::UnOp(
                    Op1::Arith(ArithmeticUnaryOp::Sub1),
                    Box::new(parse_expr(e)?),
                )),
                "isnum" => Ok(Expr::UnOp(
                    Op1::Comp(ComparisonUnaryOp::IsNum),
                    Box::new(parse_expr(e)?),
                )),
                "isbool" => Ok(Expr::UnOp(
                    Op1::Comp(ComparisonUnaryOp::IsBool),
                    Box::new(parse_expr(e)?),
                )),
                op => Err(ParseError::UnknownOperator(op, all)),
            },
            // (+  <expr> <expr>)
            // (-  <expr> <expr>)
            // (*  <expr> <expr>)
            // (<  <expr> <expr>)
            // (>  <expr> <expr>)
            // (<= <expr> <expr>)
            // (>= <expr> <expr>)
            // (=  <expr> <expr>)
            all @ [Sexp::Atom(S(op)), e1, e2] => match op.as_str() {
                "+" => Ok(Expr::BinOp(
                    Op2::Arith(ArithmeticBinaryOp::Plus),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "-" => Ok(Expr::BinOp(
                    Op2::Arith(ArithmeticBinaryOp::Minus),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "*" => Ok(Expr::BinOp(
                    Op2::Arith(ArithmeticBinaryOp::Times),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "=" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Equality(CompEqualityOp::Equal)),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "!=" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Equality(CompEqualityOp::NotEqual)),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "<=" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Inequality(CompInequalityOp::LessEqual)),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                "<" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Inequality(CompInequalityOp::Less)),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                ">=" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Inequality(
                        CompInequalityOp::GreaterEqual,
                    )),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                ">" => Ok(Expr::BinOp(
                    Op2::Comp(ComparisonBinaryOp::Inequality(CompInequalityOp::Greater)),
                    Box::new(parse_expr(e1)?),
                    Box::new(parse_expr(e2)?),
                )),
                _ => Err(ParseError::UnknownBinaryOperator(op, all)),
            },
            raw_sexpr => Err(ParseError::InvalidSExpr(raw_sexpr)),
        },
    }
}

/// A helper function to parse a `let`-binding.
///
/// # Parameters
/// - `s`: The s-expression containing a binding.
///
/// # Returns
/// The binding, represented as a tuple where the first item is the identifier
/// and the second item is the associated expression.
///
/// If the binding is invalid, this will instead return a `ParseError`.
fn parse_bind(s: &Sexp) -> Result<(&str, Expr), ParseError> {
    if let Sexp::List(data) = s {
        if let [Sexp::Atom(S(name)), expr] = data.as_slice() {
            // Make sure the name to use as the identifier is valid AND is not
            // a reserved keyword.
            if !is_valid_identifier(name) {
                Err(ParseError::InvalidIdentifier(name, data))
            } else if RESERVED_KEYWORDS.contains(&name.as_str()) {
                Err(ParseError::ReservedIdentifierUsed(name, data))
            } else {
                Ok((name, parse_expr(expr)?))
            }
        } else {
            Err(ParseError::BadBinding(s))
        }
    } else {
        Err(ParseError::InvalidBinding(s))
    }
}

#[cfg(test)]
pub mod parser_tests {
    use crate::types::Expr::*;
    use crate::types::Op1;
    use crate::types::Op2;
    use sexp::parse;

    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(parse_expr(&parse("5").unwrap()), Ok(Number(5)));
    }

    #[test]
    fn test_sub1_add1() {
        assert_eq!(
            parse_expr(&parse("(sub1 (add1 (sub1 5)))").unwrap()),
            Ok(UnOp(
                Op1::Arith(ArithmeticUnaryOp::Sub1),
                Box::new(UnOp(
                    Op1::Arith(ArithmeticUnaryOp::Add1),
                    Box::new(UnOp(
                        Op1::Arith(ArithmeticUnaryOp::Sub1),
                        Box::new(Number(5))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn test_let1() {
        assert_eq!(
            parse_expr(&parse("(let ((x 5)) (add1 x))").unwrap()),
            Ok(Let(
                vec![("x", Number(5))],
                Box::new(UnOp(Op1::Arith(ArithmeticUnaryOp::Add1), Box::new(Id("x"))))
            ))
        );
    }

    #[test]
    fn test_sub1() {
        assert_eq!(
            parse_expr(&parse("(sub1 5)").unwrap()),
            Ok(UnOp(
                Op1::Arith(ArithmeticUnaryOp::Sub1),
                Box::new(Number(5))
            ))
        );
    }

    #[test]
    fn test_let2() {
        assert_eq!(
            parse_expr(&parse("(let ((x 10) (y 7)) (* (- x y) 2))").unwrap()),
            Ok(Let(
                vec![("x", Number(10)), ("y", Number(7))],
                Box::new(BinOp(
                    Op2::Arith(ArithmeticBinaryOp::Times),
                    Box::new(BinOp(
                        Op2::Arith(ArithmeticBinaryOp::Minus),
                        Box::new(Id("x")),
                        Box::new(Id("y"))
                    )),
                    Box::new(Number(2))
                ))
            ))
        );
    }

    #[test]
    fn test_block_1() {
        assert_eq!(
            parse_expr(&parse("(let ((x 5)) (block (set! x (+ x 1))))").unwrap()),
            Ok(Let(
                vec![("x", Number(5))],
                Box::new(Block(vec![Set(
                    "x",
                    Box::new(BinOp(
                        Op2::Arith(ArithmeticBinaryOp::Plus),
                        Box::new(Id("x")),
                        Box::new(Number(1))
                    ))
                )]))
            ))
        );
    }

    #[test]
    fn test_fail_invalid_binop1() {
        assert_eq!(
            parse_expr(&parse("(/ 10 5)").unwrap()),
            Err(ParseError::UnknownBinaryOperator(
                "/",
                &[
                    Sexp::Atom(S("/".to_owned())),
                    Sexp::Atom(I(10)),
                    Sexp::Atom(I(5))
                ]
            ))
        );
    }

    #[test]
    fn test_fail_bad_identifier() {
        assert_eq!(
            parse_expr(&parse("(let ((131cse 10)) (+ 5 131cse))").unwrap()),
            Err(ParseError::InvalidIdentifier(
                "131cse",
                &[Sexp::Atom(S("131cse".to_owned())), Sexp::Atom(I(10))]
            ))
        );
    }

    #[test]
    fn test_fail_reserved_identifier1() {
        assert_eq!(
            parse_expr(&parse("(let ((let 10)) (+ 5 let))").unwrap()),
            Err(ParseError::ReservedIdentifierUsed(
                "let",
                &[Sexp::Atom(S("let".to_owned())), Sexp::Atom(I(10))]
            ))
        );
    }

    #[test]
    fn test_fail_reserved_identifier2() {
        assert_eq!(
            parse_expr(&parse("(let ((x 10) (y 15) (add1 20)) (+ (+ x y) add1))").unwrap()),
            Err(ParseError::ReservedIdentifierUsed(
                "add1",
                &[Sexp::Atom(S("add1".to_owned())), Sexp::Atom(I(20))]
            ))
        );
    }

    #[test]
    fn test_fail_block_empty() {
        assert_eq!(
            parse_expr(&parse("(block )").unwrap()),
            Err(ParseError::BlockEmpty)
        );
    }
}
