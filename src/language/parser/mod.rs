use std::mem::discriminant;

use super::lexer::Token;
use logos::Span;

// parse a vector of tokens into an AST
struct Parser {
    tokens: Vec<(Token, Span)>,
    index: usize,
}

#[derive(Debug, PartialEq)]
enum ASTNode {
    Block(Vec<ASTNode>),
    Tuple(Vec<ASTNode>),
    Pipe(Vec<ASTNode>),
    Paste(Box<ASTNode>),
    Type(Box<ASTNode>),
    Binding((Box<ASTNode>, Box<ASTNode>)),
    Identifier(String),
    Literal(LiteralVariant),
}

#[derive(Debug, PartialEq)]
enum LiteralVariant {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    FloatLiteral(f64),
}

/// generic parser result. either an ast node or a [ParserError]
type ParseResult = Result<ASTNode, ParserError>;

/// the type of the error - contains the ast node that threw the error, the
/// error message, and the byte span in the input from which the error
/// originated.
type ParserError = (String, String, Span);

impl Parser {
    fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self { tokens, index: 0 }
    }

    /// current_token
    ///
    /// Obtains the token at the current index and if this is not possible,
    /// returns an error.
    fn current_token(
        &self,
        ast_node_name: &str,
    ) -> Result<&(Token, Span), ParserError> {
        if self.index < self.tokens.len() {
            Ok(&self.tokens[self.index])
        } else {
            let last_token_span = match self.tokens.last() {
                Some((_, span)) => span.clone(),
                None => 0..1,
            };

            Err((
                ast_node_name.to_string(),
                "Unexpected end of the input".to_string(),
                last_token_span,
            ))
        }
    }

    /// consume_token
    ///
    /// Consumes a token with the same discriminant as the provided token.
    /// Returns parser error with the provided function name if this is not feasible.
    fn consume_token(
        &mut self,
        ast_node_name: &str,
        expected_tok: &Token,
    ) -> Result<(), ParserError> {
        let (curr_tok, span) = self.current_token(ast_node_name)?;
        if discriminant(curr_tok) != discriminant(expected_tok) {
            Err((
                ast_node_name.to_string(),
                format!("Expected '{}'", expected_tok),
                span.clone(),
            ))
        } else {
            self.index += 1;
            Ok(())
        }
    }

    fn parse(&mut self) -> ParseResult {
        let (tok, span) = self.current_token("expr root")?;

        /*
         * TODO: Handle cases q
         */
        let ret = match tok {
            Token::StringLiteral(value) => Ok(ASTNode::Literal(
                LiteralVariant::StringLiteral(value.clone()),
            )),
            Token::IntegerLiteral(value) => Ok(ASTNode::Literal(
                LiteralVariant::IntegerLiteral(value.clone()),
            )),
            Token::BooleanLiteral(value) => Ok(ASTNode::Literal(
                LiteralVariant::BooleanLiteral(value.clone()),
            )),
            Token::FloatLiteral(value) => Ok(ASTNode::Literal(
                LiteralVariant::FloatLiteral(value.clone()),
            )),
            Token::LeftParen => self.parse_tuple(),
            _ => Err((
                "expression".to_string(),
                "undefined error".to_string(),
                span.clone(),
            )),
        };

        if let Ok(_) = ret {
            self.index += 1;
        };

        ret
    }

    /// parse_tuple
    ///
    /// Given a situation in which the current token is a left parenthesis,
    /// starts parsing a tuple from that location.
    fn parse_tuple(&mut self) -> ParseResult {
        self.consume_token("tuple", &Token::LeftParen)?;
        let mut ret_vec: Vec<ASTNode> = vec![];
        loop {
            if let (Token::RightParen, _) = self.current_token("tuple")? {
                self.consume_token("tuple", &Token::RightParen)?;
                break;
            }

            ret_vec.push(self.parse()?);
        }

        Ok(ASTNode::Tuple(ret_vec))
    }
}

#[cfg(test)]
mod tests {
    use logos::Span;

    use crate::language::lexer::{lex, Token};

    use super::Parser;

    fn lex_unconditionally(input: &str) -> Vec<(Token, Span)> {
        lex(input)
            .into_iter()
            .map(|(tok, span)| (tok.unwrap(), span))
            .collect()
    }

    fn lex_and_parse(
        input: &str,
    ) -> Result<super::ASTNode, super::ParserError> {
        Parser::new(lex_unconditionally(input)).parse()
    }

    #[test]
    fn test_integer() {
        let code = "1 12 -1 -12";

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Literal(
                super::LiteralVariant::IntegerLiteral(1)
            ))
        );
    }

    #[test]
    fn test_empty_tuple() {
        let code = "()";

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Tuple(vec![]))
        );
    }

    #[test]
    fn test_literal_tuple() {
        let code = r#"
            (
                "hello"
                "world"
                2
                2.0
                -2
                -2.0
            )
        "#;

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Tuple(vec![
                super::ASTNode::Literal(super::LiteralVariant::StringLiteral(
                    "hello".to_string()
                )),
                super::ASTNode::Literal(super::LiteralVariant::StringLiteral(
                    "world".to_string()
                )),
                super::ASTNode::Literal(super::LiteralVariant::IntegerLiteral(
                    2
                )),
                super::ASTNode::Literal(super::LiteralVariant::FloatLiteral(
                    2.0
                )),
                super::ASTNode::Literal(super::LiteralVariant::IntegerLiteral(
                    -2
                )),
                super::ASTNode::Literal(super::LiteralVariant::FloatLiteral(
                    -2.0
                )),
            ]))
        );
    }
}
