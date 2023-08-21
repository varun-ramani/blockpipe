use nom::{
    branch::alt,
    bytes::complete::escaped,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit0, digit1, one_of, satisfy},
    combinator::opt,
    multi::many0,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

use crate::language::parse::ast::Expression;
use crate::language::parse::ast::LiteralType;

use super::ignore_ws;

pub fn parse_literal(input: &str) -> IResult<&str, Expression> {
    let (input, lit) =
        ignore_ws(alt((parse_float, parse_integer, parse_string)))(input)?;
    Ok((input, lit))
}

fn parse_integer(input: &str) -> IResult<&str, Expression> {
    let (input, (opt_neg, str_int)) = pair(opt(char('-')), digit1)(input)?;

    //  TODO properly handle this by bubbling up error
    let unsigned_int: i64 = str::parse(str_int)
        .expect(&format!("Failed to parse {} into 64-bit integer", str_int));

    let signed_int = match opt_neg {
        Some('-') => -unsigned_int,
        _ => unsigned_int,
    };

    Ok((input, Expression::Literal(LiteralType::Int(signed_int))))
}

fn parse_float(input: &str) -> IResult<&str, Expression> {
    // TODO can I implement this cleaner with regular expressions?
    let (input, (opt_neg, (before_dec, after_dec))) = pair(
        opt(char('-')),
        alt((
            separated_pair(digit0, char('.'), digit1),
            separated_pair(digit1, char('.'), digit0),
        )),
    )(input)?;

    let float_str = format!("{}.{}", before_dec, after_dec);
    let unsigned_float: f64 = format!("{}.{}", before_dec, after_dec)
        .parse()
        .expect(&format!("Failed to parse {} into 64-bit float", float_str));

    let signed_float = match opt_neg {
        Some('-') => -unsigned_float,
        _ => unsigned_float,
    };

    Ok((input, Expression::Literal(LiteralType::Float(signed_float))))
}

/// TODO implement string parsing
pub fn parse_string(input: &str) -> IResult<&str, Expression> {
    let content_parser =
        delimited(tag("\""), many0(satisfy(|char| char != '"')), tag("\""));
    let escape_parser = one_of(r#"""#);
    let (input, string) = escaped(content_parser, '\\', escape_parser)(input)?;
    Ok((
        input,
        Expression::Literal(LiteralType::Str(string.to_owned())),
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::parse::ast::{Expression, LiteralType};

    use super::parse_literal;

    #[test]
    fn positive_integer() {
        let (_, expr) = parse_literal(" 10 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Int(10)));
    }

    #[test]
    fn negative_integer() {
        let (_, expr) = parse_literal(" -10 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Int(-10)))
    }

    #[test]
    fn positive_float() {
        let (_, expr) = parse_literal(" 1.0 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(1.0)))
    }

    #[test]
    fn negative_float() {
        let (_, expr) = parse_literal(" -1.0 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(-1.0)))
    }

    #[test]
    fn positive_float_nothing_after_point() {
        let (_, expr) = parse_literal(" 10. ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(10.)))
    }

    #[test]
    fn positive_float_nothing_before_point() {
        let (_, expr) = parse_literal(" .10 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(0.1)))
    }

    #[test]
    fn negative_float_nothing_after_point() {
        let (_, expr) = parse_literal(" -10. ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(-10.)))
    }

    #[test]
    fn negative_float_nothing_before_point() {
        let (_, expr) = parse_literal(" -.10 ").expect("Failed to parse: ");
        assert_eq!(expr, Expression::Literal(LiteralType::Float(-0.1)))
    }

    #[test]
    fn basic_string() {
        let (_, expr) = parse_literal(indoc! {r#"
            "abc"
        "#})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Literal(LiteralType::Str(r#""abc""#.to_owned()))
        )
    }

    // TODO make this test pass
    #[test]
    fn escaped_string() {
        let (_, expr) = parse_literal(indoc! {r#"
            "a\"b"
        "#})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::Literal(LiteralType::Str(r#""a\"b""#.to_owned()))
        )
    }
}
