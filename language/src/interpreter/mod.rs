mod environment;
mod value;
mod interp;
#[cfg(test)]
mod tests;

use environment::*;
use value::*;
use interp::*;

/// results and errors of evaluation operations. these types do need to be refined.
pub type EvaluateResult = Result<Value, EvaluationError>;
pub type EvaluationError = String;