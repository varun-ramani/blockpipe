use crate::language::parse::ast::Expression;

use super::{stack::InterpStack, value::Value};

pub fn interpret_program(e: Expression) -> Result<Value, String> {
    if let Expression::Bind(binding, expr) = e {
        if binding == "main" {
            let mut stack = InterpStack::new();
            interpret_expression(&mut stack, &expr)
        } else {
            Err("Program root expression is a binding, but it needs to be to main".to_owned())
        }
    } else {
        Err("Expected program root expression to be a binding".to_owned())
    }
}

pub fn interpret_expression(
    stack: &mut InterpStack,
    expr: &Expression,
) -> Result<Value, String> {
    Ok(Value::Unit)
}