#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Block(Vec<ASTNode>),
    Tuple(Vec<ASTNode>),
    Pipe(Vec<ASTNode>, Vec<PipeType>),
    Paste(Box<ASTNode>),
    Type(Box<ASTNode>),
    Binding((String, Box<ASTNode>)),
    Identifier(String),
    Literal(LiteralVariant),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralVariant {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    FloatLiteral(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PipeType {
    Standard,
    Destructure,
}

