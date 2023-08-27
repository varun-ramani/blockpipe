use std::fmt::Debug;

use clap::{Parser, ValueEnum};

use crate::language::{
    interpret::{
        interpreter::{interpret_expression, interpret_program},
        stack::InterpStack,
        value::Value,
    },
    parse::{
        ast::{Expression},
        parser::{parse_from_string},
    },
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum InterpreterMode {
    Expression,
    Program,
}

#[derive(Parser, Debug)]
pub struct InterpreterArgs {
    #[arg(value_enum)]
    mode: InterpreterMode,
    filename: String,
}

pub fn interpreter_standalone(interp_args: &InterpreterArgs) {
    println!(
        "{:#?}",
        interp_file(interp_args.mode, &interp_args.filename)
    );
}

fn interp_file(mode: InterpreterMode, filename: &str) -> Result<Value, String> {
    let file_data = match std::fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    let parse_tree = match parse_from_string::<Expression>(&file_data) {
        Ok(parsed) => parsed,
        Err(parser_error) => return Err(parser_error.to_string()),
    };

    match mode {
        InterpreterMode::Program => interpret_program(parse_tree),
        InterpreterMode::Expression => {
            interpret_expression(&mut InterpStack::new(), parse_tree)
        }
    }
}
