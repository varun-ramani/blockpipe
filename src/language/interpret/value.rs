#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Closure(),

    // A few built-in higher order 
    // types
    Tuple(Vec<Value>),
    Bind(String, Box<Value>),
    
    // This is the most basic type of value that 
    // everything else is composed from
    Primitive(PrimitiveType),

    // This is the void datatype
    Unit 
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Value {
    pub fn enforce_tuple(self) -> Value {
        match self {
            Value::Tuple(data) => Value::Tuple(data),
            Value::Unit => Value::Tuple(vec![]),
            e => Value::Tuple(vec![e])
        }
    }

    pub fn evaluate(&self, args: Vec<Value>) -> Result<Value, String> {
        // the reason why we need to clone the value here is because values can
        // be reused in Blockpipe
        let cloned_self = self.clone();
        match cloned_self {
            Value::Unit => Ok(Value::Tuple(args)),
            Value::Primitive(data) => Ok(Value::Primitive(data)),
            _ => Err("Unsupported by blockpipe".to_owned())
        }
    }
}
