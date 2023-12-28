use crate::lexer::Token;
use logos::Span;
use super::*;

// parse a vector of tokens into an AST
pub struct Parser {
    tokens: Vec<(Token, Span)>,
    index: usize,
}

/// generic parser result. either an ast node or a [ParserError]
pub type ParseResult = Result<ASTNode, ParserError>;

/// the type of the error - contains the ast node that threw the error, the
/// error message, and the byte span in the input from which the error
/// originated.
pub type ParserError = (String, String, Span);

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self { tokens, index: 0 }
    }

    /// current_token
    ///
    /// Obtains the current index or errors out if the index is out of bounds.
    fn curr_index(&self, ast_node_name: &str) -> Result<usize, ParserError> {
        if self.index < self.tokens.len() {
            Ok(self.index)
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

    pub fn parse(&mut self) -> ParseResult {
        let expr1 = self.parse_self_contained()?;

        // we need to handle the case in which we're done
        if self.index >= self.tokens.len() {
            return Ok(expr1);
        }

        let (tok, _) = &self.tokens[self.curr_index("expression")?];
        match (expr1, tok) {
            // could be a binding
            (ASTNode::Identifier(value), Token::Colon) => {
                self.index += 1;
                Ok(ASTNode::Binding((value, Box::new(self.parse()?))))
            }

            // could be a pipe
            (expr1, Token::Pipe) | (expr1, Token::PipeStar) => {
                self.index += 1;
                let pipe_type = match tok {
                    Token::Pipe => PipeType::Standard,
                    Token::PipeStar => PipeType::Destructure,
                    _ => unreachable!(),
                };
                let expr2 = self.parse()?;

                // one slightly weird thing that we need to take care of is pipe
                // chains - if the second expression is also a pipe, then we'll fold the
                // pipe chain into a single ASTNode::Pipe
                if let ASTNode::Pipe(mut expr_vec, mut pipetype_vec) = expr2 {
                    expr_vec.insert(0, expr1);
                    pipetype_vec.insert(0, pipe_type);

                    Ok(ASTNode::Pipe(expr_vec, pipetype_vec))
                } else {
                    Ok(ASTNode::Pipe(vec![expr1, expr2], vec![pipe_type]))
                }
            }

            // but if it's neither, then we can just return it
            (expr1, _) => Ok(expr1),
        }
    }

    fn parse_self_contained(&mut self) -> ParseResult {
        let (tok, span) = &self.tokens[self.curr_index("self contained")?];

        /*
         * TODO: Handle cases q
         */
        let ret = match tok {
            Token::StringLiteral(value) => {
                self.index += 1;
                Ok(ASTNode::Literal(LiteralVariant::StringLiteral(
                    value.clone(),
                )))
            }
            Token::IntegerLiteral(value) => {
                self.index += 1;
                Ok(ASTNode::Literal(LiteralVariant::IntegerLiteral(
                    value.clone(),
                )))
            }
            Token::BooleanLiteral(value) => {
                self.index += 1;
                Ok(ASTNode::Literal(LiteralVariant::BooleanLiteral(
                    value.clone(),
                )))
            }
            Token::FloatLiteral(value) => {
                self.index += 1;
                Ok(ASTNode::Literal(LiteralVariant::FloatLiteral(
                    value.clone(),
                )))
            }
            Token::LeftParen => self.parse_tuple(),
            Token::LeftBrace => self.parse_block(),
            Token::Type => {
                self.index += 1;
                Ok(ASTNode::Type(Box::new(self.parse_tuple()?)))
            }
            Token::Paste => {
                self.index += 1;
                Ok(ASTNode::Paste(Box::new(self.parse_tuple()?)))
            }
            Token::Identifier(value) => {
                self.index += 1;
                Ok(ASTNode::Identifier(value.clone()))
            }
            _ => Err((
                "expression".to_string(),
                "undefined error".to_string(),
                span.clone(),
            )),
        };

        ret
    }

    /// parse_tuple
    ///
    /// Given a situation in which the current token is a left parenthesis,
    /// starts parsing a tuple from that location.
    fn parse_tuple(&mut self) -> ParseResult {
        self.index += 1;
        let mut ret_vec: Vec<ASTNode> = vec![];
        loop {
            if let (Token::RightParen, _) =
                &self.tokens[self.curr_index("tuple")?]
            {
                self.index += 1;
                break;
            }

            ret_vec.push(self.parse()?);
        }

        Ok(ASTNode::Tuple(ret_vec))
    }

    /// parse_block
    ///
    /// Given a situation in which the current token is a left brace,
    /// starts parsing a block from that location.
    fn parse_block(&mut self) -> ParseResult {
        self.index += 1;
        let mut ret_vec: Vec<ASTNode> = vec![];
        loop {
            if let (Token::RightBrace, _) =
                &self.tokens[self.curr_index("block")?]
            {
                self.index += 1;
                break;
            }

            ret_vec.push(self.parse()?);
        }

        Ok(ASTNode::Block(ret_vec))
    }
}