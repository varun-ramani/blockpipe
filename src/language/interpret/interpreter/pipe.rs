use crate::language::{
    interpret::{stack::InterpStack, value::Value},
    parse::ast::{Expression, PipeType},
};

use super::interpret_expression;

pub fn interp_pipe(
    stack: &mut InterpStack,
    e1: Expression,
    pipe_type: PipeType,
    e2: Expression,
) -> Result<Value, String> {
    // remember that all pipes are actually just pipelines. furthermore, we
    // can't work with deeply nested Values. therefore, it's worth starting off
    // by flattening the parse tree into a list of Expressions instead of a
    // single deeply nested expression
    let mut expressions: Vec<(PipeType, Expression)> = vec![(PipeType::Flow, e1)];
    let mut curr_state = (pipe_type, e2);
    loop {
        let (old_pipe_type, e2) = curr_state;
        match e2 {
            Expression::Pipe(e1, new_pipe_type, e2) => {
                curr_state = (new_pipe_type, *e2);
                expressions.push((old_pipe_type, *e1));
            }
            expr => {
                expressions.push((old_pipe_type, expr));
                break;
            }
        };
    }

    // now we're ready to turn the vector of expressions into a vector of values
    let values: Vec<(PipeType, Value)> = match expressions
        .into_iter()
        .map(|(pipe_type, expr)| {
            interpret_expression(stack, expr).map(|value| (pipe_type, value))
        })
        .collect()
    {
        Ok(values) => values,
        Err(err) => return Err(err),
    };

    // now we can interpret the pipeline by just invoking it with () as an
    // argument
    let mut current_result = Value::Unit;
    for (pipe_type, section) in values {
        let potential_result = match pipe_type {
            PipeType::Flow => section.evaluate(vec![current_result]),
            PipeType::Destructure => {
                let inner_data = match current_result.enforce_tuple() {
                    Value::Tuple(data) => data,
                    _ => panic!("enforce_tuple returned non-tuple data. this is a bug in Blockpipe")
                };
                section.evaluate(inner_data)
            }
        };

        current_result = potential_result?;
    }

    Ok(current_result)
}

