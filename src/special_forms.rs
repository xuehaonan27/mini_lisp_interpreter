use crate::builtins::list;
use crate::value::Value;
use crate::eval_env::EvalEnv;
use std::rc::Rc;
pub type SpecialForm = fn(Vec<Value>, Rc<EvalEnv>) -> Value;

pub fn define_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    if args.len() < 2 {
        panic!("SyntaxError: Missing parameter in form <define>.");
    }
    match args[0].clone() {
        Value::SymbolValue(s) => {
            // env.symbol_map的类型是RefCell<HashMap<String, Value>>, 检查其中是否有key键, 如果有那么向其中插入value值
            // 注意这里必须用temp_map保存一份原来哈希表的副本
            // 因为向env.symbol_map插入值, 首先不能把整个field所有权移动出来, 也不能用into_inner什么的消耗掉RefCell的所有权.
            // 所以必须用borrow_mut()方法获得RefCell里面哈希表的可变引用.
            // 但是这个可变引用不可以作为get的self参数, 因为self要求一个共享引用
            // 因此出此下策!
            /*let temp_env = env.clone();
            let mut ref_of_map = env.symbol_map.borrow_mut();
            if temp_env.symbol_map.borrow().contains_key(&s) {
                let borrow = temp_env.symbol_map.borrow();
                let value: Option<&Value> = borrow.get(&args[1].to_string());
                if value.is_some() {
                    _ = ref_of_map.insert(s, value.unwrap().clone());
                }
                else {
                    _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                }
            }
            else {
                _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
            }*/

            let temp_env = env.clone();
            if temp_env.symbol_map.borrow().contains_key(&s) {
                let borrow = temp_env.symbol_map.borrow();
                let value: Option<&Value> = borrow.get(&args[1].to_string());
                if value.is_some() {
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value.unwrap().clone());
                    println!("{:?}", ref_of_map);
                }
                else {
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                    println!("{:?}", ref_of_map);
                }
            }
            else {
                let mut ref_of_map = env.symbol_map.borrow_mut();
                _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                println!("{:?}", ref_of_map);
            }
        },
        Value::PairValue(car, cdr) => {
            match *car {
                Value::SymbolValue(s) => {
                    let mut lambda_args: Vec<Value> = vec![*cdr];
                    lambda_args.append(&mut args[1..].to_vec());
                    let temp_env = env.clone();
                    _ = env.symbol_map.borrow_mut().insert(s, lambda_form(lambda_args, temp_env));
                    println!("{:?}", env.symbol_map);
                },
                _ => panic!("SyntaxError: Malformed define."),
            }
        },
        _ => panic!("SyntaxError: Malformed define."),
    }
    Value::NilValue
}
pub fn quote_form(args: Vec<Value>, _env: Rc<EvalEnv>) -> Value {
    if args.len() < 1 {
        panic!("SyntaxError: Missing parameter in form <quote>.");
    }
    else {
        args[0].clone()
    }
}
pub fn if_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    let result = env.eval(args[0].clone());
    if result.is_ok() {
        match result.unwrap() {
            Value::BooleanValue(false) => return env.eval(args[2].clone()).expect("Corruption when evaluating a value in form <if>"),
            Value::BooleanValue(true) => return env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <if>"),
            _ => return env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <if>"),
        }
    }
    else {
        env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <if>")
    }
}
pub fn and_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    if args.is_empty() {
        return Value::BooleanValue(true);
    }
    for arg in args.clone() {
        let result = env.eval(arg.clone());
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
    let result = env.eval(args[args.len() - 1].clone());
    if result.is_ok() {
        result.unwrap()
    }
    else {
        Value::NilValue
    }
}
pub fn or_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    if args.is_empty() {
        return Value::BooleanValue(false);
    }
    for arg in args.clone() {
        let result = env.eval(arg.clone());
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
pub fn lambda_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    let vec: Vec<Value> = args[0].to_vector().expect("Corruption when converting a value to vector in form <lambda>.");
    let mut params: Vec<String> = Vec::new();
    vec.iter().for_each(|value| params.push(value.to_string()));
    let body: Vec<Value> = args[1..].to_vec();
    match args[0] {
        // Value::PairValue(_, _) => return Value::LambdaValue(Box::new(params), Box::new(body), env.clone()),
        // _ => return Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), env.clone()),
        Value::PairValue(_, _) => return Value::LambdaValue(Box::new(params), Box::new(body), Rc::clone(&env)),
        _ => return Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), Rc::clone(&env)),
    }
}
pub fn cond_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    for (index, arg) in args.iter().enumerate() {
        match arg {
            Value::PairValue(_, _) => {
                let arg_vec: Vec<Value> = arg.to_vector().expect("Corruption when converting a value to vector in form <cond>.");
                let flag = env.eval(arg_vec[0].clone()).expect("Corruption when evaluating a value in form <cond>.");
                match flag {
                    Value::BooleanValue(false) => continue,
                    Value::SymbolValue(s) if s == "else".to_string() => {
                        if index == args.len() - 1 {
                            let mut result_vec:Vec<Value> = Vec::new(); 
                            arg_vec.iter().for_each(|arg_v| 
                                result_vec.push(env.eval(arg_v.clone()).expect("Corruption when evaluating a value in form <cond>."))
                            );
                            return result_vec.pop().expect("SyntaxError: Missing executing part of <else> clause.");
                        }
                        else {
                            panic!("SyntaxError: \"else\" must be at the condition position in the last clause of form <cond>.");
                        }
                    },
                    _ => {
                        let mut result_vec:Vec<Value> = Vec::new(); 
                        arg_vec.iter().for_each(|arg_v| 
                            result_vec.push(env.eval(arg_v.clone()).expect("Corruption when evaluating a value in form <cond>."))
                        );
                        return result_vec.pop().expect("SyntaxError: Missing executing part of a clause.");
                    },
                }
            },
            _ => panic!("SyntaxError: missing parameter in form <cond>."),
        }
    }
    Value::NilValue
}
pub fn begin_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    if args.is_empty() {
        panic!("SyntaxError: missing parameter in form <begin>.");
    }
    let mut result: Value = Value::NilValue;
    for arg in args {
        result = env.eval(arg).expect("Corruption when evaluating a value in form <begin>");
    }
    result
}
pub fn let_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    let mut params1: Vec<String> = Vec::new();
    let mut params2: Vec<Value> = Vec::new();
    let bindings: Vec<Value>;
    match args[0] {
        Value::PairValue(_, _) => bindings = args[0].to_vector().expect("Corruption when converting a value to a vector in form <let>."),
        _ => panic!("SyntaxError: temporary bindings without parentheses: \n (let ((#<binding>)(...)) (#<procedure>)(..) \n      ^                 ^"),
    }
    for binding in bindings {
        match binding {
            Value::PairValue(_, _) => {
                let binding_vec: Vec<Value> = binding.to_vector().expect("Corruption when converting a value to vector in form <let>.");
                if binding_vec.len() == 2 {
                    params1.push(binding_vec[0].to_string());
                    params2.push(env.eval(binding_vec[1].clone()).expect("Corruption when evaluating a value in form <let>."));
                }
                else {
                    panic!("SyntaxError: temporary binding should be a 2-element list.");
                }
            },
            _ => panic!("SyntaxError: temporary binding should be a 2-element list."),
        }
    }
    let mut results: Vec<Value> = Vec::new();
    let env_derived: EvalEnv = env.derive(params1, params2);
    /*for (index, arg) in args.iter().enumerate() {
        if index == 0 {
            continue;
        }
        results.push(env_derived.eval(arg.clone()).expect("Corruption when evaluating a value in form <let>"));
    }*/
    args[1..].iter().for_each(|arg| results.push(env_derived.eval(arg.clone()).expect("Corruption when evaluating a value in form <let>")));
    results.pop().unwrap_or(Value::NilValue)
}
pub fn quasiquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    let mut results: Vec<Value> = Vec::new();
    let arg_vec: Vec<Value> = args[0].to_vector().expect("Corruption when converting a value to vector in form <quasiquote>.");
    for arg in arg_vec {
        match arg.clone() {
            Value::PairValue(car, cdr) => {
                match *car {
                    Value::SymbolValue(s) if s == "unquote".to_string() => results.push(unquote_form(cdr.to_vector().expect("Corruption when converting a value to vector in form <quasiquote>."), env.clone())), // clone here!
                    Value::SymbolValue(s) if s == "quasiquote".to_string() => panic!("Calling quasiquote inside quasiquote is an undefined behavior."),
                    _ => results.push(arg), 
                }
            },
            _ => results.push(arg),
        }
    }
    list(results, env)
}
pub fn unquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Value {
    if args.len() < 1 {
        panic!("SyntaxError: Missing argument in form <unquote>.");
    }
    else if args.len() > 1{
        panic!("SyntaxError: Too many argument in form <unquote>.");
    }
    else {
        env.eval(args[0].clone()).expect("Corruption when evaluating a value in form <unquote>.")
    }
}