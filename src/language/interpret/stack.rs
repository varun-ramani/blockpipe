use crate::language::parse::ast::Identifier;

use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct InterpStack {
    pub frames: Vec<StackFrame>,
    pub bindings: HashMap<String, Vec<Value>>
}

#[derive(Debug, PartialEq)]
pub struct StackFrame {
    pub identifiers: Vec<Identifier>
}