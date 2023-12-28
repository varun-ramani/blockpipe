mod interpreter;
mod lexer;
mod parser;

use interpreter::{EvaluateResult, Interpreter, Value};
use lexer::Token;
use logos::{Logos, Span};
use parser::{ASTNode, ParseResult, Parser, ParserError};

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

    let transformed_parameters = parameters.map(|parameters| {
        parameters
            .into_iter()
            .map(|parameter| Value::String(parameter))
            .collect()
    });
    let res = interpreter.evaluate_from_root(transformed_parameters.clone())?;

    if execute_root {
        let parameters_vector = transformed_parameters.unwrap_or_default();
        Ok(interpreter.execute_closure(parameters_vector, &res)?)
    } else {
        Ok(res)
    }
}
