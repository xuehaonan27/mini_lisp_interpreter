use crate::value::Value;
use crate::eval_env::EvalEnv;

pub type SpecialForm = fn(Vec<Value>, &EvalEnv) -> Value;

pub fn define_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn quote_form(args: Vec<Value>, _env: &EvalEnv) -> Value {
    if args.len() < 1 {
        panic!("Missing parameter in quote form.");
    }
    else {
        args[0].clone()
    }
}
pub fn if_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    let result = std::panic::catch_unwind(||
        env.eval(args[0].clone())
    );
    if result.is_ok() {
        match result.unwrap() {
            Value::BooleanValue(false) => return env.eval(args[2].clone()),
            Value::BooleanValue(true) => return env.eval(args[1].clone()),
            _ => return env.eval(args[1].clone()),
        }
    }
    else {
        env.eval(args[1].clone())
    }
}
pub fn and_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    if args.is_empty() {
        return Value::BooleanValue(true);
    }
    for arg in args.clone() {
        let result = std::panic::catch_unwind(||
            env.eval(arg.clone())
        );
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => return Value::BooleanValue(false),
                Value::BooleanValue(true) => continue,
                _ => continue,
            }
        }
        else {
            continue;
        }
    }
    let result = std::panic::catch_unwind(||
        env.eval(args[args.len() - 1].clone())
    );
    if result.is_ok() {
        result.unwrap()
    }
    else {
        Value::NilValue
    }
}
pub fn or_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    if args.is_empty() {
        return Value::BooleanValue(false);
    }
    for arg in args.clone() {
        let result = std::panic::catch_unwind(||
            env.eval(arg.clone())
        );
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => continue,
                v @ Value::BooleanValue(true) => return v,
                v @ _ => return v,
            }
        }
        else {
            return Value::NilValue;
        }
    }
    Value::BooleanValue(false)
}
pub fn lambda_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn cond_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    todo!();
}
pub fn begin_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    if args.is_empty() {
        panic!("SyntaxError: missing parameter.");
    }
    let mut result: Value = Value::NilValue;
    for arg in args {
        result = env.eval(arg);
    }
    result
}
pub fn let_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn quasiquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn unquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }