use super::invoke_runtime;
use super::Environment;
use super::Value;
use crate::interpreter::EvaluateResult;
use crate::parser::{ASTNode, LiteralVariant, PipeType};

pub struct Interpreter {
    pub root_node: ASTNode,
    pub env: Environment,
}

impl Interpreter {
    pub fn new(root_node: ASTNode) -> Interpreter {
        Interpreter {
            root_node,
            env: Environment::new(),
        }
    }

    pub fn bind_parameters(&mut self, parameters: Vec<Value>) {
        for (index, parameter) in parameters.iter().enumerate() {
            let parameter_id = format!("${}", index);
            self.env.bind(parameter_id, parameter.clone());
        }

        self.env
            .bind("$n".to_string(), Value::Integer(parameters.len() as i64));
    }

    pub fn evaluate_from_root(
        &mut self,
        parameters: Option<Vec<Value>>,
    ) -> EvaluateResult {
        self.env.push_stack_frame();
        let root = self.root_node.clone();
        if let Some(parameters) = parameters {
            self.bind_parameters(parameters)
        }
        let result = self.evaluate(&root);
        self.env
            .pop_stack_frame()
            .expect("stack corruption in eval from root");
        result
    }

    pub fn evaluate(&mut self, node: &ASTNode) -> EvaluateResult {
        match node {
            ASTNode::Literal(literal) => self.evaluate_literal(literal),
            ASTNode::Tuple(tuple) => self.evaluate_tuple(tuple),
            ASTNode::Identifier(id) => self.evaluate_identifier(id),
            ASTNode::Block(expressions) => self.evaluate_block(expressions),
            ASTNode::Binding((identifier, value)) => {
                self.evaluate_binding(identifier, value)
            }
            ASTNode::Pipe(expressions, pipe_types) => {
                self.evaluate_pipe(expressions, pipe_types)
            }
            _ => panic!("Unimplemented ASTNode variant"),
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

    fn evaluate_pipe(
        &mut self,
        expressions: &Vec<ASTNode>,
        pipe_types: &Vec<PipeType>,
    ) -> EvaluateResult {
        let mut curr_value = self.evaluate(&expressions[0])?;

        for (expr, pipe_type) in expressions[1..].iter().zip(pipe_types) {
            let closure = self.evaluate(expr)?;
            let transformed_input = match pipe_type {
                PipeType::Standard => vec![curr_value.clone()],
                PipeType::Destructure => {
                    if let Value::Tuple(values) = curr_value.clone() {
                        values
                    } else {
                        return Err(
                            "Trying to destructure non-tuple value".to_string()
                        );
                    }
                }
            };
            curr_value = match closure {
                Value::RuntimeInvocation => {
                    if transformed_input.len() != 2 {
                        return Err(
                            "Runtime invocation requires 2 arguments - parameters to runtime and runtime call"
                                .to_string(),
                        );
                    } else {
                        let runtime_parameters = &transformed_input[0];
                        let runtime_call = &transformed_input[1];
                        match (runtime_parameters, runtime_call) {
                            (Value::Tuple(parameters), Value::String(call)) => {
                                invoke_runtime(parameters.clone(), call.clone())?
                            },
                            _ => {
                                return Err(
                                    "Runtime parameters should be tuple and runtime call should be string"
                                        .to_string(),
                                )
                            }
                        }
                    }
                }
                _ => Self::execute_closure(transformed_input, &closure)?,
            };
        }

        Ok(curr_value)
    }

    pub fn execute_closure(
        parameters: Vec<Value>,
        closure: &Value,
    ) -> EvaluateResult {
        if let Value::Closure(c_exps, env_image) = closure {
            // this is hacky, but we'll actually just create a new interpreter
            // to execute the closure in with a dummy root node
            let mut new_interpreter = Interpreter::new(ASTNode::Block(vec![]));

            // the closure needs to execute in a new stack frame
            new_interpreter.env.push_stack_frame();

            // we'll then bind everything that it needs; conflicting identifiers
            // from the environment get shadowed.
            for (id, val) in env_image {
                new_interpreter.env.bind(id.clone(), val.clone());
            }

            // the closure needs to know how to recurse, so we'll bind it to rec
            new_interpreter.env.bind("rec".to_string(), closure.clone());

            // then we'll have to bind arguments in the $0, $1, ... $n fashion.
            new_interpreter.bind_parameters(parameters);

            // then we actually run the closure - the value that the last
            // statement evaluates to is the one that we return. note that empty
            // blocks just evaluate to the empty tuple.
            let mut last_value = Value::Tuple(vec![]);
            for expression in c_exps {
                last_value = new_interpreter.evaluate(expression)?;
            }

            // lastly, we'll have to pop off the stack frame
            new_interpreter.env.pop_stack_frame().expect("stack corruption");

            // and we're done
            Ok(last_value)
        } else {
            Err("Passed non-closure for evaluation".to_string())
        }
    }
}