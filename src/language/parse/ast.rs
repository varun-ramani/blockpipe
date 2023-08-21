//! The definition of the abstract syntax tree (AST) lives 
//! inside this file. 


#[derive(Debug)]
/// The root node; all blockpipe programs are actually just expressions within
/// expressions.
pub enum Expression {
    Block(Vec<Expression>),
    Pipe(Box<Expression>, PipeType, Box<Expression>),
    Tuple(Vec<Expression>),
    Literal(LiteralType),
    Bind(Identifier, Box<Expression>)
}

/// Blockpipe supports string, integer, and float literals.
#[derive(Debug)]
pub enum LiteralType {
    Str(String),
    Int(i64),
    Float(f64),
}

/// If you want to pass multiple arguments to a closure, you'll have to pass
/// them as a tuple through a destructure pipe.
#[derive(Debug)]
pub enum PipeType {
    Destructure,
    Flow
}

/// We're just going to redefine the String datatype for clarity 
pub type Identifier = String;