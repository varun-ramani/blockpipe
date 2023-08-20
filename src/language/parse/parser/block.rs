//! Contains code to parse blocks


use nom::{IResult, sequence::delimited};
use nom::character::complete::char;
use nom::multi::many0;

use crate::language::parse::ast::Expression;

use super::{ignore_ws, parse};

/// the core function that parses blocks
pub fn parse_block(input: &str) -> IResult<&str, Expression> {
    let (input, expressions) = delimited(
        ignore_ws(char('{')),
        many0(parse),
        ignore_ws(char('}')),
    )(input)?;
    Ok((input, Expression::Block(expressions)))
}
