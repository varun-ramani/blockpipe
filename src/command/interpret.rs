use std::fmt::Debug;

use clap::{Parser, ValueEnum};

use crate::language::{
    interpret::{interpret_program, value::Value},
    parse::{
        ast::{self, Expression},
        parser::{parse_from_string, ParseRoot},
    },
};

#[derive(Parser, Debug)]
pub struct InterpreterArgs {
    filename: String,
}

pub fn interpreter_standalone(interp_args: &InterpreterArgs) {
    interp_file(&interp_args.filename);
}

pub fn interp_file(filename: &str) -> Result<Value, String> {
    let file_data = match std::fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    let parse_tree = match parse_from_string::<Expression>(&file_data) {
        Ok(parsed) => parsed,
        Err(parser_error) => return Err(parser_error.to_string()),
    };

    interpret_program(&parse_tree)
}
