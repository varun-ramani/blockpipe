mod block;
mod literal;
mod pipe;
mod tuple;

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::{sequence::delimited, IResult};

use self::block::parse_block;
use self::literal::parse_literal;
use self::pipe::parse_pipe;
use self::tuple::parse_tuple;

use super::{ast::Expression, error::ParserError};

/// this is the parser "root". it is the only level of parsing in which we
/// don't use nom.
pub fn parse_from_string(input: &str) -> Result<Expression, ParserError> {
    let res = parse(input);
    match res {
        Ok(("", expression)) => Ok(expression),
        Ok(_) => Err(ParserError::Remainder),
        Err(e) => Err(ParserError::NomError(e.to_string())),
    }
}

/// this is the root function that parses expressions
pub fn parse(input: &str) -> IResult<&str, Expression> {
    let res =
        ignore_ws(alt((parse_pipe, parse_literal, parse_block, parse_tuple)))(
            input,
        )?;
    Ok(res)
}

fn ignore_ws<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}
