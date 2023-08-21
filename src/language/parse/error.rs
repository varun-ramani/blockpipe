use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParserError {
    Remainder,
    NomError(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
