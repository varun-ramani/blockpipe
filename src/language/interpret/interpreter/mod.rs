mod literal;
mod tuple;
mod pipe;

use crate::language::parse::ast::Expression;

use self::{literal::interp_literal, tuple::interp_tuple, pipe::interp_pipe};

use super::{stack::InterpStack, value::Value};

pub fn interpret_program(e: Expression) -> Result<Value, String> {
    if let Expression::Bind(binding, expr) = e {
        if binding == "main" {
            let mut stack = InterpStack::new();
            interpret_expression(&mut stack, *expr)
        } else {
            Err("Program root expression is a binding, but it needs to be to main".to_owned())
        }
    } else {
        Err("Expected program root expression to be a binding".to_owned())
    }
}

pub fn interpret_expression(
    stack: &mut InterpStack,
    expr: Expression,
) -> Result<Value, String> {
    match expr {
        Expression::Literal(literal_type) => interp_literal(literal_type),
        Expression::Tuple(expressions) => interp_tuple(stack, expressions),
        Expression::Pipe(e1, pipe_type, e2) => interp_pipe(stack, *e1, pipe_type, *e2),
        e => return Err(format!("Unsupported expression type: {:#?}", e))
    }
}