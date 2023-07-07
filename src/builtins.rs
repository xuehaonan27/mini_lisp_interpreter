/// 定义了所有内置过程

use crate::value::{Value, is_integer};
use crate::eval_env::EvalEnv;
use std::process;
use std::panic;
use std::rc::Rc;
use crate::error::ErrorEval;

pub fn apply(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2{
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <apply>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 2 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <apply>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::ProcedureValue(f) => {
                // let args: Vec<Value> = params[1..].iter().cloned().map(|value| env.eval(value)).collect();
                let args: Vec<Value> = params[1].to_vector().map_err(|error| ErrorEval{
                    message: format!("{}: Builtin Procedure <apply>: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                return f(args, env);
            },
            Value::LambdaValue(params_in_lambda, body, env) => {
                let env_derived: Rc<EvalEnv> = env.derive(*params_in_lambda, params[1].to_vector().map_err(|error| ErrorEval{
                    message: format!("{}: Builtin Procedure <apply>: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?).into();
                let mut result: Value = Value::NilValue;
                for bodyv in *body {
                    result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                        message: format!("{}: Builtin Procedure <apply>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                        index: error.index + 1
                    })?
                }
                return Ok(result);
            },
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <apply>: Fail to evaluate a value", 0), index: 0 }),
        }
    }
}
pub fn print(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    params.iter().for_each(|param| println!("{}", param.to_string()));
    Ok(Value::NilValue)
}
pub fn display(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <display>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <display>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::StringValue(s) => {
                print!("{}", s);
            },
            v => {
                print!("{}", v.to_string());
            }
        }
        Ok(Value::NilValue)
    }
}
pub fn displayln(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <displayln>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <displayln>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::StringValue(s) => {
                println!("{}", s);
            },
            v => {
                println!("{}", v.to_string());
            }
        }
        Ok(Value::NilValue)
    }
}
pub fn error(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <error>: Too many argument", 0), index: 0 });
    }
    else if params.len() == 1 {
        panic!("{}", params[0].to_string());
    }
    else {
        panic!("Error thrown.");
    }
}
pub fn eval(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <eval>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <eval>: Too many argument", 0), index: 0 });
    }
    else {
        env.eval(params[0].clone()).map_err(|error| ErrorEval{
            message: format!("{}: Builtin Procedure <eval>: Fail to evaluate a value\n{}", error.index + 1, error.message),
            index: error.index + 1
        })
    }
}
/// 非安全退出. 
/// 并不保证能够顺利退出. 
/// 当exit调用格式不对时会panic而非exit.
pub fn exit(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.is_empty() {
        process::exit(0);
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <exit>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => process::exit(n as i32),
            // _ => panic!("SyntaxError: Non integer exit code is forbidden"),
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <exit>: Non integer exit code is forbidden", 0), index: 0 }),
        }
    }
}
/// 强制安全退出. 
/// 当出现exit_force调用格式不对时会以127退出码退出.
pub fn exit_force(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.is_empty() {
        process::exit(0);
    }
    else if params.len() > 1 {
        eprint!("SyntaxError: Too many argument in procedure <exit>");
        process::exit(127);
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => process::exit(n as i32),
            _ => { 
                eprint!("SyntaxError: Non integer exit code is forbidden");
                process::exit(127);
            }
        }
    }
}
pub fn newline(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.is_empty() {
        println!();
        Ok(Value::NilValue)
    }
    else {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <newline>: Cannot append argument", 0), index: 0 });
    }
}

pub fn atom_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <atom?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <atom?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::BooleanValue(_) => return Ok(Value::BooleanValue(true)),
            Value::NumericValue(_) => return Ok(Value::BooleanValue(true)),
            Value::StringValue(_) => return Ok(Value::BooleanValue(true)),
            Value::SymbolValue(_) => return Ok(Value::BooleanValue(true)),
            Value::NilValue => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn boolean_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <boolean?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <boolean?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::BooleanValue(_) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn integer_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <integer?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <integer?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Ok(Value::BooleanValue(true));
                }
                else {
                    return Ok(Value::BooleanValue(false));
                }
            },
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn list_or_not(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval>{
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <list?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <list?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::NilValue => return Ok(Value::BooleanValue(true)),
            Value::PairValue(_, cdr) => return list_or_not(vec![*cdr], env).map_err(|error| ErrorEval {
                message: format!("{}: Builtin Procedure <list?>: Recursivly finding error...\n{}", error.index + 1, error.message),
                index: error.index + 1
            }),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn number_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <number?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <number?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::NumericValue(_) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn null_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <null?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <null?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::NilValue => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn pair_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <pair?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <pair?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::PairValue(_, _) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn procedure_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <procedure?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <procedure?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::ProcedureValue(_) => return Ok(Value::BooleanValue(true)),
            Value::LambdaValue(_, _, _) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn string_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <string?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <string?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::StringValue(_) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn symbol_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <symbol?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <symbol?>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0] {
            Value::SymbolValue(_) => return Ok(Value::BooleanValue(true)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
/// 自己拓展的功能
/// 检查某个符号是否已经在当前环境绑定
pub fn defined_local_or_not(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <defined_local?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <defined_local?>: Too many argument", 0), index: 0 });
    }
    else {
        if env.symbol_map.borrow().contains_key(&params[0].to_string()) {
            return Ok(Value::BooleanValue(true));
        }
        else {
            return Ok(Value::BooleanValue(false));
        }
    }
}
/// 自己拓展的功能
/// 检查某个符号是否已经在所有可见环境内绑定
pub fn defined_all_or_not(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <defined_all?>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <defined_all?>: Too many argument", 0), index: 0 });
    }
    else {
        let bind = env.find_binding(&params[0].to_string());
        if bind.is_some() {
            return Ok(Value::BooleanValue(true));
        }
        else {
            return Ok(Value::BooleanValue(false));
        }
    }
}
/// ( append list1 ... ) 内置过程
/// 将 list 内的元素按顺序拼接为一个新的列表. 
/// 返回值:拼接后的列表
/// 实参个数为零时返回空表。
pub fn append(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut ret: Vec<Value> = Vec::new();
    for param in params {
        match param {
            Value::NilValue => (),
            Value::PairValue(_, _) => {
                // 注意这里可能逻辑实现有错误, 如果发生错误请立刻改正为忠实翻译
                let result = param.to_vector();
                if result.is_ok() {
                    ret.append(result.unwrap().as_mut());
                }
                else {
                    return Err(ErrorEval { message: format!("{}: Builtin Procedure <append>: Cannot append a procedure value", 0), index: 0 });
                }
            },
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <append>: Cannot append a procedure value", 0), index: 0 }),
        }
    }
    list(ret, env).map_err(|error| ErrorEval {
        message: format!("{}: Builtin Procedure <append>: Fail to pack the result\n{}", error.index + 1, error.message),
        index: error.index + 1
    })
}
/// ( push list value ) 自定义过程
/// 将 value 加入到 list 末尾
/// value 只可以是原子类型.
/// value 是空表的时候将不进行任何操作
/// value 是过程类型与lambda类型时将报错
pub fn push(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut ret: Vec<Value> = Vec::new();
    for param in params {
        match param {
            Value::NilValue => (),
            Value::PairValue(_, _) => {
                // 注意这里可能逻辑实现有错误, 如果发生错误请立刻改正为忠实翻译
                let result = param.to_vector();
                if result.is_ok() {
                    ret.append(result.unwrap().as_mut());
                }
                else {
                    // panic!("Cannot append a procedure value.");
                    return Err(ErrorEval { message: format!("{}: Builtin Procedure <push>: Cannot append a procedure value", 0), index: 0 });
                }
            },
            Value::BooleanValue(_) => ret.push(param),
            Value::NumericValue(_) => ret.push(param),
            Value::StringValue(_) => ret.push(param),
            Value::SymbolValue(_) => ret.push(param),
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <push>: Cannot append a procedure value", 0), index: 0 }),
        }
    }
    list(ret, env).map_err(|error| ErrorEval {
        message: format!("{}: Builtin Procedure <push>: Fail to pack the result\n{}", error.index + 1, error.message),
        index: error.index + 1
    })
}
pub fn car(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <car>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <car>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::PairValue(car, _) => return Ok(*car),
            // _ => panic!("Cannot get car of a non-pair/list type value."),
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <length>: Cannot get car of a non-pair/list type value", 0), index: 0 })
        }
    }
}
pub fn cdr(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <cdr>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <cdr>: Too many argument", 0), index: 0 });
    }
    else {
        match params[0].clone() {
            Value::PairValue(_, cdr) => return Ok(*cdr),
            // _ => panic!("Cannot get car of a non-pair/list type value."),
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <length>: Cannot get cdr of a non-pair/list type value", 0), index: 0 })
        }
    }
}
pub fn cons(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <cons>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 2 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <cons>: Too many argument", 0), index: 0 });
    }
    else {
        Ok(Value::PairValue(Box::new(params[0].clone()), Box::new(params[1].clone())))
    }
}
pub fn length(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <length>: Missing argument", 0), index: 0 });
    }
    else if params.len() > 1 {
        return Err(ErrorEval { message: format!("{}: Builtin Procedure <length>: Too many argument", 0), index: 0 });
    } 
    else {
        match params[0] {
            Value::PairValue(_, _) => {
                let vec: Vec<Value> = params[0].to_vector().map_err(|error| ErrorEval{
                    message: format!("{}: Builtin Procedure <length>: Missing argument\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                if vec.len() == 1  {
                    match vec[0] {
                        Value::NilValue => return Ok(Value::NumericValue(0f64)),
                        _ => {},
                    }
                }
                return Ok(Value::NumericValue(vec.len() as f64));
            },
            _ => {
                // panic!("TypeError. Cannot get length of a non-list value.");
                return Err(ErrorEval { message: format!("{}: Builtin Procedure <length>: Cannot get length of a non-list value", 0), index: 0 })
            },
        }
    }
}
pub fn list(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.is_empty() {
        Ok(Value::NilValue)
    }
    else {
        Ok(Value::PairValue(Box::new(params[0].clone()), Box::new(list(params[1..].to_vec(), env)?)))
    }
}
pub fn map(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map>: Too many argument", 0), index: 0});
    }
    else {
        let args = params[1].to_vector();
        if args.is_ok() {
            let mut results: Vec<Value> = Vec::new();
            match params[0].clone() {
                Value::ProcedureValue(f) => {
                    args.unwrap().iter().clone().try_for_each(|arg| -> Result<(), ErrorEval> {
                        let arg: Value = f(vec![arg.clone()], Rc::clone(&env)).map_err(|error| ErrorEval{
                            message: format!("{}: Builtin Procedure <map>: Fail to call the given procedure\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?;
                        results.push(arg);
                        Ok(())
                    })?;
                    return list(results, Rc::clone(&env));
                }
                Value::LambdaValue(params, body, env_in_lambda) => {
                    args.unwrap().iter().clone().try_for_each(|arg| -> Result<(), ErrorEval>{
                        let arg: Value = {
                            let args_in_lambda = vec![arg.clone()];
                            let env_derived: Rc<EvalEnv> = env_in_lambda.clone().derive(*params.clone(), args_in_lambda).into();
                            let mut result: Value = Value::NilValue;
                            for bodyv in *body.clone() {
                                result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                    message: format!("{}: Builtin Procedure <map>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                    index: error.index + 1
                                })?;
                            }
                            result
                        };
                        results.push(arg);
                        Ok(())
                    })?;
                    return list(results, env);

                    /*results = args.unwrap().iter().cloned().map(|arg| -> Value {
                        let args_in_lambda = vec![arg.clone()];
                        let env_derived: Rc<EvalEnv> = env_in_lambda.clone().derive(*params.clone(), args_in_lambda).into();
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                message: format!("{}: Builtin Procedure <map>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })?;
                        }
                        arg
                    }).collect();
                    return list(results, env);*/
                },
                _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map_expand>: Need a procedure", 0), index: 0}),
            }
        }
        else {
            return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map_expand>: Cannot map a non-list value", 0), index: 0});
        }
    }
}
pub fn map_expand(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map_expand>: Missing argument", 0), index: 0});
    }
    let mut size: Option<usize> = None;
    let mut vecs: Vec<Vec<Value>> = Vec::new();
    let mut results: Vec<Value> = Vec::new();
    params[1..].iter().try_for_each(|param|->Result<(), ErrorEval> {
        match param {
            Value::PairValue(_, _) => {
                // let vec = param.to_vector().expect("Corruption when converting a value to vector in procedure <map_expand>.");
                let vec=  param.to_vector().map_err(|error| ErrorEval{
                    message: format!("{}: Builtin Procedure <map_expand>: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                if size.is_none() { size = Some(vec.len()); vecs.push(vec); Ok(())}
                else if size != Some(vec.len()) {
                    // panic!("Error size in procedure <map_expand>: lists should have the same size.");
                    return Err(ErrorEval { message: format!("{}: Builtin Procedure <map_expand>: Lists should have the same size", 0), index: 0 })
                }
                else { vecs.push(vec); Ok(())}
            },
            // _ => panic!("Error type in procedure <map_expand>: need a procedure."),
            _ => return Err(ErrorEval { message: format!("{}: Builtin Procedure <map_expand>: Need a procedure", 0), index: 0 }),
        }
    })?;
    for i in 0..size.unwrap() {
        let mut temp_args: Vec<Value> = Vec::new();
        vecs.iter().try_for_each(|vec| -> Result<(), ErrorEval> {
            let arg: Value = env.clone().eval(vec[i].clone()).map_err(|error| ErrorEval{
                message: format!("{}: Builtin Procedure <map_expand>: Need a procedure\n{}", error.index + 1, error.message),
                index: error.index + 1,
            })?;
            temp_args.push(arg);
            Ok(())
        })?;
        match params[0].clone() {
            Value::ProcedureValue(f) => {
                let result = f(temp_args, Rc::clone(&env)).map_err(|error| ErrorEval{
                    message: format!("{}: Builtin Procedure <map_expand>: Fail to call the given procedure\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                results.push(result);
            },
            Value::LambdaValue(params_in_lambda, body, env_in_lambda) => {
                let env_derived: Rc<EvalEnv> = env_in_lambda.derive(*params_in_lambda, temp_args).into();
                let mut result: Value = Value::NilValue;
                for bodyv in *body {
                    result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                        message: format!("{}: Builtin Procedure <map_expand>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                        index: error.index + 1
                    })?;
                }
                results.push(result);
            },
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <map_expand>: Need a procedure", 0), index: 0}),
        }
    }
    list(results, env).map_err(|error| ErrorEval{
        message: format!("{}: Builtin Procedure <map_expand>: Fail to pack the result\n{}", error.index + 1, error.message),
        index: error.index + 1
    })
}
pub fn filter(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <filter>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <filter>: Too many argument", 0), index: 0});
    }
    else {
        let args = params[1].to_vector();
        if args.is_ok() {
            let mut results: Vec<Value> = Vec::new();
            match params[0].clone() {
                Value::ProcedureValue(f) => {
                    for arg in args.unwrap() {
                        let result: Value = f(vec![arg.clone()], Rc::clone(&env)).map_err(|error| ErrorEval{
                            message: format!("{}: Builtin Procedure <filter>: Fail to call the given procedure\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?;
                        match result {
                            Value::BooleanValue(false) => {},
                            _ => results.push(arg.clone()),
                        }
                    }
                    return list(results, env);
                }
                /*Value::LambdaValue(_, _) => {
                    todo!();
                },*/
                Value::LambdaValue(params, body, env_in_lambda) => {
                    for arg in args.unwrap() {
                        let args_in_lambda = vec![arg];
                        let env_derived: Rc<EvalEnv> = env_in_lambda.clone().derive(*params.clone(), args_in_lambda).into();
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                message: format!("{}: Builtin Procedure <filter>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })?;
                        }
                        match result {
                            Value::BooleanValue(false) => continue,
                            _ => results.push(result),
                        }
                    }
                    return list(results, env);
                },
                _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <reduce>: Need a procedure and a list", 0), index: 0}),
            }
        }
        else {
            return Err(ErrorEval{ message: format!("{}: Builtin Procedure <reduce>: Need a procedure and a list", 0), index: 0});
        }
    }
}
pub fn reduce(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <reduce>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <reduce>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::ProcedureValue(f), Value::PairValue(car, cdr)) => {
                match *cdr {
                    Value::NilValue => return Ok(*car),
                    _ => {
                        let args: Vec<Value> = vec![*car, reduce(vec![params[0].clone(), *cdr], Rc::clone(&env)).map_err(|error| ErrorEval{
                            message: format!("{}: Builtin Procedure <reduce>: Recursive finding error...\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?];
                        return f(args, Rc::clone(&env));
                    },
                }
            },
            (Value::LambdaValue(params_in_lambda, body, env_in_lambda), Value::PairValue(car, cdr)) => {
                match *cdr {
                    Value::NilValue => return Ok(*car),
                    _ => {
                        let args: Vec<Value> = vec![*car, reduce(vec![params[0].clone(), *cdr], env).map_err(|error| ErrorEval{
                            message: format!("{}: Builtin Procedure <reduce>: Recursivly finding error...\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?];
                        let env_derived: Rc<EvalEnv> = env_in_lambda.derive(*params_in_lambda, args).into();
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                message: format!("{}: Builtin Procedure <reduce>: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })? //.expect("Corruption when evaluating a value in procedure <reduce>");
                        }
                        return Ok(result);
                    }
                }
            },
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <reduce>: need a procedure and a list", 0), index: 0}),
        }
    }
}

pub fn add(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut result: f64 = 0f64;
    for param in params {
        match param {
            Value::NumericValue(n) => result += n,
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'-'>: Cannot add a non-numeric value", 0), index: 0}),
        }
    }
    Ok(Value::NumericValue(result))
}
pub fn subtract(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'-'>: Missing argument", 0), index: 0});
    }
    else if params.len() == 1 {
        match params[0].clone() {
            Value::NumericValue(n) => return Ok(Value::NumericValue(-n)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'-'>: Cannot subtract a non-numeric value", 0), index: 0}),
        }
    }
    else if params.len() == 2 {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n1), Value::NumericValue(n2)) => return Ok(Value::NumericValue(n1 - n2)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'-'>: Cannot do subtraction with a non-numeric value", 0), index: 0}),
        }
    }
    else {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'-'>: Too many argument", 0), index: 0});
    }
}
pub fn multiply(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut ret: f64 = 1f64;
    for param in params {
        match param {
            Value::NumericValue(n) => ret *= n,
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'*'>: Cannot multiply non-numeric values", 0), index: 0}),
        }
    }
    Ok(Value::NumericValue(ret))
}
pub fn divide(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'/'>: Missing argument", 0), index: 0});
    }
    else if params.len() == 1 {
        match params[0].clone() {
            Value::NumericValue(n) if n != 0f64 => return Ok(Value::NumericValue(1f64 / n)),
            Value::NumericValue(n) if n == 0f64 => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'/'>: Division by zero", 0), index: 0}),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'/'>: Cannot divide a non-numeric value", 0), index: 0}),
        }
    }
    else if params.len() == 2 {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n1), Value::NumericValue(n2)) => return Ok(Value::NumericValue(n1 - n2)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'/'>: Cannot divide a non-numeric value", 0), index: 0}),
        }
    }
    else {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <divide>: Too many argument", 0), index: 0});
    }
}
pub fn abs(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <abs>: Missing argument", 0), index: 0});
    }
    else if params.len() > 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <abs>: Too many argument", 0), index: 0});
    }
    else {
        match params[0] {
            Value::NumericValue(n) => return Ok(Value::NumericValue(n.abs())),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <abs>: Cannot do abs with non-numeric value", 0), index: 0}),
        }
    }
}
pub fn expt(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <expt>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <expt>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(base), Value::NumericValue(expo)) if base != 0f64 && expo != 0f64 => return Ok(Value::NumericValue(base.powf(expo))),
            (Value::NumericValue(base), Value::NumericValue(expo)) if base == 0f64 && expo == 0f64 => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <expt>: Cannot calculate 0^0", 0), index: 0}),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <expt>: Cannot do quotient with non-numeric values", 0), index: 0}),
            
        }
    }
}
pub fn quotient(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <quotient>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <quotient>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => return Ok(Value::NumericValue((n0 / n1) as i64 as f64)),
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <quotient>: Division by zero", 0), index: 0}),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <quotient>: Cannot do quotient with non-numeric values", 0), index: 0}),
            
        }
    }
}
pub fn modulo(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <modulo>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <modulo>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => {
                if is_integer(&n0) && is_integer(&n1) {
                    let ans: f64 = n0 % n1;
                    if ans == 0f64 || n1 * ans > 0f64 {
                        return Ok(Value::NumericValue(ans));
                    }
                    else {
                        return Ok(Value::NumericValue(ans + n1));
                    }
                }
                else {
                    return Err(ErrorEval{ message: format!("{}: Builtin Procedure <modulo>: Cannot do modulo with non-integer values", 0), index: 0});
                }
            }
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Division by zero", 0), index: 0}),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <modulo>: Cannot do modulo with non-numeric values", 0), index: 0}),
        }
    }
}
pub fn remainder(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => {
                if is_integer(&n0) && is_integer(&n1) {
                    return Ok(Value::NumericValue(n0 % n1));
                }
                else {
                    return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Cannot do remainder with non-integer values.", 0), index: 0});
                }
            }
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 
                => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Division by zero", 0), index: 0}),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <remainder>: Cannot do remainder with non-numeric values", 0), index: 0}),
        }
    }
}
pub fn eq_q(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <eq?>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <eq?>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 == n1)),
            (Value::BooleanValue(b0), Value::BooleanValue(b1)) => return Ok(Value::BooleanValue(b0 == b1)),
            (Value::NilValue, Value::NilValue) => return Ok(Value::BooleanValue(true)),
            (Value::SymbolValue(s0), Value::SymbolValue(s1)) => return Ok(Value::BooleanValue(s0 == s1)),
            (Value::StringValue(s0), Value::StringValue(s1)) => return Ok(Value::BooleanValue(s0 == s1)),
            (Value::PairValue(car0, cdr0), Value::PairValue(car1, cdr1)) => {
                match eq_q(vec![*car0, *car1].to_vec(), Rc::clone(&env))? {
                    v @ Value::BooleanValue(false) => return Ok(v),
                    Value::BooleanValue(true) => return eq_q(vec![*cdr0, *cdr1].to_vec(), Rc::clone(&env)),
                    _ => panic!("You should never see this message."),
                }
            },
            (Value::ProcedureValue(f0), Value::ProcedureValue(f1)) => 
                return Ok(Value::BooleanValue(std::ptr::eq(&*f0, &*f1))),
            // 我直接规定, 任何两个lambda表达式都是不一样的! 如何?!
            (Value::LambdaValue(_params_in_lambda_0, _body0, _env_in_lambda_0), 
            Value::LambdaValue(_params_in_lambda_1, _body1, _env_in_lambda1)) 
            => return Ok(Value::BooleanValue(false)),
            _ => return Ok(Value::BooleanValue(false)),
        }
    }
}
pub fn equal_q(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <equal?>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <equal?>: Too many argument", 0), index: 0});
    }
    else {
        return Ok(Value::BooleanValue(params[0].to_string() == params[1].to_string()));
    }
}
pub fn not(params: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <not>: Missing argument", 0), index: 0});
    }
    else if params.len() > 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <not>: Too many argument", 0), index: 0});
    }
    else {
        let result = env.eval(params[0].clone());
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => return Ok(Value::BooleanValue(true)),
                Value::BooleanValue(true) => return Ok(Value::BooleanValue(false)),
                _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <not>: Unknown Error", 0), index: 0}),
            }
        }
        else {
            match params[0] {
                Value::NilValue => return Ok(Value::BooleanValue(false)),
                Value::PairValue(_, _) => return Ok(Value::BooleanValue(false)),
                Value::SymbolValue(_) => return Ok(Value::BooleanValue(false)),
                _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <not>: Unknown Error", 0), index: 0}),
            }
        }
    }
}
pub fn equal_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'='>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'='>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 == n1)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'='>: Cannot compare a non-numeric values", 0), index: 0}),
        } 
    }
}
pub fn less_than_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<'>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<'>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 < n1)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<'>: Cannot compare a non-numeric values", 0), index: 0}),
        } 
    }
}
pub fn more_than_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>'>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>'>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 > n1)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>'>: Cannot compare a non-numeric values", 0), index: 0}),
        } 
    }
}
pub fn less_than_or_equal_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<='>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<='>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 <= n1)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'<='>: Cannot compare a non-numeric values", 0), index: 0}),
        } 
    }
}
pub fn more_than_or_equal_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>='>: Missing argument", 0), index: 0});
    }
    else if params.len() > 2 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>='>: Too many argument", 0), index: 0});
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Ok(Value::BooleanValue(n0 >= n1)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <'>='>: Cannot compare a non-numeric values", 0), index: 0}),
        } 
    }
}
pub fn even_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <even?>: Missing argument", 0), index: 0});
    }
    else if params.len() > 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <even?>: Too many argument", 0), index: 0});
    }
    else {
        match params[0] {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Ok(Value::BooleanValue(n as i32 % 2 == 0));
                }
                else {
                    return Err(ErrorEval{ message: format!("{}: Builtin Procedure <even?>: Cannot judge even/odd with a non-integer number", 0), index: 0});
                }
            },
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <even?>: Cannot compare a non-numeric values", 0), index: 0}),
        }
    }
}
pub fn odd_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <odd?>: Missing argument", 0), index: 0});
    }
    else if params.len() > 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <odd?>: Too many argument", 0), index: 0});
    }
    else {
        match params[0] {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Ok(Value::BooleanValue(n as i32 % 2 == 1));
                }
                else {
                    return Err(ErrorEval{ message: format!("{}: Builtin Procedure <odd?>: Cannot judge even/odd with a non-integer number", 0), index: 0});
                }
            },
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <odd?>: Cannot compare a non-numeric values", 0), index: 0}),
        }
    }
}
pub fn zero_or_not(params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if params.len() < 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <zero?>: Missing argument", 0), index: 0});
    }
    else if params.len() > 1 {
        return Err(ErrorEval{ message: format!("{}: Builtin Procedure <zero?>: Too many argument", 0), index: 0});
    }
    else {
        match params[0] {
            Value::NumericValue(n) => return Ok(Value::BooleanValue(n == 0f64)),
            _ => return Err(ErrorEval{ message: format!("{}: Builtin Procedure <zero?>: Cannot compare a non-numeric values", 0), index: 0}),
        }
    }
}
pub fn sort(_params: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    todo!();
}
