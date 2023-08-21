//! Contains code to parse blocks

use nom::character::complete::char;
use nom::multi::many0;
use nom::{sequence::delimited, IResult};

use crate::language::parse::ast::Expression;

use super::{ignore_ws, ParseRoot};

/// the core function that parses blocks
pub fn parse_block(input: &str) -> IResult<&str, Expression> {
    let (input, expressions) = delimited(
        ignore_ws(char('{')),
        many0(Expression::parse),
        ignore_ws(char('}')),
    )(input)?;
    Ok((input, Expression::Block(expressions)))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::parse::{ast::Expression, parser::block::parse_block};

    #[test]
    fn empty_block() {
        let (_, expr) = parse_block(indoc! {r"
            { }
        "})
        .expect("Failed to parse: ");
        assert_eq!(expr, Expression::Block(vec![]))
    }

    #[test]
    fn nested_block() {
        let (_, expr) = parse_block(indoc! {r"
            { {} }
        "})
        .expect("Failed to parse: ");
        assert_eq!(expr, Expression::Block(vec![Expression::Block(vec![])]))
    }

    #[test]
    fn multiple_nested_block() {
        let (_, expr) = parse_block(indoc! {r"
            { {} {{} { } } }
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Block(vec![
                Expression::Block(vec![]),
                Expression::Block(vec![
                    Expression::Block(vec![]),
                    Expression::Block(vec![])
                ])
            ])
        )
    }
}
