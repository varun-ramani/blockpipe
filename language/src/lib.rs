mod interpreter;
mod lexer;
mod parser;

use interpreter::{EvaluateResult, Interpreter, Value};
use lexer::Token;
use logos::{Logos, Span};
use parser::{ASTNode, ParseResult, Parser, ParserError};
use wasm_bindgen::prelude::*;

pub fn lex_from_string(input: &str) -> Vec<(Result<Token, ()>, Span)> {
    Token::lexer(input).spanned().collect()
}

pub fn parse_from_string(input: &str) -> ParseResult {
    Parser::new(
        lex_from_string(input)
            .into_iter()
            .map(|(tok, span)| Ok((tok?, span)))
            .collect::<Result<Vec<(Token, Span)>, ()>>()
            .ok()
            .ok_or(("Root".to_string(), "Parsing Failed".to_string(), 0..1))?,
    )
    .parse()
}

pub fn interpret_from_string(
    input: &str,
    parameters: Option<Vec<String>>,
    execute_root: bool,
) -> EvaluateResult {
    let ast = parse_from_string(input)
        .ok()
        .ok_or("Failed to interpret".to_string())?;
    let mut interpreter = Interpreter::new(ast);

    interpreter.env.push_stack_frame();
    interpreter
        .env
        .bind("plz".to_string(), Value::RuntimeInvocation);

    let transformed_parameters = parameters.map(|parameters| {
        parameters
            .into_iter()
            .map(|parameter| Value::String(parameter))
            .collect()
    });
    let res = interpreter.evaluate_from_root(transformed_parameters.clone())?;

    if execute_root {
        let parameters_vector = transformed_parameters.unwrap_or_default();
        Ok(Interpreter::execute_closure(parameters_vector, &res)?)
    } else {
        Ok(res)
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_interpret_from_string(
    input: &str,
    parameters: Option<Vec<String>>,
    execute_root: bool
) -> String {
    let res = interpret_from_string(input, parameters, execute_root);
    match res {
        Ok(ret_value) => {
            ret_value.to_string()
        },
        Err(some_err) => {
            some_err.to_string()
        }
    }
}