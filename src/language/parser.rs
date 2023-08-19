use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug)]
pub enum Expression {
    Block(Vec<Expression>),
    Pipe(Box<Expression>, Box<Expression>),
    Tuple(Vec<Expression>),
    Literal(LiteralType)
}

#[derive(Debug)]
pub enum ParserError {
    Remainder,
    NomError(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl Expression {
    /// this is the parser "root". it is the only level of parsing in which we
    /// don't use nom.
    pub fn parse_from_string(input: &str) -> Result<Expression, ParserError> {
        let res = Self::parse(input);
        match res {
            Ok(("", expression)) => Ok(expression),
            Ok(_) => Err(ParserError::Remainder),
            Err(e) => Err(ParserError::NomError(e.to_string())),
        }
    }

    /// this is the root function that parses expressions
    fn parse(input: &str) -> IResult<&str, Expression> {
        let res = alt((Self::parse_pipe, Self::parse_block))(input)?;
        Ok(res)
    }

    fn parse_block(input: &str) -> IResult<&str, Expression> {
        let (input, expressions) = delimited(
            Self::ignore_ws(char('{')),
            many0(Self::parse),
            Self::ignore_ws(char('}')),
        )(input)?;
        Ok((input, Expression::Block(expressions)))
    }

    fn parse_pipe(input: &str) -> IResult<&str, Expression> {
        let (input, (expr1, expr2)) =
            separated_pair(Self::parse_block, Self::ignore_ws(char('|')), Self::parse)(input)?;
        Ok((input, Expression::Pipe(Box::new(expr1), Box::new(expr2))))
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
}
