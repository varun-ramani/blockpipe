use super::{EvaluateResult, Interpreter, Value};

pub fn invoke_runtime(parameters: Vec<Value>, call: String) -> EvaluateResult {
    match call.as_str() {
        "foo" => foo(parameters),
        "binop_arith" => binop_arith(parameters),
        "binop_cmp" => binop_cmp(parameters),
        "strcat" => strcat(parameters),
        "print" => print(parameters),
        "if" => if_runtime_call(parameters),
        _ => Err(format!("Unknown runtime call: {}", call)),
    }
}

fn foo(parameters: Vec<Value>) -> EvaluateResult {
    Ok(Value::String("bar".to_string()))
}

fn binop_arith(parameters: Vec<Value>) -> EvaluateResult {
    if parameters.len() != 3 {
        return Err("binop_arith requires 2 numbers and an operation".to_string());
    }

    let left = &parameters[0];
    let right = &parameters[1];
    let op = match &parameters[2] {
        Value::String(op) => op.as_str(),
        _ => return Err("Third parameter must be an operation string".to_string()),
    };

    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => {
            perform_arith_int(*left, *right, op).map(Value::Integer)
        }
        (Value::Float(left), Value::Float(right)) => {
            perform_arith_float(*left, *right, op).map(Value::Float)
        }
        (Value::Integer(left), Value::Float(right)) => {
            perform_arith_float(*left as f64, *right, op).map(Value::Float)
        }
        (Value::Float(left), Value::Integer(right)) => {
            perform_arith_float(*left, *right as f64, op).map(Value::Float)
        }
        _ => Err("binop_arith requires both operands to be numeric".to_string()),
    }
}

fn perform_arith_int(left: i64, right: i64, op: &str) -> Result<i64, String> {
    match op {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err(format!("Unknown arithmetic operation: {}", op)),
    }
}

fn perform_arith_float(left: f64, right: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err(format!("Unknown arithmetic operation: {}", op)),
    }
}



fn binop_cmp(parameters: Vec<Value>) -> EvaluateResult {
    if parameters.len() != 3 {
        return Err("binop_cmp requires 2 numbers and a comparison operation".to_string());
    }

    let left = &parameters[0];
    let right = &parameters[1];
    let op = match &parameters[2] {
        Value::String(op) => op.as_str(),
        _ => return Err("Third parameter must be a comparison operation string".to_string()),
    };

    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => 
            perform_cmp_int(*left, *right, op).map(Value::Boolean),
        (Value::Float(left), Value::Float(right)) => 
            perform_cmp_float(*left, *right, op).map(Value::Boolean),
        (Value::Integer(left), Value::Float(right)) => 
            perform_cmp_float(*left as f64, *right, op).map(Value::Boolean),
        (Value::Float(left), Value::Integer(right)) => 
            perform_cmp_float(*left, *right as f64, op).map(Value::Boolean),
        _ => Err("binop_cmp requires both operands to be numeric".to_string()),
    }
}

fn perform_cmp_int(left: i64, right: i64, op: &str) -> Result<bool, String> {
    match op {
        "<" => Ok(left < right),
        "<=" => Ok(left <= right),
        ">" => Ok(left > right),
        ">=" => Ok(left >= right),
        "==" => Ok(left == right),
        "!=" => Ok(left != right),
        _ => Err(format!("Unknown comparison operation: {}", op)),
    }
}

fn perform_cmp_float(left: f64, right: f64, op: &str) -> Result<bool, String> {
    match op {
        "<" => Ok(left < right),
        "<=" => Ok(left <= right),
        ">" => Ok(left > right),
        ">=" => Ok(left >= right),
        "==" => Ok(left == right),
        "!=" => Ok(left != right),
        _ => Err(format!("Unknown comparison operation: {}", op)),
    }
}


fn strcat(parameters: Vec<Value>) -> EvaluateResult {
    if parameters.len() != 2 {
        return Err("strcat requires 2 arguments".to_string());
    } else {
        let left = &parameters[0];
        let right = &parameters[1];
        match (left, right) {
            (Value::String(left), Value::String(right)) => {
                Ok(Value::String(format!("{}{}", left, right)))
            }
            _ => Err("strcat requires two strings".to_string()),
        }
    }
}

fn print(parameters: Vec<Value>) -> EvaluateResult {
    if parameters.len() != 1 {
        return Err("print requires 1 argument".to_string());
    } else {
        let value = &parameters[0];

        println!("{}", value);
        Ok(Value::Tuple(vec![]))
    }
}

fn if_runtime_call(parameters: Vec<Value>) -> EvaluateResult {
    if parameters.len() != 3 {
        return Err("if requires 3 arguments".to_string());
    } else {
        let condition = &parameters[0];
        let true_branch = &parameters[1];
        let false_branch = &parameters[2];

        // let's do a quick sanity check
        if let (Value::Boolean(_), Value::Closure(_, _), Value::Closure(_, _)) =
            (condition, true_branch, false_branch)
        {
            // then we can conditionally execute one of the closures
            match condition {
                Value::Boolean(true) => {
                    Interpreter::execute_closure(vec![], true_branch)
                }
                Value::Boolean(false) => {
                    Interpreter::execute_closure(vec![], false_branch)
                }
                _ => unreachable!(),
            }
        } else {
            return Err("if requires boolean and two closures".to_string());
        }
    }
}
