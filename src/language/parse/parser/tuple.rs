//! Contains code to parse tuples, which are basically blocks but delimated by ()

use nom::{IResult, sequence::delimited};
use nom::character::complete::char;
use nom::multi::many0;

use crate::language::parse::ast::Expression;

use super::{ignore_ws, ParseRoot};

/// the core function that parses tuples
pub fn parse_tuple(input: &str) -> IResult<&str, Expression> {
    let (input, expressions) = delimited(
        ignore_ws(char('(')),
        many0(Expression::parse),
        ignore_ws(char(')')),
    )(input)?;
    Ok((input, Expression::Tuple(expressions)))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::parse::{ast::Expression, parser::tuple::parse_tuple};

    #[test]
    fn empty_tuple() {
        let (_, expr) = parse_tuple(indoc! {r"
            ( )
        "})
        .expect("Failed to parse: ");
        assert_eq!(expr, Expression::Tuple(vec![]))
    }

    #[test]
    fn nested_tuple() {
        let (_, expr) = parse_tuple(indoc! {r"
            (( ) )
        "})
        .expect("Failed to parse: ");
        assert_eq!(expr, Expression::Tuple(vec![Expression::Tuple(vec![])]))
    }

    #[test]
    fn multiple_nested_tuple() {
        let (_, expr) = parse_tuple(indoc! {r"
            ( () (() ( ) ) )
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Tuple(vec![
                Expression::Tuple(vec![]),
                Expression::Tuple(vec![
                    Expression::Tuple(vec![]),
                    Expression::Tuple(vec![])
                ])
            ])
        )
    }
}

