use clap::Id;

use crate::language::parse::ast::Identifier;

use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct InterpStack {
    pub frames: Vec<StackFrame>,
    pub bindings: HashMap<Identifier, Vec<Value>>,
}

#[derive(Debug, PartialEq)]
pub struct StackFrame {
    pub identifiers: Vec<Identifier>,
}

impl InterpStack {
    /// creates a new stack. this should only be called once, when we start the
    /// interpreter
    pub fn new() -> InterpStack {
        InterpStack {
            frames: Vec::new(),
            bindings: HashMap::new(),
        }
    }

    /// creates a new stack frame. this should be called every time we start
    /// evaluating a closure
    pub fn push_frame(&mut self) {
        self.frames.push(StackFrame::new())
    }

    /// binds a new identifier in the context of the last stack frame created
    pub fn push_binding(&mut self, binding: Value) {
        match binding {
            Value::Bind(id, value) => {
                // clone the ID
                let id_clone: Identifier = id.clone();

                // push the new identifier onto the frame
                self.frames
                    .last_mut()
                    .expect("Did not find a StackFrame to work with")
                    .identifiers
                    .push(id);

                // then bind the new identifier
                self.bindings
                    .entry(id_clone)
                    .or_insert_with(Vec::new)
                    .push(*value);
            }
            _ => panic!(
                "trying to push a value that is not a binding onto the stack"
            ),
        }
    }

    /// removes the last occurrence of the provided key from the bindings
    pub fn unbind(&mut self, id: Identifier) {
        let key_vector = self
            .bindings
            .get_mut(&id)
            .expect("provided key to unbind() that does not exist");

        key_vector.pop();

        if key_vector.len() == 0 {
            self.bindings.remove(&id);
        }
    }

    /// looks up a binding
    pub fn lookup(&self, id: Identifier) -> Option<&Value> {
        self.bindings.get(&id).and_then(|value| value.last())
    }

    /// removes the last stack frame from the map
    pub fn pop_frame(&mut self) {
        let last_frame = self
            .frames
            .pop()
            .expect("Asked to pop frame but no such frame exists");
        last_frame
            .identifiers
            .into_iter()
            .rev()
            .for_each(|id| self.unbind(id))
    }
}

impl StackFrame {
    pub fn new() -> StackFrame {
        StackFrame {
            identifiers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::interpret::value::{PrimitiveType, Value};

    use super::InterpStack;

    #[test]
    fn add_frame() {
        let mut stack = InterpStack::new();
        stack.push_frame();
    }

    #[test]
    fn rm_frame() {
        let mut stack = InterpStack::new();
        stack.push_frame();
        stack.pop_frame();
    }

    #[test]
    fn lookup() {
        let mut stack = InterpStack::new();
        stack.push_frame();
        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Bool(true))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Bool(true))));
        stack.pop_frame();
    }

    #[test]
    fn shadow() {
        let mut stack = InterpStack::new();
        stack.push_frame();

        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Bool(true))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Bool(true))));

        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Float(1.0))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Float(1.0))));

        stack.push_frame();
        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Int(2))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Int(2))));

        stack.pop_frame();
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Float(1.0))));
    }

    #[test]
    fn unbind() {
        let mut stack = InterpStack::new();
        stack.push_frame();

        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Bool(true))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Bool(true))));

        stack.push_binding(Value::Bind(
            "a".to_owned(),
            Box::new(Value::Primitive(PrimitiveType::Float(1.0))),
        ));
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Float(1.0))));

        stack.unbind("a".to_owned());
        let value = stack.lookup("a".to_owned());
        assert_eq!(value, Some(&Value::Primitive(PrimitiveType::Bool(true))));
    }
}
