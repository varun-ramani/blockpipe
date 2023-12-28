use logos::Span;

use super::*;
use crate::lexer::*;
use crate::parser::*;

fn lex_unconditionally(input: &str) -> Vec<(Token, Span)> {
    lex(input)
        .into_iter()
        .map(|(tok, span)| (tok.unwrap(), span))
        .collect()
}

fn lex_and_parse(input: &str) -> Result<ASTNode, ParserError> {
    Parser::new(lex_unconditionally(input)).parse()
}

fn lex_parse_evaluate(input: &str) -> EvaluateResult {
    let mut interpreter = Interpreter::new(lex_and_parse(input).unwrap());
    interpreter.evaluate(&interpreter.root_node.clone())
}

#[test]
fn test_evaluate_integer_literal() {
    assert_eq!(lex_parse_evaluate("1"), Ok(Value::Integer(1)));
    assert_eq!(lex_parse_evaluate("-1"), Ok(Value::Integer(-1)));
}

#[test]
fn test_evaluate_boolean_literal() {
    assert_eq!(lex_parse_evaluate("T"), Ok(Value::Boolean(true)));
    assert_eq!(lex_parse_evaluate("F"), Ok(Value::Boolean(false)));
}

#[test]
fn test_evaluate_string_literal() {
    assert_eq!(
        lex_parse_evaluate("\"hello world\""),
        Ok(Value::String(String::from("hello world")))
    );
}

#[test]
fn test_evaluate_float_literal() {
    assert_eq!(lex_parse_evaluate("1.0"), Ok(Value::Float(1.0)));
    assert_eq!(lex_parse_evaluate("-1.0"), Ok(Value::Float(-1.0)));
}

#[test]
fn test_evaluate_tuple() {
    assert_eq!(
        lex_parse_evaluate("(1 2 3)"),
        Ok(Value::Tuple(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3)
        ]))
    );
    assert_eq!(
        lex_parse_evaluate("(T F 1 2.0 \"hello world\")"),
        Ok(Value::Tuple(vec![
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Integer(1),
            Value::Float(2.0),
            Value::String(String::from("hello world"))
        ]))
    );
}

#[test]
fn test_evaluate_identifiers() {
    let code = r#"
        (a b c)
    "#;

    let ast = lex_and_parse(code).unwrap();

    let mut interpreter = Interpreter::new(ast);

    interpreter.env.push_stack_frame();
    interpreter.env.bind("a".to_string(), Value::Boolean(true));
    interpreter.env.bind("b".to_string(), Value::Integer(1));
    interpreter
        .env
        .bind("c".to_string(), Value::String("hello".to_string()));
    assert_eq!(
        interpreter.evaluate(&interpreter.root_node.clone()),
        Ok(Value::Tuple(vec![
            Value::Boolean(true),
            Value::Integer(1),
            Value::String("hello".to_string())
        ]))
    );

    interpreter.env.bind("a".to_string(), Value::Integer(3));
    assert_eq!(
        interpreter.evaluate(&interpreter.root_node.clone()),
        Ok(Value::Tuple(vec![
            Value::Integer(3),
            Value::Integer(1),
            Value::String("hello".to_string())
        ]))
    );

    interpreter.env.push_stack_frame();
    assert_eq!(
        interpreter.evaluate(&interpreter.root_node.clone()),
        Ok(Value::Tuple(vec![
            Value::Integer(3),
            Value::Integer(1),
            Value::String("hello".to_string())
        ]))
    );

    interpreter.env.bind("b".to_string(), Value::Boolean(true));
    interpreter.env.bind("b".to_string(), Value::Boolean(false));
    interpreter.env.bind("a".to_string(), Value::Boolean(true));
    interpreter.env.bind("c".to_string(), Value::Integer(3));
    interpreter.env.bind("a".to_string(), Value::Integer(1));
    interpreter.env.bind("b".to_string(), Value::Integer(2));
    assert_eq!(
        interpreter.evaluate(&interpreter.root_node.clone()),
        Ok(Value::Tuple(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3)
        ]))
    );

    assert_eq!(interpreter.env.pop_stack_frame().ok(), Some(()));
    assert_eq!(
        interpreter.evaluate(&interpreter.root_node.clone()),
        Ok(Value::Tuple(vec![
            Value::Integer(3),
            Value::Integer(1),
            Value::String("hello".to_string())
        ]))
    );
}

#[test]
fn test_evaluate_binding() {
    let mut interpreter =
        Interpreter::new(lex_and_parse("a: \"bruh\"").unwrap());
    interpreter.env.push_stack_frame();
    let root_node = interpreter.root_node.clone();
    interpreter.evaluate(&root_node);
    let image = interpreter.env.image();

    assert_eq!(image.len(), 1);
    assert_eq!(image.get("a").unwrap(), &Value::String("bruh".to_string()));
}

#[test]
fn test_evaluate_closure() {
    let code = r#"
        {
            a: $0
            b: $1
            (a b)
        }
    "#;

    let ast = lex_and_parse(code).unwrap();
    let mut closure_interpreter = Interpreter::new(ast);
    closure_interpreter.env.push_stack_frame();
    let root_node = closure_interpreter.root_node.clone();
    let closure = closure_interpreter.evaluate(&root_node).unwrap();
    let final_value = closure_interpreter.execute_closure(
        vec![Value::Integer(1), Value::Boolean(true)],
        &closure,
    );
    assert_eq!(
        final_value,
        Ok(Value::Tuple(vec![Value::Integer(1), Value::Boolean(true)]))
    );
}

#[test]
fn test_evaluate_pipe() {
    let code = r#"
        (0 1) | { $0 } |* { ($0 $1 $0 $1) } | { $0 }
    "#;

    let mut interpreter = Interpreter::new(lex_and_parse(code).unwrap());
    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::Tuple(vec![
            Value::Integer(0),
            Value::Integer(1),
            Value::Integer(0),
            Value::Integer(1)
        ]))
    );
}

#[test]
fn test_basic_blockpipe() {
    let code = r#"
        (
            main: {
                swap: {
                    ($1 $0)
                }

                a: 1
                b: 2

                combined_args: (a b)

                do_something: {
                    $0 |* $1
                }

                ((a b) swap) |* do_something |* {$0} | {($0 "bruh")}
            }

            () | main
        )
    "#;

    let mut interpreter = Interpreter::new(lex_and_parse(code).unwrap());
    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::Tuple(vec![
            Value::Tuple(vec![]),
            Value::Tuple(vec![
                Value::Integer(2),
                Value::String("bruh".to_string())
            ])
        ]))
    );
}