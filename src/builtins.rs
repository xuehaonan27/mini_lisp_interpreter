use std::collections::HashMap;
use crate::value::BuiltinFn;
use crate::value::Value;
use crate::eval_env::EvalEnv;
/*static BUILTIN_PROCS: HashMap<String, BuiltinFn> = HashMap::from([
    ("apply".to_string(), apply as BuiltinFn),
]);*/

pub fn apply(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn print(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn display(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn error(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn eval(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn exit(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn newline(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn atom_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn boolean_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn integer_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn list_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn number_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn null_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn pair_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn procedure_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn string_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn symbol_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }

fn append(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn car(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn cdr(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn cons(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn length(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn list(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn map(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn loosemap(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn strictmap(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn filter(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn reduce(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }

fn add(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn subtract(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn multiply(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn divide(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn abs(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn expt(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn quotient(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn modulo(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn remainder(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }

fn eq_q(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn equal_q(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn equal_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn less_than_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn more_than_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn less_than_or_equal_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn more_than_or_equal_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn even_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn odd_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn zero_or_not(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
fn sort(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
