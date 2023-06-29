use crate::value::Value;
use crate::eval_env::EvalEnv;

pub type SpecialForm = fn(Vec<Value>, &EvalEnv) -> Value;
/*static SPECIAL_FORMS: HashMap<String, SpecialForm> = HashMap::from([
    ("define".to_string(), define_form as SpecialForm),
    ("quote".to_string(), quote_form as SpecialForm),
    ("if".to_string(), if_form as SpecialForm),
    ("and".to_string(), and_form as SpecialForm),
    ("or".to_string(), or_form as SpecialForm),
    ("lambda".to_string(), lambda_form as SpecialForm),
    ("cond".to_string(), cond_form as SpecialForm),
    ("begin".to_string(), begin_form as SpecialForm),
    ("let".to_string(), let_form as SpecialForm),
    ("quasiquote".to_string(), quasiquote_form as SpecialForm),
    ("unquote".to_string(), unquote_form as SpecialForm),
]);*/

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
pub fn cond_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn begin_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn let_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn quasiquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn unquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }