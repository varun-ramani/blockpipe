use nom::{
    branch::alt,
    character::complete::char,
    character::complete::digit1,
    combinator::{fail, recognize},
    combinator::{map_res, opt},
    complete::tag,
    sequence::{pair, preceded},
    IResult,
};

use crate::language::parse::ast::Expression;
use crate::language::parse::ast::LiteralType;

pub fn parse_literal(input: &str) -> IResult<&str, Expression> {
    let (input, lit) = alt((parse_integer, parse_string))(input)?;
    Ok((input, lit))
}

fn parse_integer(input: &str) -> IResult<&str, Expression> {
    let (input, (opt_char, str_int)) = pair(opt(char('-')), digit1)(input)?;

    //  TODO properly handle this by bubbling up error
    let unsigned_int: i64 = str::parse(str_int)
        .expect(&format!("Failed to parse {} into 64-bit integer", str_int));

    let signed_int = match opt_char {
        Some('-') => -unsigned_int,
        _ => unsigned_int,
    };

    Ok((input, Expression::Literal(LiteralType::Int(signed_int))))
}

/// TODO implement string parsing
pub fn parse_string(input: &str) -> IResult<&str, Expression> {
    fail(input)
}
