#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}