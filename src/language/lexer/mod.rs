use logos::{Logos, Lexer};

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

#[derive(Logos, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_integer() {
        let mut lex = Token::lexer("1 12 -1 -12");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::IntegerLiteral(1)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::IntegerLiteral(12)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::IntegerLiteral(-1)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::IntegerLiteral(-12)))
        );
    }

    #[test]
    fn test_float() {
        let mut lex = Token::lexer("1.0 12.0 -1.0 -12.0");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::FloatLiteral(1.0)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::FloatLiteral(12.0)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::FloatLiteral(-1.0)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::FloatLiteral(-12.0)))
        );
    }

    #[test]
    fn test_string() {
        let mut lex = Token::lexer(r#""hello world""#);
        assert_eq!(
            lex.next(),
            Some(Ok(Token::StringLiteral("hello world".to_string())))
        );
    }

    #[test]
    fn test_boolean() {
        let mut lex = Token::lexer("T F");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::BooleanLiteral(true)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::BooleanLiteral(false)))
        );
    }

    #[test]
    fn test_parentheses() {
        let mut lex = Token::lexer("() ( )");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftParen))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightParen))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftParen))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightParen))
        );
    }

    #[test]
    fn test_braces() {
        let mut lex = Token::lexer("{} { }");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftBrace))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightBrace))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftBrace))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightBrace))
        );
    }

    #[test]
    fn test_pipe() {
        let mut lex = Token::lexer("|");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Pipe))
        );
    }

    #[test]
    fn test_type() {
        let mut lex = Token::lexer("type");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Type))
        );
    }

    #[test]
    fn test_paste() {
        let mut lex = Token::lexer("paste");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Paste))
        );
    }

    #[test]
    fn test_all() {
        let mut lex = Token::lexer("(){}|\"hello world\"T F 1 1.0 type paste");
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftParen))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightParen))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::LeftBrace))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::RightBrace))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Pipe))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::StringLiteral("hello world".to_string())))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::BooleanLiteral(true)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::BooleanLiteral(false)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::IntegerLiteral(1)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::FloatLiteral(1.0)))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Type))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Paste))
        );
    }
}
