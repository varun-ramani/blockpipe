use std::collections::{HashMap, HashSet};
use crate::parser::ASTNode;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    // primitive values
    Integer(i64),
    Boolean(bool),
    String(String),
    Float(f64),

    // tuples
    Tuple(Vec<Value>),

    // closure
    Closure(Vec<ASTNode>, HashMap<String, Value>),
    RuntimeInvocation, // special type of closure to invoke runtime calls
}