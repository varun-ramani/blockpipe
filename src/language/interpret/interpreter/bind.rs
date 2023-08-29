use crate::language::{
    interpret::{
        stack::InterpStack,
        value::{Value},
    },
    parse::ast::{Expression, Identifier},
};

use super::interpret_expression;

pub fn interp_bind(
    stack: &mut InterpStack,
    id: Identifier,
    expr: Expression,
) -> Result<Value, String> {
    let result_value = interpret_expression(stack, expr)?;
    let boxed_result = Box::new(result_value);
    Ok(Value::Bind(id, boxed_result))
}
