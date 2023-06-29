use crate::value::Value;
use crate::eval_env::EvalEnv;

pub type SpecialForm = fn(Vec<Value>, &EvalEnv) -> Value;

pub fn define_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    if args.len() < 2 {
        panic!("SyntaxError: Missing parameter in form <define>.");
    }
    match args[0].clone() {
        Value::SymbolValue(s) => {
            if env.symbol_map.contains_key(&s) {
                _ = env.symbol_map.insert(s, env.symbol_map.get(&args[1].to_string()).unwrap().clone());
            }
            else {
                _ = env.symbol_map.insert(s, env.eval(args[1].clone()));
            }
        },
        Value::PairValue(car, cdr) => {
            match *car {
                Value::SymbolValue(s) => {
                    let mut lambda_args: Vec<Value> = vec![*cdr];
                    lambda_args.append(&mut args[1..].to_vec());
                    _ = env.symbol_map.insert(s, lambda_form(lambda_args, env));
                },
                _ => panic!("SyntaxError: Malformed define."),
            }
        },
        _ => panic!("SyntaxError: Malformed define."),
    }
    todo!();
}
pub fn quote_form(args: Vec<Value>, _env: &EvalEnv) -> Value {
    if args.len() < 1 {
        panic!("SyntaxError: Missing parameter in form <quote>.");
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
pub fn lambda_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    let vec: Vec<Value> = args[0].to_vector();
    let mut params: Vec<String> = Vec::new();
    vec.iter().for_each(|value| params.push(value.to_string()));
    let body: Vec<Value> = args[1..].to_vec();
    match args[0] {
        Value::PairValue(_, _) => return Value::LambdaValue(Box::new(params), Box::new(body), env.clone()),
        _ => return Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), env.clone()),
    }
}
pub fn cond_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    todo!();
}
pub fn begin_form(args: Vec<Value>, env: &EvalEnv) -> Value {
    if args.is_empty() {
        panic!("SyntaxError: missing parameter in form <begin>.");
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