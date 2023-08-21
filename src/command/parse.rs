use clap::{Parser, ValueEnum};

use crate::language::parse::{ast, parser::{parse_from_string, ParseRoot}};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ParseType {
    Expression,
    Identifier
}

#[derive(Parser, Debug)]
pub struct ParserArgs {
    #[arg(value_enum)]
    parse_type: ParseType,

    filename: String
}

pub fn parser_standalone(parser_args: &ParserArgs) {
    // TODO How do I clean up this code using a match statement?
    if parser_args.parse_type == ParseType::Expression {
        println!("{:?}", parse_file::<ast::Expression>(&parser_args.filename));
    } else if parser_args.parse_type == ParseType::Identifier {
        println!("{:?}", parse_file::<ast::Identifier>(&parser_args.filename));
    }
}

pub fn parse_file<T: ParseRoot>(filename: &str) -> Result<T, String> {
    let file_data = match std::fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string())
    };

    match parse_from_string(&file_data) {
        Ok(parsed) => Ok(parsed),
        Err(parser_error) => Err(parser_error.to_string())
    }
}   