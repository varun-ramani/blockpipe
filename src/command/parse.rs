use clap::Parser;
use nom::IResult;

use crate::language::parse::{ast::Expression, parser::parse_from_string};

#[derive(Parser, Debug)]
pub struct ParserArgs {
    filename: String
}

pub fn parser_standalone(parser_args: &ParserArgs) {
    println!("{:?}", parse_file(&parser_args.filename));
}

pub fn parse_file(filename: &str) -> Result<Expression, String> {
    let file_data = match std::fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string())
    };

    match parse_from_string(&file_data) {
        Ok(expression) => Ok(expression),
        Err(parser_error) => Err(parser_error.to_string())
    }
}   