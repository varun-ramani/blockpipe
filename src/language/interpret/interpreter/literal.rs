use crate::language::{parse::ast::LiteralType, interpret::value::{Value, PrimitiveType, self}};

pub fn interp_literal(literal_type: LiteralType) -> Result<Value, String> {
    match literal_type {
        LiteralType::Bool(value) => Ok(Value::Primitive(PrimitiveType::Bool(value))),
        LiteralType::Float(value) => Ok(Value::Primitive(PrimitiveType::Float(value))),
        LiteralType::Str(value) => Ok(Value::Primitive(PrimitiveType::Str(value))),
        LiteralType::Int(value) => Ok(Value::Primitive(PrimitiveType::Int(value)))
    }
}