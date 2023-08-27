#[derive(Debug, PartialEq)]
pub enum Value {
    Closure(),
    
    // Any piece of data evaluates to this type
    Data(PrimitiveType),

    // This is the void datatype
    Unit 
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}