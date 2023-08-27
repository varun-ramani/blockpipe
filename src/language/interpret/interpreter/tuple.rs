use crate::language::{
    interpret::{stack::InterpStack, value::Value},
    parse::ast::Expression,
};

use super::interpret_expression;

pub fn interp_tuple(
    stack: &mut InterpStack,
    expressions: Vec<Expression>,
) -> Result<Value, String> {
    let values: Vec<Value> = match expressions
        .into_iter()
        .map(|expr| interpret_expression(stack, expr))
        .collect()
    {
        Ok(values) => values,
        Err(err) => return Err(err),
    };

    Ok(Value::Tuple(values))
}
