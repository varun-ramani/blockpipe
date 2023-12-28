use crate::language::parser::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
enum Value {
    // primitive values
    Integer(i64),
    Boolean(bool),
    String(String),
    Float(f64),

    // tuples
    Tuple(Vec<Value>),

    // closure
    Closure(Vec<ASTNode>, HashMap<String, Value>),
}

/// results and errors of evaluation operations. these types do need to be refined.
type EvaluateResult = Result<Value, EvaluationError>;
type EvaluationError = String;

/// the runtime stack
#[derive(Debug)]
struct Environment {
    stack_frames: Vec<HashSet<String>>,
    keys: HashMap<String, Vec<Value>>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            stack_frames: Vec::new(),
            keys: HashMap::new(),
        }
    }

    /// invoked at the start of a block
    fn push_stack_frame(&mut self) {
        self.stack_frames.push(HashSet::new());
    }

    fn pop_stack_frame(&mut self) -> Result<(), ()> {
        let last_frame = self.stack_frames.pop().ok_or(())?;

        for key in last_frame {
            self.keys
                .get_mut(&key)
                .expect("stack corruption: pop stack frame")
                .pop();

            if self
                .keys
                .get_mut(&key)
                .expect("stack corruption: pop stack frame")
                .is_empty()
            {
                self.keys.remove(&key);
            }
        }

        Ok(())
    }

    fn bind(&mut self, key: String, value: Value) {
        // if the current key is already in the stack frame, then we'll remove it
        if self
            .stack_frames
            .last()
            .expect("stack corruption: bind")
            .contains(&key)
        {
            self.keys
                .get_mut(&key)
                .expect("stack corruption: bind")
                .pop();
        }
        // otherwise, we'll just go ahead and insert the current key to the stack frame
        else {
            self.stack_frames
                .last_mut()
                .expect("stack corruption: bind")
                .insert(key.clone());
        }

        // and then we'll unconditionally insert the key
        self.keys.entry(key).or_insert(Vec::new()).push(value);
    }

    fn lookup(&self, key: &str) -> Option<Value> {
        Some(self.keys.get(key)?.last()?.clone())
    }

    /// grab the most recent set of bindings in the environment
    fn image(&self) -> HashMap<String, Value> {
        self.keys
            .iter()
            .map(|(key, value)| {
                let cloned_key = key.clone();
                let cloned_value = value
                    .last()
                    .expect("binding without associated values")
                    .clone();

                (cloned_key, cloned_value)
            })
            .collect()
    }
}

struct Interpreter {
    root_node: ASTNode,
    env: Environment,
}

impl Interpreter {
    fn new(root_node: ASTNode) -> Interpreter {
        Interpreter {
            root_node,
            env: Environment::new(),
        }
    }

    fn evaluate_from_root(&mut self) -> EvaluateResult {
        self.env.push_stack_frame();
        let root = self.root_node.clone();
        let result = self.evaluate(&root);
        self.env
            .pop_stack_frame()
            .expect("stack corruption in eval from root");
        result
    }

    fn evaluate(&mut self, node: &ASTNode) -> EvaluateResult {
        match node {
            ASTNode::Literal(literal) => self.evaluate_literal(literal),
            ASTNode::Tuple(tuple) => self.evaluate_tuple(tuple),
            ASTNode::Identifier(id) => self.evaluate_identifier(id),
            ASTNode::Block(expressions) => self.evaluate_block(expressions),
            ASTNode::Binding((identifier, value)) => {
                self.evaluate_binding(identifier, value)
            }
            _ => Err(String::from("not implemented")),
        }
    }

    fn evaluate_literal(&self, literal: &LiteralVariant) -> EvaluateResult {
        match literal {
            LiteralVariant::IntegerLiteral(i) => Ok(Value::Integer(*i)),
            LiteralVariant::BooleanLiteral(b) => Ok(Value::Boolean(*b)),
            LiteralVariant::StringLiteral(s) => Ok(Value::String(s.clone())),
            LiteralVariant::FloatLiteral(f) => Ok(Value::Float(*f)),
        }
    }

    fn evaluate_tuple(&mut self, tuple: &Vec<ASTNode>) -> EvaluateResult {
        let mut values = Vec::new();
        for node in tuple {
            values.push(self.evaluate(node)?);
        }
        Ok(Value::Tuple(values))
    }

    fn evaluate_identifier(&self, identifier: &String) -> EvaluateResult {
        self.env
            .lookup(identifier)
            .ok_or(format!("Unbound symbol '{}'", identifier))
    }

    fn evaluate_block(&self, expressions: &Vec<ASTNode>) -> EvaluateResult {
        let env_image = self.env.image();
        Ok(Value::Closure(expressions.clone(), env_image))
    }

    fn evaluate_binding(
        &mut self,
        identifier: &String,
        value: &Box<ASTNode>,
    ) -> EvaluateResult {
        let expr_value = self.evaluate(value)?;
        self.env.bind(identifier.clone(), expr_value);
        Ok(Value::Tuple(vec![]))
    }

    fn execute_closure(
        &mut self,
        parameters: Vec<Value>,
        closure: &Value,
    ) -> EvaluateResult {
        if let Value::Closure(c_exps, env_image) = closure {
            // the closure needs to execute in a new stack frame
            self.env.push_stack_frame();

            // we'll then bind everything that it needs; conflicting identifiers
            // from the environment get shadowed.
            for (id, val) in env_image {
                self.env.bind(id.clone(), val.clone());
            }

            // then we'll have to bind arguments in the $0, $1, ... $n fashion.
            for (index, parameter) in parameters.iter().enumerate() {
                let parameter_id = format!("${}", index);
                self.env.bind(parameter_id, parameter.clone());
            }

            // then we actually run the closure - the value that the last
            // statement evaluates to is the one that we return. note that empty
            // blocks just evaluate to the empty tuple.
            let mut last_value = Value::Tuple(vec![]);
            for expression in c_exps {
                last_value = self.evaluate(expression)?;
            }

            // lastly, we'll have to pop off the stack frame
            self.env.pop_stack_frame().expect("stack corruption");

            // and we're done
            Ok(last_value)
        } else {
            Err("Passed non-closure for evaluation".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Span;

    use super::*;
    use crate::language::lexer::*;
    use crate::language::parser::*;

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
}
