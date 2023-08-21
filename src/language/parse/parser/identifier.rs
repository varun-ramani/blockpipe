use nom::{
    character::complete::satisfy, combinator::recognize, multi::many0,
    sequence::pair, IResult,
};

use crate::language::parse::ast::Identifier;

/// Recognizes identifiers - a string of length at least one that must start
/// with a letter or underscore, then contain any number of
/// alphanumeric/underscore characters.
pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let (input, token) = recognize(pair(
        satisfy(|c| c.is_alphabetic() || c == '_'),
        // TODO how do I clean the disgusting parameterization here?
        many0::<&str, char, nom::error::Error<&str>, _>(satisfy(|c| {
            c.is_alphanumeric() || c == '_'
        })),
    ))(input)?;

    Ok((input, token.to_owned()))
}

#[cfg(test)]
mod tests {
    use crate::language::{
        parse::ast::Identifier, parse::parser::parse_from_string,
    };

    macro_rules! test_reflect_input {
        ($test_name: ident, $source: literal) => {
            #[test]
            fn $test_name() {
                let source = $source;
                let parsed: Identifier = parse_from_string(source)
                    .expect("Failed to parse single character: ");
                assert_eq!(parsed, $source)
            }
        };
    }

    macro_rules! test_fail_parse {
        ($test_name: ident, $source: literal) => {
            #[test]
            fn $test_name() {
                let source = $source;
                parse_from_string::<Identifier>(source)
                    .expect_err("Expected parsing to fail, but it passed: ");
            }
        };
    }

    test_reflect_input!(single_lowercase_alpha, "a");
    test_reflect_input!(single_uppercase_alpha, "A");
    test_reflect_input!(underscore, "_");
    test_fail_parse!(single_number, "0");

    test_reflect_input!(lowercase_alpha, "abc");
    test_reflect_input!(mixedcase_alpha, "aBc");
    test_reflect_input!(long_identifier, "aBCababE");
    test_reflect_input!(underscore_then_long_id, "_abcbEabsb");
    test_reflect_input!(wrapped_underscore, "__abc__");
    test_reflect_input!(underscore_wrapped, "abc__abc");
    test_reflect_input!(alphanumeric_valid, "b2a");
    test_fail_parse!(alphanumeric_invalid, "2ba");

    test_reflect_input!(longest_id, "_1bab2ak2j__ank2j3k1bsh__2k3k1ba__");
    test_fail_parse!(longest_id_bad, "1__");

}
