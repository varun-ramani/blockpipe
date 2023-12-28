use std::mem::discriminant;

use super::lexer::Token;
use logos::Span;

// parse a vector of tokens into an AST
pub struct Parser {
    tokens: Vec<(Token, Span)>,
    index: usize,
}

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

    /// consume_token
    ///
    /// Consumes a token with the same discriminant as the provided token.
    /// Returns parser error with the provided function name if this is not feasible.
    fn consume_token(
        &mut self,
        ast_node_name: &str,
        expected_tok: &Token,
    ) -> Result<(), ParserError> {
        let (tok, span) = &self.tokens[self.curr_index(ast_node_name)?];
        if discriminant(tok) != discriminant(expected_tok) {
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
        self.consume_token("tuple", &Token::LeftParen)?;
        let mut ret_vec: Vec<ASTNode> = vec![];
        loop {
            if let (Token::RightParen, _) =
                &self.tokens[self.curr_index("tuple")?]
            {
                self.consume_token("tuple", &Token::RightParen)?;
                break;
            }

            ret_vec.push(self.parse()?);
        }

        println!("successfully exiting parse tuple");
        Ok(ASTNode::Tuple(ret_vec))
    }

    /// parse_block
    ///
    /// Given a situation in which the current token is a left brace,
    /// starts parsing a block from that location.
    fn parse_block(&mut self) -> ParseResult {
        self.consume_token("block", &Token::LeftBrace)?;
        let mut ret_vec: Vec<ASTNode> = vec![];
        loop {
            if let (Token::RightBrace, _) =
                &self.tokens[self.curr_index("block")?]
            {
                self.consume_token("block", &Token::RightBrace)?;
                break;
            }

            ret_vec.push(self.parse()?);
        }

        Ok(ASTNode::Block(ret_vec))
    }
}

#[cfg(test)]
mod tests {
    use logos::Span;

    use crate::language::lexer::{lex, Token};

    use super::ASTNode;
    use super::LiteralVariant;
    use super::Parser;
    use super::PipeType;

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

        assert_eq!(lex_and_parse(code), Ok(super::ASTNode::Tuple(vec![])));
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

    #[test]
    fn test_nested_tuple() {
        let code = r"(())";

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Tuple(vec![super::ASTNode::Tuple(vec![])]))
        )
    }

    #[test]
    fn test_empty_block() {
        let code = "{}";

        assert_eq!(lex_and_parse(code), Ok(super::ASTNode::Block(vec![])));
    }

    #[test]
    fn test_literal_block() {
        let code = r#"
            {
                "hello"
                "world"
                2
                2.0
                -2
                -2.0
            }
        "#;

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Block(vec![
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

    #[test]
    fn test_nested_block() {
        let code = r"{{} {}}";

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Block(vec![
                super::ASTNode::Block(vec![]),
                super::ASTNode::Block(vec![])
            ]))
        )
    }

    #[test]
    fn test_identifier() {
        let code = "hello world";

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Identifier("hello".to_string()))
        );
    }

    #[test]
    fn test_binding_literal() {
        let code = r#"
            (a: "hello"
             b: 2)
        "#;

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Tuple(vec![
                super::ASTNode::Binding((
                    "a".to_string(),
                    Box::new(super::ASTNode::Literal(
                        super::LiteralVariant::StringLiteral(
                            "hello".to_string()
                        )
                    ))
                )),
                super::ASTNode::Binding((
                    "b".to_string(),
                    Box::new(super::ASTNode::Literal(
                        super::LiteralVariant::IntegerLiteral(2)
                    ))
                )),
            ]))
        );
    }

    #[test]
    fn test_bind_identifier() {
        let code = r#"
            (
                a: bruh
                b: string
            )
        "#;

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Tuple(vec![
                super::ASTNode::Binding((
                    "a".to_string(),
                    Box::new(super::ASTNode::Identifier("bruh".to_string()))
                )),
                super::ASTNode::Binding((
                    "b".to_string(),
                    Box::new(super::ASTNode::Identifier("string".to_string()))
                )),
            ]))
        )
    }

    #[test]
    fn test_type() {
        let code = r#"
            type (a: string
                  b: integer)
        "#;

        assert_eq!(
            lex_and_parse(code),
            Ok(super::ASTNode::Type(Box::new(super::ASTNode::Tuple(vec![
                super::ASTNode::Binding((
                    "a".to_string(),
                    Box::new(super::ASTNode::Identifier("string".to_string()))
                )),
                super::ASTNode::Binding((
                    "b".to_string(),
                    Box::new(super::ASTNode::Identifier("integer".to_string()))
                )),
            ]))))
        );
    }

    #[test]
    fn test_basic_pipe() {
        let code = r#"
            a | b
        "#;

        let expected_ast = super::ASTNode::Pipe(
            vec![
                super::ASTNode::Identifier("a".to_string()),
                super::ASTNode::Identifier("b".to_string()),
            ],
            vec![super::PipeType::Standard],
        );

        assert_eq!(lex_and_parse(code), Ok(expected_ast));
    }

    #[test]
    fn test_basic_pipe_chain() {
        let code = r#"
            a | b | c
        "#;

        let expected_ast = super::ASTNode::Pipe(
            vec![
                super::ASTNode::Identifier("a".to_string()),
                super::ASTNode::Identifier("b".to_string()),
                super::ASTNode::Identifier("c".to_string()),
            ],
            vec![super::PipeType::Standard, super::PipeType::Standard],
        );

        assert_eq!(lex_and_parse(code), Ok(expected_ast));
    }

    #[test]
    fn test_mixed_pipe_chain() {
        let code = r#"
            a |* b | c |* d
        "#;

        let expected_ast = super::ASTNode::Pipe(
            vec![
                super::ASTNode::Identifier("a".to_string()),
                super::ASTNode::Identifier("b".to_string()),
                super::ASTNode::Identifier("c".to_string()),
                super::ASTNode::Identifier("d".to_string()),
            ],
            vec![
                super::PipeType::Destructure,
                super::PipeType::Standard,
                super::PipeType::Destructure,
            ],
        );

        assert_eq!(lex_and_parse(code), Ok(expected_ast));
    }

    #[test]
    fn parse_complete_blockpipe() {
        let code = r#"
            main: {
                paste (
                    "declarations.blkp"
                )

                add: {
                    in: type (arg1: integer
                              arg2: integer)
                    out: type (out1: integer)

                    (in "add") |* plz
                }

                (a b) |* add
            }"#;

        let expected_ast = ASTNode::Binding((
            "main".to_string(),
            Box::new(ASTNode::Block(vec![
                ASTNode::Paste(Box::new(ASTNode::Tuple(vec![
                    ASTNode::Literal(LiteralVariant::StringLiteral(
                        "declarations.blkp".to_string(),
                    )),
                ]))),
                ASTNode::Binding((
                    "add".to_string(),
                    Box::new(ASTNode::Block(vec![
                        ASTNode::Binding((
                            "in".to_string(),
                            Box::new(ASTNode::Type(Box::new(ASTNode::Tuple(
                                vec![
                                    ASTNode::Binding((
                                        "arg1".to_string(),
                                        Box::new(ASTNode::Identifier(
                                            "integer".to_string(),
                                        )),
                                    )),
                                    ASTNode::Binding((
                                        "arg2".to_string(),
                                        Box::new(ASTNode::Identifier(
                                            "integer".to_string(),
                                        )),
                                    )),
                                ],
                            )))),
                        )),
                        ASTNode::Binding((
                            "out".to_string(),
                            Box::new(ASTNode::Type(Box::new(ASTNode::Tuple(
                                vec![ASTNode::Binding((
                                    "out1".to_string(),
                                    Box::new(ASTNode::Identifier(
                                        "integer".to_string(),
                                    )),
                                ))],
                            )))),
                        )),
                        ASTNode::Pipe(
                            vec![
                                ASTNode::Tuple(vec![
                                    ASTNode::Identifier("in".to_string()),
                                    ASTNode::Literal(
                                        LiteralVariant::StringLiteral(
                                            "add".to_string(),
                                        ),
                                    ),
                                ]),
                                ASTNode::Identifier("plz".to_string()),
                            ],
                            vec![PipeType::Destructure],
                        ),
                    ])),

                )),
                ASTNode::Pipe(
                    vec![
                        ASTNode::Tuple(vec![
                            ASTNode::Identifier("a".to_string()),
                            ASTNode::Identifier("b".to_string()),
                        ]),
                        ASTNode::Identifier("add".to_string()),
                    ],
                    vec![PipeType::Destructure],
                ),
            ])),
        ));

        assert_eq!(lex_and_parse(code), Ok(expected_ast));
    }
}
