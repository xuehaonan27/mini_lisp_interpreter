/// 定义特殊形式

use crate::builtins::list;
use crate::value::Value;
use crate::eval_env::EvalEnv;
use std::rc::Rc;
use crate::error::ErrorEval;
pub type SpecialForm = fn(Vec<Value>, Rc<EvalEnv>) -> Result<Value, ErrorEval>;

/// define 特殊形式.
/// 作用: 向当前求值环境绑定变量
/// ```ignore
/// >>> (define x 42) 绑定x到42
/// >>> (define (double y) (+ y y)) 绑定y到一个过程上
/// >>> (define x (lambda (t)(+ 1 (double t)))) 重新绑定x到一个lambda表达式上
/// ```
pub fn define_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 2 {
        return Err(ErrorEval {
            message: format!("{}: Special Form <define>: Missing parameter", 0),
            index: 0
        });
    }
    match args[0].clone() {
        Value::SymbolValue(s) => {
            // env.symbol_map的类型是RefCell<HashMap<String, Value>>, 检查其中是否有key键, 如果有那么向其中插入value值
            // 先获取共享引用, 进行查找并保存查找结果, 然后drop掉这个共享引用
            // 然后用borrow_mut()方法获得RefCell里面哈希表的可变引用, 进行插入工作
            
            if env.symbol_map.borrow().contains_key(&s) {
                let borrow = env.symbol_map.borrow();
                let value: Option<&Value> = borrow.get(&args[1].to_string());
                
                if value.is_some() {
                    let value_to_be_inserted = value.unwrap().clone();
                    std::mem::drop(borrow);
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value_to_be_inserted);
                }
                else {
                    let value_to_be_inserted = env.clone().eval(args[1].clone()).map_err(|error| ErrorEval{
                        message: format!("{}: Special Form <define>: Fail to evaluate a value\n{}", error.index + 1 ,error.message),
                        index: error.index + 1
                    })?;
                    std::mem::drop(borrow);
                    let mut ref_of_map = env.symbol_map.borrow_mut();
                    _ = ref_of_map.insert(s, value_to_be_inserted);
                }
            }
            else {
                let value_to_be_inserted = env.clone().eval(args[1].clone()).map_err(|error| ErrorEval{
                    message: format!("{}: Special Form <define>: Fail to evaluate a value\n{}", error.index + 1 ,error.message),
                    index: error.index + 1
                })?;
                let mut ref_of_map = env.symbol_map.borrow_mut();
                _ = ref_of_map.insert(s, value_to_be_inserted);
            }
        },
        Value::PairValue(car, cdr) => {
            match *car {
                Value::SymbolValue(s) => {
                    let mut lambda_args: Vec<Value> = vec![*cdr];
                    lambda_args.append(&mut args[1..].to_vec());
                    let temp_env = env.clone();
                    _ = env.symbol_map.borrow_mut().insert(s, lambda_form(lambda_args, temp_env)?);
                },
                _ => return Err(ErrorEval { message: format!("{}: Special Form <define>: Malformed define", 0), index: 0 }),
            }
        },
        _ => return Err(ErrorEval { message: format!("{}: Special Form <define>: Malformed define", 0), index: 0 })
    }
    Ok(Value::NilValue)
}

/// quote 特殊形式
/// 其引导的表达式将不被求值, 任何时候返回字符串外部表达
/// ```ignore
/// (print (+ 1 2)) 输出结果: 3
/// (print '(+ 1 2)) 输出结果: (+ 1 2)
pub fn quote_form(args: Vec<Value>, _env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 1 {
        Err(ErrorEval{message: format!("{}: Special Form <quote>: Missing parameter", 0), index: 0})
    }
    else {
        Ok(args[0].clone())
    }
}

/// if 特殊形式
/// (if (条件) (真分支) (假分支))
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

/// and 特殊形式
/// (and <expr 1> <expr 2> <expr 3>)
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
    env.eval(args[args.len() - 1].clone()).or(Ok(Value::NilValue))
}

/// or 特殊形式
/// (and <expr 1> <expr 2> <expr 3>)
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

/// lambda 特殊形式
/// (define foobar (lambda (x) (print x))))
pub fn lambda_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 2{
        return Err(ErrorEval{message: format!("{}: Special Form <lambda>: Missing part of lambda expression", 0), index: 0});
    }
    let vec: Vec<Value> = args[0].to_vector().map_err(|error| ErrorEval {
        message: format!("{}: Special Form <lambda>: Fail to convert value to vector\n{}", error.index + 1, error.message),
        index: error.index + 1
    })?;
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
        Value::PairValue(_, _) => return Ok(Value::LambdaValue(Box::new(params), Box::new(body), Rc::clone(&env))),
        _ => return Ok(Value::LambdaValue(Box::new(Vec::<String>::new()), Box::new(body), Rc::clone(&env))),
    }
}

/// cond 特殊形式
/// (cond (条件1 值1) (条件2 值2) (条件3 值3) ... )
/// 逐个条件求值, 除非求得为布尔字面量否, 否则都认为是真
/// 一旦遇到真, 则返回该条件对应的值.
/// 未遇到真, 则不返回
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

/// begin 特殊形式
/// (begin <expr1> <expr2> ... <expr n>)
/// 每个子句逐个执行
/// ```ignore
/// >>> (begin
/// ...       (define x 42)
/// ...       (define double (lambda (t) (+ t t)))
/// ...       (define y 3.14)
/// ...       (print y)
/// ...       (cond ((= x 41) (print "pos 1")) ((= y 3.14) (print "pos 2")))
/// ...       )
/// ```
/// 输出结果
/// 3.14
/// "pos 2"
pub fn begin_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.is_empty() {
        return Err(ErrorEval {
            message: format!("{}: Special Form <begin>: Missing parameter", 0),
            index: 0
        });
    }
    let mut result: Value = Value::NilValue;
    for arg in args {
        result = env.clone().eval(arg).map_err(|error| ErrorEval{
            message: format!("{}: Special Form <begin>: Missing parameter\n{}", error.index + 1, error.message),
            index: error.index + 1
        })?
    }
    Ok(result)
}

/// let 特殊形式
/// 在当前求值环境中绑定一些临时变量
pub fn let_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut params1: Vec<String> = Vec::new();
    let mut params2: Vec<Value> = Vec::new();
    let bindings: Vec<Value>;
    match args[0] {
        Value::PairValue(_, _) => bindings = args[0].to_vector().map_err(|error| ErrorEval {
            message: format!("{}: Special Form <let>: Fail to convert value to vector\n{}", error.index + 1, error.message),
            index: error.index + 1
        })?,
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
                    return Err(ErrorEval{
                        message: format!("{}: Special Form <let>: temporary binding should be a 2-element list", 0),
                        index: 0
                    });
                }
            },
            _ => return Err(ErrorEval{
                message: format!("{}: Special Form <let>: temporary binding should be a 2-element list", 0),
                index: 0
            }),
        }
    }
    let mut results: Vec<Value> = Vec::new();
    let env_derived: Rc<EvalEnv> = env.derive(params1, params2).into();
    args[1..].iter().for_each(|arg| results.push(env_derived.clone().eval(arg.clone()).expect("Corruption when evaluating a value in form <let>")));
    Ok(results.pop().unwrap_or(Value::NilValue))
}

/// quasiquote 特殊形式
/// 与quote类似, 不过由,逗号表达式(unquote)引导的表达式会被求值
pub fn quasiquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    let mut results: Vec<Value> = Vec::new();
    let arg_vec: Vec<Value> = args[0].to_vector().map_err(|error| ErrorEval{
        message: format!("{}: Special Form <quasiquote>: Fail to evaluate a value\n{}", error.index + 1, error.message),
        index: error.index + 1
    })?;
    for arg in arg_vec {
        match arg.clone() {
            Value::PairValue(car, cdr) => {
                match *car {
                    Value::SymbolValue(s) if s == "unquote".to_string() => {
                        results.push(unquote_form(cdr.to_vector().map_err(|error| ErrorEval{
                            message: format!("{}: Special Form <quasiquote>: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?, env.clone())?);
                    },
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

/// unquote特殊形式
/// 用于在quasiquote中豁免表达式的
pub fn unquote_form(args: Vec<Value>, env: Rc<EvalEnv>) -> Result<Value, ErrorEval> {
    if args.len() < 1 {
        return Err(ErrorEval{
            message: format!("{}: Special Form <unquote>: Missing argument", 0),
            index: 0
        });
    }
    else if args.len() > 1{
        return Err(ErrorEval {
            message: format!("{}: Special Form <unquote>: Too many argument", 0),
            index: 0
        });
    }
    else {
        env.eval(args[0].clone()).map_err(|error| ErrorEval {
            message: format!("{}: Special Form <let>: Fail to convert value to vector\n{}", error.index + 1, error.message),
            index: error.index + 1
        })
    }
}