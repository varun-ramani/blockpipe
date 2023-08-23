use core::panic;

use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::sequence::tuple;
use nom::IResult;

use crate::language::parse::ast::Expression;
use crate::language::parse::ast::PipeType;

use super::bind::parse_bind;
use super::binding::parse_binding;
use super::block::parse_block;
use super::literal::parse_literal;
use super::tuple::parse_tuple;
use super::type_bind::parse_type_bind;
use super::{ignore_ws, ParseRoot};

pub fn parse_pipe(input: &str) -> IResult<&str, Expression> {
    let (input, (expr1, pipe_char, expr2)) = ignore_ws(tuple((
        alt((
            parse_literal,
            parse_block,
            parse_tuple,
            parse_type_bind,
            parse_bind,
            parse_binding,
        )),
        alt((tag("|*"), tag("|"))),
        Expression::parse,
    )))(input)?;

    let pipe_type = match pipe_char {
        "|" => PipeType::Flow,
        "|*" => PipeType::Destructure,
        _ => panic!(
            "Parsed pipe, but delimited is {} instead of | or |*",
            pipe_char
        ),
    };

    Ok((
        input,
        Expression::Pipe(Box::from(expr1), pipe_type, Box::from(expr2)),
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::parse::{
        ast::{Expression, Identifier, LiteralType, PipeType},
        parser::parse_from_string,
    };

    use super::parse_pipe;

    #[test]
    fn basic_flow_pipe() {
        let (_, expr) = parse_pipe(indoc! {r"
            10 | {}
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Literal(LiteralType::Int(10))),
                PipeType::Flow,
                Box::new(Expression::Block(vec![]))
            )
        )
    }

    #[test]
    fn basic_destructure_pipe() {
        let (_, expr) = parse_pipe(indoc! {r"
            10 |* {}
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Literal(LiteralType::Int(10))),
                PipeType::Destructure,
                Box::new(Expression::Block(vec![]))
            )
        )
    }

    #[test]
    fn flow_pipeline() {
        let expr = parse_from_string::<Expression>(indoc! {r"
            1 | {2} | {3} | {4}
        "})
        .expect("Failed to parse: ");

        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Literal(LiteralType::Int(1))),
                PipeType::Flow,
                Box::new(Expression::Pipe(
                    Box::new(Expression::Block(vec![Expression::Literal(
                        LiteralType::Int(2)
                    )])),
                    PipeType::Flow,
                    Box::new(Expression::Pipe(
                        Box::new(Expression::Block(vec![Expression::Literal(
                            LiteralType::Int(3)
                        )])),
                        PipeType::Flow,
                        Box::new(Expression::Block(vec![Expression::Literal(
                            LiteralType::Int(4)
                        )]))
                    ))
                ))
            )
        )
    }

    #[test]
    fn destructure_pipeline() {
        let expr = parse_from_string::<Expression>(indoc! {r"
            1 |* {2} |* {3} |* {4}
        "})
        .expect("Failed to parse: ");

        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Literal(LiteralType::Int(1))),
                PipeType::Destructure,
                Box::new(Expression::Pipe(
                    Box::new(Expression::Block(vec![Expression::Literal(
                        LiteralType::Int(2)
                    )])),
                    PipeType::Destructure,
                    Box::new(Expression::Pipe(
                        Box::new(Expression::Block(vec![Expression::Literal(
                            LiteralType::Int(3)
                        )])),
                        PipeType::Destructure,
                        Box::new(Expression::Block(vec![Expression::Literal(
                            LiteralType::Int(4)
                        )]))
                    ))
                ))
            )
        )
    }

    #[test]
    fn heterogenous_pipeline() {
        let (_, expr) = parse_pipe(indoc! {r"
            5 |* {} | {} | {(10 20) |* {}} 
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Literal(LiteralType::Int(5))),
                PipeType::Destructure,
                Box::new(Expression::Pipe(
                    Box::new(Expression::Block(vec![])),
                    PipeType::Flow,
                    Box::new(Expression::Pipe(
                        Box::new(Expression::Block(vec![])),
                        PipeType::Flow,
                        Box::new(Expression::Block(vec![Expression::Pipe(
                            Box::new(Expression::Tuple(vec![
                                Expression::Literal(LiteralType::Int(10)),
                                Expression::Literal(LiteralType::Int(20))
                            ])),
                            PipeType::Destructure,
                            Box::new(Expression::Block(vec![]))
                        )]))
                    ))
                ))
            )
        )
    }

    #[test]
    fn pipeline_with_bindings() {
        let (_, expr) = parse_pipe(indoc! {r"
            (1 2) |* a_bc | e_FG | hij
        "})
        .expect("Failed to parse: ");

        assert_eq!(
            expr,
            Expression::Pipe(
                Box::new(Expression::Tuple(vec![
                    Expression::Literal(LiteralType::Int(1)),
                    Expression::Literal(LiteralType::Int(2)),
                ])),
                PipeType::Destructure,
                Box::new(Expression::Pipe(
                    Box::new(Expression::Binding(Identifier::from("a_bc"))),
                    PipeType::Flow,
                    Box::new(Expression::Pipe(
                        Box::new(Expression::Binding(Identifier::from("e_FG"))),
                        PipeType::Flow,
                        Box::new(Expression::Binding(Identifier::from("hij")))
                    ))
                ))
            )
        )
    }
}
