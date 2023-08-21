mod bind;
mod block;
mod identifier;
mod literal;
mod pipe;
mod tuple;

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::{sequence::delimited, IResult};

use self::bind::parse_bind;
use self::block::parse_block;
use self::identifier::parse_identifier;
use self::literal::parse_literal;
use self::pipe::parse_pipe;
use self::tuple::parse_tuple;

use super::ast::Identifier;
use super::{ast::Expression, error::ParserError};

/// this is the parser "root". it is the only level of parsing in which we
/// don't use nom.
pub fn parse_from_string<T: ParseRoot>(input: &str) -> Result<T, ParserError> {
    let res = T::parse(input);
    match res {
        Ok(("", expression)) => Ok(expression),
        Ok(_) => Err(ParserError::Remainder),
        Err(e) => Err(ParserError::NomError(e.to_string())),
    }
}

pub trait ParseRoot {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl ParseRoot for Expression {
    fn parse(input: &str) -> IResult<&str, Expression> {
        let res = ignore_ws(alt((
            parse_pipe,
            parse_literal,
            parse_block,
            parse_tuple,
            parse_bind,
        )))(input)?;
        Ok(res)
    }
}

impl ParseRoot for Identifier {
    fn parse(input: &str) -> IResult<&str, Identifier> {
        parse_identifier(input)
    }
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
