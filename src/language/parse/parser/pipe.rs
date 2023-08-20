use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use nom::branch::alt;


use crate::language::parse::ast::Expression;

use super::block::parse_block;

// pub fn parse_pipe(input: &str) -> IResult<&str, Expression> {
//     let (input, (expr1, expr2)) =
//         alt(
//             parse_destructure,
//             parse_flow
//         )
        
//         separated_pair(parse, Self::ignore_ws(char('|')), Self::parse)(input)?;
//     Ok((input, Expression::Pipe(Box::new(expr1), Box::new(expr2))))
// }