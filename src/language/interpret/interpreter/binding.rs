use crate::language::{
    interpret::{stack::InterpStack, value::Value},
    parse::ast::{Expression, Identifier},
};

pub fn interp_binding(
    stack: &mut InterpStack,
    id: Identifier,
) -> Result<Value, String> {
    Ok(stack
        .lookup(id.clone())
        .ok_or(format!("Invalid binding {}", id.clone()))?
        .clone())
}

#[cfg(test)]
mod tests {
    use crate::{language::{
        interpret::{
            interpreter::interpret_expression,
            stack::InterpStack,
            value::{PrimitiveType, Value},
        },
        parse::parser::parse_from_string,
    }, command::interpret};

    #[test]
    fn lookup_binding() {
        let mut stack = InterpStack::new();
        stack.push_frame();
        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Int(20))),
        ));
        stack.push_binding(Value::Bind(
            "b".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Int(30))),
        ));

        assert_eq!(
            interpret_expression(&mut stack, parse_from_string("a").unwrap()),
            Ok(Value::Primitive(PrimitiveType::Int(20))),
        );

        assert_eq!(
            interpret_expression(&mut stack, parse_from_string("b").unwrap()),
            Ok(Value::Primitive(PrimitiveType::Int(30))),
        );
    }
}
