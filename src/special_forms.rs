use crate::builtins::list;
use crate::value::Value;
use crate::eval_env::EvalEnv;
use std::error::Error;
use std::rc::Rc;
use crate::error::ErrorEval;
pub type SpecialForm = fn(Vec<Value>, Rc<EvalEnv>) -> Result<Value, ErrorEval>;

pub fn define_form(args: Vec<Value>, mut env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 2 {
        // panic!("SyntaxError: Missing parameter in form <define>.");
        return Err(ErrorEval {
            message: format!("{}: Special Form <define>: Missing parameter", 0),
            index: 0
        });
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

            /*let temp_env = env.clone();
            
            if temp_env.symbol_map.borrow().contains_key(&s) {
                let borrow = temp_env.symbol_map.borrow();
                let value: Option<&Value> = borrow.get(&args[1].to_string());
                if value.is_some() {
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value.unwrap().clone());
                    println!("{:?}", ref_of_map);
                }
                else {
                    let value_to_be_inserted = temp_env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>.");
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value_to_be_inserted);
                    // _ = ref_of_map.insert(s, temp_env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                    println!("{:?}", ref_of_map);
                }
            }
            else {
                println!("Define entering here.");
                let value_to_be_inserted = temp_env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>.");
                let mut ref_of_map = env.symbol_map.borrow_mut();
                _ = ref_of_map.insert(s, value_to_be_inserted);
                // _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                println!("{:?}", ref_of_map);
            }*/
            
            if env.symbol_map.borrow().contains_key(&s) {
                let borrow = env.symbol_map.borrow();
                let value: Option<&Value> = borrow.get(&args[1].to_string());
                /*let value_done = value.unwrap().clone();
                // 其实检查发现, 这三行就可以替换下面的那段代码
                // 都已经contains_key了, 肯定能unwrap成功的.
                let mut ref_of_map = env.symbol_map.borrow_mut();
                _ = ref_of_map.insert(s, value.unwrap().clone());
                println!("{:?}", ref_of_map);*/
                if value.is_some() {
                    let value_to_be_inserted = value.unwrap().clone();
                    std::mem::drop(borrow);
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value_to_be_inserted);
                    // println!("{:?}", ref_of_map);
                }
                else {
                    // let value_to_be_inserted = env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>.");
                    let value_to_be_inserted = env.clone().eval(args[1].clone()).map_err(|error| ErrorEval{
                        message: format!("{}: Special Form <define>: Fail to evaluate a value\n{}", error.index + 1 ,error.message),
                        index: error.index + 1
                    })?;
                    std::mem::drop(borrow);
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value_to_be_inserted);
                    // _ = ref_of_map.insert(s, temp_env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                    // println!("{:?}", ref_of_map);
                }
            }
            else {
                // println!("Define entering here.");
                //let value_to_be_inserted = env.clone().eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>.");
                let value_to_be_inserted = env.clone().eval(args[1].clone()).map_err(|error| ErrorEval{
                    message: format!("{}: Special Form <define>: Fail to evaluate a value\n{}", error.index + 1 ,error.message),
                    index: error.index + 1
                })?;
                let mut ref_of_map = env.symbol_map.borrow_mut();
                _ = ref_of_map.insert(s, value_to_be_inserted);
                // _ = ref_of_map.insert(s, temp_env.eval(args[1].clone()).expect("Corruption when evaluating a value in form <define>."));
                // println!("{:?}", ref_of_map);
            }
        },
        Value::PairValue(car, cdr) => {
            match *car {
                Value::SymbolValue(s) => {
                    let mut lambda_args: Vec<Value> = vec![*cdr];
                    lambda_args.append(&mut args[1..].to_vec());
                    let temp_env = env.clone();
                    _ = env.symbol_map.borrow_mut().insert(s, lambda_form(lambda_args, temp_env)?);
                    // println!("{:?}", env.symbol_map);
                },
                // _ => panic!("SyntaxError: Malformed define."),
                _ => return Err(ErrorEval { message: format!("{}: Special Form <define>: Malformed define", 0), index: 0 }),
            }
        },
        // _ => panic!("SyntaxError: Malformed define."),
        _ => return Err(ErrorEval { message: format!("{}: Special Form <define>: Malformed define", 0), index: 0 })
    }
    Ok(Value::NilValue)
}
pub fn quote_form(args: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 1 {
        // panic!("SyntaxError: Missing parameter in form <quote>.");
        Err(ErrorEval{message: format!("{}: Special Form <quote>: Missing parameter", 0), index: 0})
    }
    else {
        Ok(args[0].clone())
    }
}
pub fn if_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let result = env.clone().eval(args[0].clone());
    if result.is_ok() {
        match result.unwrap() {
            Value::BooleanValue(false) => return env.eval(args[2].clone()).map_err(|error|ErrorEval {
                message: format!("{}: Special Form <if>: Fail to evaluate the false branch\n{}", error.index + 1, error.message),
                index: error.index + 1
            }),
            Value::BooleanValue(true) => return env.eval(args[1].clone()).map_err(|error| ErrorEval { 
                message: format!("{}: Special Form <if>: Fail to evaluate the true branch\n{}", error.index + 1, error.message),
                index: error.index + 1 
            }),
            _ => return env.clone().eval(args[1].clone()).map_err(|error| ErrorEval {
                message: format!("{}: Special Form <if>: Fail to evaluate the true branch\n{}", error.index + 1, error.message),
                index: error.index + 1
            }),
        }
    }
    else {
        env.eval(args[1].clone()).map_err(|error| ErrorEval {
            message: format!("{}: Special Form <if>: Fail to evaluate the condition\n{}", error.index + 1, error.message),
            index: error.index + 1
        })
    }
}
pub fn and_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.is_empty() {
        return Ok(Value::BooleanValue(true));
    }
    for arg in args.clone() {
        let result = env.clone().eval(arg.clone());
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => return Ok(Value::BooleanValue(false)),
                Value::BooleanValue(true) => continue,
                _ => continue,
            }
        }
        else {
            continue;
        }
    }
    /*let result = env.eval(args[args.len() - 1].clone());
    if result.is_ok() {
        Ok(result.unwrap())
    }
    else {
        Ok(Value::NilValue)
    }*/
    env.eval(args[args.len() - 1].clone()).or(Ok(Value::NilValue))
}
pub fn or_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.is_empty() {
        return Ok(Value::BooleanValue(false));
    }
    for arg in args.clone() {
        let result = env.clone().eval(arg.clone());
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => continue,
                v @ Value::BooleanValue(true) => return Ok(v),
                v @ _ => return Ok(v),
            }
        }
        else {
            return Ok(Value::NilValue);
        }
    }
    Ok(Value::BooleanValue(false))
}
pub fn lambda_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 2{
        return Err(ErrorEval{message: format!("{}: Special Form <lambda>: Missing part of lambda expression", 0), index: 0});
    }
    let vec: Vec<Value> = args[0].to_vector().map_err(|error| ErrorEval {
        message: format!("{}: Special Form <lambda>: Fail to convert value to vector\n{}", error.index + 1, error.message),
        index: error.index + 1
    })?;
    //.expect("Corruption when converting a value to vector in form <lambda>.");
    let mut params: Vec<String> = Vec::new();
    let temp_arg = args[0].clone();
    vec.iter().for_each(|value| params.push(value.to_string()));
    let body: Vec<Value> = args.into_iter().skip(1).filter(|bodyv| 
        match bodyv {
            Value::NilValue => false,
            _ => true,
        }
    ).collect();
    match temp_arg {
        // Value::PairValue(_, _) => return Value::LambdaValue(Box::new(params), Box::new(body), env.clone()),
        // _ => return Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), env.clone()),
        Value::PairValue(_, _) => return Ok(Value::LambdaValue(Box::new(params), Box::new(body), Rc::clone(&env))),
        _ => return Ok(Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), Rc::clone(&env))),
    }
}
pub fn cond_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    for (index, arg) in args.iter().enumerate() {
        match arg {
            Value::PairValue(_, _) => {
                let arg_vec: Vec<Value> = arg.to_vector().map_err(|error| ErrorEval {
                    message: format!("{}: Special Form <cond>: Fail to convert value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                let flag = env.clone().eval(arg_vec[0].clone()).map_err(|error| ErrorEval {
                    message: format!("{}: Special Form <cond>: Fail to evaluate condition\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                match flag {
                    Value::BooleanValue(false) => continue,
                    Value::SymbolValue(s) if s == "else".to_string() => {
                        if index == args.len() - 1 {
                            let mut result_vec:Vec<Value> = Vec::new(); 
                            arg_vec.iter().for_each(|arg_v| 
                                result_vec.push(env.clone().eval(arg_v.clone()).expect("Corruption when evaluating a value in form <cond>."))
                            );
                            return result_vec.pop().ok_or(ErrorEval{
                                message: format!("{}: Special Form <cond>: Fail to pop a value", 0),
                                index: 0
                            });
                        }
                        else {
                            return Err(ErrorEval{
                                message: format!("{}: Special Form <cond>: \"else\" must be at the condition position in the last clause", 0),
                                index: 0,
                            });
                        }
                    },
                    _ => {
                        let mut result_vec:Vec<Value> = Vec::new(); 
                        arg_vec.iter().for_each(|arg_v| 
                            result_vec.push(env.clone().eval(arg_v.clone()).expect("Corruption when evaluating a value in form <cond>."))
                        );
                        // return result_vec.pop().expect("SyntaxError: Missing executing part of a clause.");
                        return result_vec.pop().ok_or(ErrorEval{
                            message: format!("{}: Special Form <cond>: Missing executing part of a clause", 0),
                            index: 0
                        });
                    },
                }
            },
            _ => return Err(ErrorEval {
                message: format!("{}: Special Form <cond>: Missing parameter", 0),
                index: 0
            }),
        }
    }
    Ok(Value::NilValue)
}
pub fn begin_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.is_empty() {
        // panic!("SyntaxError: missing parameter in form <begin>.");
        return Err(ErrorEval {
            message: format!("{}: Special Form <begin>: Missing parameter", 0),
            index: 0
        });
    }
    let mut result: Value = Value::NilValue;
    for arg in args {
        // result = env.clone().eval(arg).expect("Corruption when evaluating a value in form <begin>");
        result = env.clone().eval(arg).map_err(|error| ErrorEval{
            message: format!("{}: Special Form <begin>: Missing parameter\n{}", error.index + 1, error.message),
            index: error.index + 1
        })?
    }
    Ok(result)
}
pub fn let_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut params1: Vec<String> = Vec::new();
    let mut params2: Vec<Value> = Vec::new();
    let bindings: Vec<Value>;
    match args[0] {
        Value::PairValue(_, _) => bindings = args[0].to_vector().map_err(|error| ErrorEval {
            message: format!("{}: Special Form <let>: Fail to convert value to vector\n{}", error.index + 1, error.message),
            index: error.index + 1
        })?,
        // _ => panic!("SyntaxError: temporary bindings without parentheses: \n (let ((#<binding>)(...)) (#<procedure>)(..) \n      ^                 ^"),
        _ => return Err(ErrorEval{
            message: format!("temporary bindings without parentheses: \n (let ((#<binding>)(...)) (#<procedure>)(..) \n      ^                 ^"),
            index: 0
        }),
    }
    for binding in bindings {
        match binding {
            Value::PairValue(_, _) => {
                let binding_vec: Vec<Value> = binding.to_vector().map_err(|error| ErrorEval {
                    message: format!("{}: Special Form <let>: Fail to convert value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                if binding_vec.len() == 2 {
                    params1.push(binding_vec[0].to_string());
                    params2.push(env.clone().eval(binding_vec[1].clone()).map_err(|error| ErrorEval {
                        message: format!("{}: Special Form <let>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                        index: error.index + 1
                    })?);
                }
                else {
                    // panic!("SyntaxError: temporary binding should be a 2-element list.");
                    return Err(ErrorEval{
                        message: format!("{}: Special Form <let>: temporary binding should be a 2-element list", 0),
                        index: 0
                    });
                }
            },
            // _ => panic!("SyntaxError: temporary binding should be a 2-element list."),
            _ => return Err(ErrorEval{
                message: format!("{}: Special Form <let>: temporary binding should be a 2-element list", 0),
                index: 0
            }),
        }
    }
    let mut results: Vec<Value> = Vec::new();
    let env_derived: Rc<EvalEnv> = env.derive(params1, params2).into();
    /*for (index, arg) in args.iter().enumerate() {
        if index == 0 {
            continue;
        }
        results.push(env_derived.eval(arg.clone()).expect("Corruption when evaluating a value in form <let>"));
    }*/
    args[1..].iter().for_each(|arg| results.push(env_derived.clone().eval(arg.clone()).expect("Corruption when evaluating a value in form <let>")));
    Ok(results.pop().unwrap_or(Value::NilValue))
}
pub fn quasiquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut results: Vec<Value> = Vec::new();
    // let arg_vec: Vec<Value> = args[0].to_vector().expect("Corruption when converting a value to vector in form <quasiquote>.");
    let arg_vec: Vec<Value> = args[0].to_vector().map_err(|error| ErrorEval{
        message: format!("{}: Special Form <quasiquote>: Fail to evaluate a value\n{}", error.index + 1, error.message),
        index: error.index + 1
    })?;
    for arg in arg_vec {
        match arg.clone() {
            Value::PairValue(car, cdr) => {
                match *car {
                    // Value::SymbolValue(s) if s == "unquote".to_string() => results.push(unquote_form(cdr.to_vector().expect("Corruption when converting a value to vector in form <quasiquote>."), env)), // clone here!
                    Value::SymbolValue(s) if s == "unquote".to_string() => {
                        results.push(unquote_form(cdr.to_vector().map_err(|error| ErrorEval{
                            message: format!("{}: Special Form <quasiquote>: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?, env.clone())?);
                    },
                    // Value::SymbolValue(s) if s == "quasiquote".to_string() => panic!("Calling quasiquote inside quasiquote is an undefined behavior."),
                    Value::SymbolValue(s) if s == "quasiquote".to_string() => return Err(ErrorEval {
                        message: format!("{}: Special Form <quasiquote>: Calling quasiquote inside quasiquote is an undefined behavior", 0),
                        index: 0
                    }),
                    _ => results.push(arg), 
                }
            },
            _ => results.push(arg),
        }
    }
    list(results, env).map_err(|error| ErrorEval {
        message: format!("{}: Special Form <quasiquote>: Fail to pack the result\n{}", error.index + 1, error.message),
        index: error.index + 1
    })
}
pub fn unquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 1 {
        // panic!("SyntaxError: Missing argument in form <unquote>.");
        return Err(ErrorEval{
            message: format!("{}: Special Form <unquote>: Missing argument", 0),
            index: 0
        });
    }
    else if args.len() > 1{
        // panic!("SyntaxError: Too many argument in form <unquote>.");
        return Err(ErrorEval {
            message: format!("{}: Special Form <unquote>: Too many argument", 0),
            index: 0
        });
    }
    else {
        // env.eval(args[0].clone()).expect("Corruption when evaluating a value in form <unquote>.")
        env.eval(args[0].clone()).map_err(|error| ErrorEval {
            message: format!("{}: Special Form <let>: Fail to convert value to vector\n{}", error.index + 1, error.message),
            index: error.index + 1
        })
    }
}