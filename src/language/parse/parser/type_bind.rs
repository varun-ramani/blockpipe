use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{many0, many0_count};
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::{IResult, Parser};

use crate::language::parse::ast::Expression;

use super::bind::parse_bind;
use super::binding::parse_binding;
use super::identifier::{self, parse_identifier};
use super::tuple::parse_tuple;
use super::{ignore_ws, ParseRoot};

pub fn parse_type_bind(input: &str) -> IResult<&str, Expression> {
    let (input, expr) =
        preceded(ignore_ws(tag("type")), parse_bind_only_allow_types)(input)?;

    let (identifier, type_bind) = match expr {
        Expression::Bind(identifier, type_bind) => (identifier, type_bind),
        _ => panic!("Type bind's bind expression parsed to something that's not a bind. This is a bug inside Blockpipe")
    };

    Ok((input, Expression::TypeBind(identifier, type_bind)))
}

fn parse_bind_only_allow_types(input: &str) -> IResult<&str, Expression> {
    let (input, (ident, expr)) = ignore_ws(separated_pair(
        parse_identifier,
        ignore_ws(tag(":")),
        parse_type,
    ))(input)?;

    Ok((input, Expression::Bind(ident, Box::new(expr))))
}

pub fn parse_type(input: &str) -> IResult<&str, Expression> {
    ignore_ws(alt((
        parse_binding,
        delimited(
            ignore_ws(tag("(")),
            many0(parse_bind_only_allow_types),
            ignore_ws(tag(")")),
        )
        .map(|res| Expression::Tuple(res)),
    )))(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::language::parse::ast::{Expression, Identifier};

    use super::parse_type_bind;

    #[test]
    fn identifier_binding() {
        let (_, expr) = parse_type_bind(indoc! {r"
            type Identifier: String
        "})
        .expect("Failed to parse: ");
        assert_eq!(
            expr,
            Expression::TypeBind(
                Identifier::from("Identifier"),
                Box::new(Expression::Binding("String".to_owned()))
            )
        )
    }

    #[test]
    fn type_tuple_binding() {
        let (_, expr) = parse_type_bind(indoc! {r"
            type Person: (
                age: int
                weight: int
                favorites: (
                    color: Color
                    food: Food
                )
            )
        "})
        .expect("Failed to parse: ");

        assert_eq!(
            expr,
            Expression::TypeBind(
                Identifier::from("Person"),
                Box::new(Expression::Tuple(vec![
                    Expression::Bind(
                        Identifier::from("age"),
                        Box::new(Expression::Binding(Identifier::from("int")))
                    ),
                    Expression::Bind(
                        Identifier::from("weight"),
                        Box::new(Expression::Binding(Identifier::from("int")))
                    ),
                    Expression::Bind(
                        Identifier::from("favorites"),
                        Box::new(Expression::Tuple(vec![
                            Expression::Bind(
                                Identifier::from("color",),
                                Box::new(Expression::Binding(
                                    Identifier::from("Color")
                                ))
                            ),
                            Expression::Bind(
                                Identifier::from("food",),
                                Box::new(Expression::Binding(
                                    Identifier::from("Food")
                                ))
                            )
                        ]))
                    ),
                ]))
            )
        )
    }
}
