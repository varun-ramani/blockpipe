use core::fmt;

use logos::{Lexer, Logos, Span};

// TODO: unescape string literals
fn load_string(lex: &mut Lexer<Token>) -> String {
    lex.slice()[1..lex.slice().len() - 1].to_string()
}

fn load_bool(lex: &mut Lexer<Token>) -> bool {
    match lex.slice() {
        "T" => true,
        "F" => false,
        _ => unreachable!(),
    }
}

fn load_integer(lex: &mut Lexer<Token>) -> i64 {
    lex.slice().parse().unwrap()
}

fn load_float(lex: &mut Lexer<Token>) -> f64 {
    lex.slice().parse().unwrap()
}

fn load_identifier(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r" |\t|\n")]
pub enum Token {
    // starting off with parentheses
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    // then the braces
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    // then the pipe operator
    #[token("|")]
    Pipe,
    #[token("|*")]
    PipeStar,

    // then the colon
    #[token(":")]
    Colon,

    // then identifiers
    #[regex(r#"\$(?:\d+|n)|[a-z|_][a-zA-Z0-9_]*"#, load_identifier)]
    Identifier(String),

    // then the literals
    #[regex(r#""([^"\\]|\\.)*""#, load_string)]
    StringLiteral(String),
    #[regex(r#"T|F"#, load_bool)]
    BooleanLiteral(bool),
    #[regex(r#"-?[0-9]+"#, load_integer)]
    IntegerLiteral(i64),
    #[regex(r#"-?[0-9]+\.[0-9]+"#, load_float)]
    FloatLiteral(f64),

    // then type and paste
    #[token("type")]
    Type,
    #[token("paste")]
    Paste,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Pipe => write!(f, "|"),
            Token::Colon => write!(f, ":"),
            Token::Identifier(s) => write!(f, "IDENTIFIER<{}>", s),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::BooleanLiteral(b) => {
                write!(f, "{}", if *b { "T" } else { "F" })
            }
            Token::IntegerLiteral(i) => write!(f, "{}", i),
            Token::FloatLiteral(fl) => write!(f, "{}", fl),
            Token::Type => write!(f, "type"),
            Token::Paste => write!(f, "paste"),
            Token::PipeStar => write!(f, "|*"),
        }
    }
}

pub fn lex(input: &str) -> Vec<(Result<Token, ()>, Span)> {
    Token::lexer(input)
        .spanned()
        // .map(|(tok, span)| (tok.unwrap(), span))
        .collect()
}