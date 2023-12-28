use logos::Span;

use crate::lexer::{lex, Token};

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