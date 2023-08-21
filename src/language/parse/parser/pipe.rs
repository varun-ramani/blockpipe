use core::panic;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::fail;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

use crate::language::parse::ast::Expression;
use crate::language::parse::ast::PipeType;

use super::block::parse_block;
use super::literal::parse_literal;
use super::tuple::parse_tuple;
use super::{ignore_ws, ParseRoot};

pub fn parse_pipe(input: &str) -> IResult<&str, Expression> {
    let (input, (expr1, pipe_char, expr2)) = ignore_ws(tuple((
        alt((parse_block, parse_tuple, parse_literal)),
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
