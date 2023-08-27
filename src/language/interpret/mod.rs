use super::parse::ast::Expression;

pub mod value;
pub mod stack;

use value::Value;

pub fn interpret_program(e: &Expression) -> Result<Value, String> {
    Ok(Value::Data(value::PrimitiveType::Str("Implement this".to_owned())))
}