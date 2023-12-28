use super::*;
use logos::{Logos, Span};

#[test]
fn test_integer() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("1 12 -1 -12").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::IntegerLiteral(1)), 0..1),
            (Ok(Token::IntegerLiteral(12)), 2..4),
            (Ok(Token::IntegerLiteral(-1)), 5..7),
            (Ok(Token::IntegerLiteral(-12)), 8..11),
        ]
    );
}

#[test]
fn test_float() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("1.0 12.0 -1.0 -12.0").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::FloatLiteral(1.0)), 0..3),
            (Ok(Token::FloatLiteral(12.0)), 4..8),
            (Ok(Token::FloatLiteral(-1.0)), 9..13),
            (Ok(Token::FloatLiteral(-12.0)), 14..19),
        ]
    );
}

#[test]
fn test_string() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("\"hi\" \"bye\"").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::StringLiteral("hi".to_string())), 0..4),
            (Ok(Token::StringLiteral("bye".to_string())), 5..10),
        ]
    );
}

#[test]
fn test_boolean() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("T F").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::BooleanLiteral(true)), 0..1),
            (Ok(Token::BooleanLiteral(false)), 2..3),
        ]
    );
}

#[test]
fn test_parentheses() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer(") ( )( () ( )").spanned().collect();
    assert_eq!(
        lexed,
        vec![
            (Ok(Token::RightParen), 0..1),
            (Ok(Token::LeftParen), 2..3),
            (Ok(Token::RightParen), 4..5),
            (Ok(Token::LeftParen), 5..6),
            (Ok(Token::LeftParen), 7..8),
            (Ok(Token::RightParen), 8..9),
            (Ok(Token::LeftParen), 10..11),
            (Ok(Token::RightParen), 12..13),
        ]
    )
}

#[test]
fn test_braces() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("} { }{ {} { }").spanned().collect();
    assert_eq!(
        lexed,
        vec![
            (Ok(Token::RightBrace), 0..1),
            (Ok(Token::LeftBrace), 2..3),
            (Ok(Token::RightBrace), 4..5),
            (Ok(Token::LeftBrace), 5..6),
            (Ok(Token::LeftBrace), 7..8),
            (Ok(Token::RightBrace), 8..9),
            (Ok(Token::LeftBrace), 10..11),
            (Ok(Token::RightBrace), 12..13),
        ]
    )
}

#[test]
fn test_pipe() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("| || |").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::Pipe), 0..1),
            (Ok(Token::Pipe), 2..3),
            (Ok(Token::Pipe), 3..4),
            (Ok(Token::Pipe), 5..6),
        ]
    )
}

#[test]
fn test_type_keyword() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("type").spanned().collect();

    assert_eq!(lexed, vec![(Ok(Token::Type), 0..4),]);
}

#[test]
fn test_paste_keyword() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("paste").spanned().collect();

    assert_eq!(lexed, vec![(Ok(Token::Paste), 0..5),]);
}

#[test]
fn test_colon() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer(":").spanned().collect();

    assert_eq!(lexed, vec![(Ok(Token::Colon), 0..1)]);
}

#[test]
fn test_identifier() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("hello $0 world").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::Identifier("hello".to_string())), 0..5),
            (Ok(Token::Identifier("$0".to_string())), 6..8),
            (Ok(Token::Identifier("world".to_string())), 9..14),
        ]
    );
}

#[test]
fn test_whitespace() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("hello\tworld\n").spanned().collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::Identifier("hello".to_string())), 0..5),
            (Ok(Token::Identifier("world".to_string())), 6..11),
        ]
    );
}

#[test]
fn test_pipestar() {
    let lexed: Vec<(Result<Token, ()>, Span)> =
        Token::lexer("|*").spanned().collect();

    assert_eq!(lexed, vec![(Ok(Token::PipeStar), 0..2)]);
}

#[test]
fn test_mixed_tokens() {
    let lexed: Vec<(Result<Token, ()>, Span)> = Token::lexer(
        "{type}(T)|-12.34 |* \"hello\" bruh type moment paste",
    )
    .spanned()
    .collect();

    assert_eq!(
        lexed,
        vec![
            (Ok(Token::LeftBrace), 0..1),
            (Ok(Token::Type), 1..5),
            (Ok(Token::RightBrace), 5..6),
            (Ok(Token::LeftParen), 6..7),
            (Ok(Token::BooleanLiteral(true)), 7..8),
            (Ok(Token::RightParen), 8..9),
            (Ok(Token::Pipe), 9..10),
            (Ok(Token::FloatLiteral(-12.34)), 10..16),
            (Ok(Token::PipeStar), 17..19),
            (Ok(Token::StringLiteral("hello".to_string())), 20..27),
            (Ok(Token::Identifier("bruh".to_string())), 28..32),
            (Ok(Token::Type), 33..37),
            (Ok(Token::Identifier("moment".to_string())), 38..44),
            (Ok(Token::Paste), 45..50)
        ]
    );
}