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
    let final_value = Interpreter::execute_closure(
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

fn interpreter_with_runtime(code: &str) -> Interpreter {
    let mut interpreter = Interpreter::new(lex_and_parse(code).unwrap());
    interpreter.env.push_stack_frame();
    interpreter
        .env
        .bind("plz".to_string(), Value::RuntimeInvocation);
    interpreter
}

#[test]
fn test_runtime_foo() {
    let code = r#"
        (() "foo") |* plz
    "#;

    let mut interpreter = interpreter_with_runtime(code);

    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::String("bar".to_string()))
    );
}

#[test]
fn test_binop_arith() {
    let mut interpreter = interpreter_with_runtime(
        r#"
        () | {
            add: {
                (($0 $1 "+") "binop_arith") |* plz
            }

            sub: {
                (($0 $1 "-") "binop_arith") |* plz
            }

            mul: {
                (($0 $1 "*") "binop_arith") |* plz
            }

            div: {
                (($0 $1 "/") "binop_arith") |* plz
            }

            data: (1.0 2.0)

            (
                data |* add
                data |* sub
                data |* mul
                data |* div
            )
        }
    "#,
    );

    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::Tuple(vec![
            Value::Float(3.0),
            Value::Float(-1.0),
            Value::Float(2.0),
            Value::Float(0.5)
        ]))
    );
}

#[test]
fn test_binop_cmp() {
    let mut interpreter = interpreter_with_runtime(
        r#"
        () | {
            less_than: {
                (($0 $1 "<") "binop_cmp") |* plz
            }

            less_than_equal: {
                (($0 $1 "<=") "binop_cmp") |* plz
            }

            greater_than: {
                (($0 $1 ">") "binop_cmp") |* plz
            }

            greater_than_equal: {
                (($0 $1 ">=") "binop_cmp") |* plz
            }

            equal: {
                (($0 $1 "==") "binop_cmp") |* plz
            }

            not_equal: {
                (($0 $1 "!=") "binop_cmp") |* plz
            }

            data_int: (2 3)
            data_float: (2.0 3.0)
            data_mixed: (2 3.0)

            (
                data_int |* less_than
                data_int |* less_than_equal
                data_int |* greater_than
                data_int |* greater_than_equal
                data_int |* equal
                data_int |* not_equal
                data_float |* less_than
                data_float |* less_than_equal
                data_float |* greater_than
                data_float |* greater_than_equal
                data_float |* equal
                data_float |* not_equal
                data_mixed |* less_than
                data_mixed |* less_than_equal
                data_mixed |* greater_than
                data_mixed |* greater_than_equal
                data_mixed |* equal
                data_mixed |* not_equal
            )
        }
    "#,
    );

    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::Tuple(vec![
            // Comparisons with integers (2, 3)
            Value::Boolean(true),   // 2 < 3
            Value::Boolean(true),   // 2 <= 3
            Value::Boolean(false),  // 2 > 3
            Value::Boolean(false),  // 2 >= 3
            Value::Boolean(false),  // 2 == 3
            Value::Boolean(true),   // 2 != 3

            // Comparisons with floats (2.0, 3.0)
            Value::Boolean(true),   // 2.0 < 3.0
            Value::Boolean(true),   // 2.0 <= 3.0
            Value::Boolean(false),  // 2.0 > 3.0
            Value::Boolean(false),  // 2.0 >= 3.0
            Value::Boolean(false),  // 2.0 == 3.0
            Value::Boolean(true),   // 2.0 != 3.0

            // Heterogeneous comparisons (2, 3.0)
            Value::Boolean(true),   // 2 < 3.0
            Value::Boolean(true),   // 2 <= 3.0
            Value::Boolean(false),  // 2 > 3.0
            Value::Boolean(false),  // 2 >= 3.0
            Value::Boolean(false),  // 2 == 3.0
            Value::Boolean(true)    // 2 != 3.0
        ]))
    );
}


#[test]
fn test_strcat() {
    let mut interpreter = interpreter_with_runtime(
        r#"
        () | {
            data: ("hello " "world")

            data |* { (($0 $1) "strcat") } |* plz
        }
    "#,
    );

    assert_eq!(
        interpreter.evaluate_from_root(None),
        Ok(Value::String("hello world".to_string()))
    );
}

#[test]
fn test_if() {
    let mut interpreter = interpreter_with_runtime(
        r#"
        () | {
            if: {
                (($0 $1 $2) "if") |* plz
            }

            (T { "all good!" } { "oof something ain't right" }) |* if
        }
    "#,
    );

    assert_eq!(interpreter.evaluate_from_root(None), Ok(Value::String("all good!".to_string())));
}

#[test]
fn test_recursive_if() {
    let mut interpreter = interpreter_with_runtime(
        r#"
        () | {
            if: {
                (($0 $1 $2) "if") |* plz
            }

            leq: {
                (($0 $1 "<=") "binop_cmp") |* plz
            }

            sub: {
                (($0 $1 "-") "binop_arith") |* plz
            }

            mul: {
                (($0 $1 "*") "binop_arith") |* plz
            }

            factorial: {
                x: $0
                fact_rec: rec
                (
                    (x 1) |* leq
                    {1}
                    {   
                        (x 1)  |* sub | fact_rec | {(x $0)} |* mul
                    }
                ) |* if
            }

            5 | factorial
        }
    "#,
    );

    println!("{:?}", interpreter.evaluate_from_root(None));
}
