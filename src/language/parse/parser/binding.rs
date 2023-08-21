use nom::IResult;

use crate::language::parse::ast::Expression;

use super::identifier::parse_identifier;

pub fn parse_binding(input: &str) -> IResult<&str, Expression> {
    let (input, identifier) = parse_identifier(input)?;

    Ok((input, Expression::Binding(identifier)))
}

