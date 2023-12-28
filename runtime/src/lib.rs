mod interpreter_runtime;

#[no_mangle]
pub extern "C" fn add_integers(left: i64, right: i64) -> i64 {
    left + right
}

pub extern "C" fn add_floats(left: f64, right: f64) -> f64 {
    left + right
}