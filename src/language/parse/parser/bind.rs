use nom::bytes::complete::tag;
use nom::sequence::{pair, preceded};
use nom::IResult;

use crate::language::parse::ast::Expression;

use super::identifier::parse_identifier;
use super::ParseRoot;

pub fn parse_bind(input: &str) -> IResult<&str, Expression> {
    let (input, (identifier, expr)) = preceded(
        tag("bind"),
        pair(parse_identifier, Expression::parse),
    )(input)?;

    Ok((input, Expression::Bind(identifier, Box::new(expr))))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::{
        parse::ast::Expression,
        parse::ast::{Identifier, LiteralType, PipeType},
    };

    use super::parse_bind;

    #[test]
    fn integer() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name 20
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Literal(LiteralType::Int(20)))
            )
        )
    }

    #[test]
    fn float() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name 20.1
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Literal(LiteralType::Float(20.1)))
            )
        )
    }

    #[test]
    fn block() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name {10}
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Block(vec![Expression::Literal(
                    LiteralType::Int(10)
                )]))
            )
        )
    }

    #[test]
    fn tuple() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name (10 20 30)
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Tuple(vec![
                    Expression::Literal(LiteralType::Int(10)),
                    Expression::Literal(LiteralType::Int(20)),
                    Expression::Literal(LiteralType::Int(30))
                ]))
            )
        )
    }

    #[test]
    fn flow_pipe() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name 5 | {} 
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Pipe(
                    Box::new(Expression::Literal(LiteralType::Int(5))),
                    PipeType::Flow,
                    Box::new(Expression::Block(vec![]))
                ))
            )
        )
    }

    #[test]
    fn destructure_pipe() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name 5 |* {} 
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Pipe(
                    Box::new(Expression::Literal(LiteralType::Int(5))),
                    PipeType::Destructure,
                    Box::new(Expression::Block(vec![]))
                ))
            )
        )
    }

    #[test]
    fn pipeline() {
        let (_, expr) = parse_bind(indoc! {r"
            bind name 5 |* {} | {} | {(10 20) |* {}} 
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Bind(
                Identifier::from("name"),
                Box::new(Expression::Pipe(
                    Box::new(Expression::Literal(LiteralType::Int(5))),
                    PipeType::Destructure,
                    Box::new(Expression::Pipe(
                        Box::new(Expression::Block(vec![])),
                        PipeType::Flow,
                        Box::new(Expression::Pipe(
                            Box::new(Expression::Block(vec![])),
                            PipeType::Flow,
                            Box::new(Expression::Block(vec![
                                Expression::Pipe(
                                    Box::new(Expression::Tuple(vec![
                                        Expression::Literal(LiteralType::Int(
                                            10
                                        )),
                                        Expression::Literal(LiteralType::Int(
                                            20
                                        ))
                                    ])),
                                    PipeType::Destructure,
                                    Box::new(Expression::Block(vec![]))
                                )
                            ]))
                        ))
                    ))
                ))
            )
        )
    }
}
