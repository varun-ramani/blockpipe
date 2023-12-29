use core::fmt;
use std::{collections::{HashMap, HashSet}, fmt::Formatter, fmt::Display};
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Tuple(t) => {
                let mut s = String::from("(");
                for (i, v) in t.iter().enumerate() {
                    s.push_str(&format!("{}", v));
                    if i != t.len() - 1 {
                        s.push_str(" ");
                    }
                }
                s.push_str(")");
                write!(f, "{}", s)
            },
            Value::Closure(_, _) => write!(f, "<closure>"),
            Value::RuntimeInvocation => write!(f, "<runtime invocation>"),
        }
    }
}