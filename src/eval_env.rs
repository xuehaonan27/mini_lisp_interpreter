/// 定义了求值环境以及解析器求值的过程

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::special_forms::*;
use crate::builtins::*;
use crate::value::BuiltinFn;
use crate::error::ErrorEval;

/// 求值环境的定义
/// symbol_map: 利用RefCell, 使得在整个求值环境实例不可变的情况下内部可变. 具有重要意义: 函数类型定义中不可以出现&mut, 整个求值环境实例不可变, 但是内部又需要修改内部
/// parent: 父级求值环境
/// special_forms: 特殊形式对应表
/// builtin_procs: 内置过程对应表
#[derive(Clone)]
pub struct EvalEnv{
    pub symbol_map: RefCell<HashMap<String, Value>>,
    pub parent: Option<Rc<EvalEnv>>,
    pub special_forms: HashMap<String, SpecialForm>,
    pub builtin_procs: HashMap<String, BuiltinFn>
}

impl EvalEnv {
    /// 新建求值环境, 完成对应表的初始化工作
    pub fn new() -> Self {
        let special_forms:HashMap<String, SpecialForm> = HashMap::from([
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
        ]);
        let builtin_procs: HashMap<String, BuiltinFn> = HashMap::from([
            ("apply".to_string(), apply as BuiltinFn),
            ("print".to_string(), print as BuiltinFn),
            ("display".to_string(), display as BuiltinFn),
            ("displayln".to_string(), displayln as BuiltinFn),
            ("error".to_string(), error as BuiltinFn),
            ("eval".to_string(), eval as BuiltinFn),
            ("exit".to_string(), exit as BuiltinFn),
            ("exit_force".to_string(), exit_force as BuiltinFn),
            ("newline".to_string(), newline as BuiltinFn),

            ("atom?".to_string(), atom_or_not as BuiltinFn),
            ("boolean?".to_string(), boolean_or_not as BuiltinFn),
            ("integer?".to_string(), integer_or_not as BuiltinFn),
            ("list?".to_string(), list_or_not as BuiltinFn),
            ("number?".to_string(), number_or_not as BuiltinFn),
            ("null?".to_string(), null_or_not as BuiltinFn),
            ("pair?".to_string(), pair_or_not as BuiltinFn),
            ("procedure?".to_string(), procedure_or_not as BuiltinFn),
            ("string?".to_string(), string_or_not as BuiltinFn),
            ("symbol?".to_string(), symbol_or_not as BuiltinFn),
            ("defined_local?".to_string(), defined_local_or_not as BuiltinFn),
            ("defined_all?".to_string(), defined_all_or_not as BuiltinFn),

            ("append".to_string(), append as BuiltinFn),
            ("push".to_string(), push as BuiltinFn),
            ("car".to_string(), car as BuiltinFn),
            ("cdr".to_string(), cdr as BuiltinFn),
            ("cons".to_string(), cons as BuiltinFn),
            ("length".to_string(), length as BuiltinFn),
            ("list".to_string(), list as BuiltinFn),
            ("map".to_string(), map as BuiltinFn),
            ("map_expand".to_string(), map_expand as BuiltinFn),
            ("filter".to_string(), filter as BuiltinFn),
            ("reduce".to_string(), reduce as BuiltinFn),

            ("+".to_string(), add as BuiltinFn),
            ("-".to_string(), subtract as BuiltinFn),
            ("*".to_string(), multiply as BuiltinFn),
            ("/".to_string(), divide as BuiltinFn),
            ("abs".to_string(), abs as BuiltinFn),
            ("expt".to_string(), expt as BuiltinFn),
            ("quotient".to_string(), quotient as BuiltinFn),
            ("modulo".to_string(), modulo as BuiltinFn),
            ("remainder".to_string(), remainder as BuiltinFn),

            ("eq?".to_string(), eq_q as BuiltinFn),
            ("equal?".to_string(), equal_q as BuiltinFn),
            ("not".to_string(), not as BuiltinFn),
            ("=".to_string(), equal_or_not as BuiltinFn),
            ("<".to_string(), less_than_or_not as BuiltinFn),
            (">".to_string(), more_than_or_not as BuiltinFn),
            ("<=".to_string(), less_than_or_equal_or_not as BuiltinFn),
            (">=".to_string(), more_than_or_equal_or_not as BuiltinFn),
            ("even?".to_string(), even_or_not as BuiltinFn),
            ("odd?".to_string(), odd_or_not as BuiltinFn),
            ("zero?".to_string(), zero_or_not as BuiltinFn),
            ("sort".to_string(), sort as BuiltinFn),
        ]);
        let symbol_map: RefCell<HashMap<String, Value>> = RefCell::new(HashMap::new());
        let parent: Option<Rc<EvalEnv>> = None;
        Self {symbol_map, parent, special_forms, builtin_procs}
    }

    /// 从当前求值环境, 插入params - args键值对, 形成新的环境
    pub fn derive(self: Rc<EvalEnv>, params: Vec<String>, args: Vec<Value>) -> Self {
        if params.len() < args.len() {
            panic!("Too many parameters.");
        }
        else if params.len() > args.len() {
            panic!("Missing parameters.");
        }
        let special_forms: HashMap<String, SpecialForm> = self.special_forms.clone();
        let builtin_procs: HashMap<String, BuiltinFn> = self.builtin_procs.clone();
        let parent: Option<Rc<EvalEnv>> = Some(Rc::clone(&self));
        let mut symbol_map: HashMap<String, Value> = HashMap::new();
        for (key, value) in params.iter().zip(args.iter()) {
            symbol_map.insert(key.clone(), value.clone());
        }
        let symbol_map = RefCell::new(symbol_map);
        Self {symbol_map, parent, special_forms, builtin_procs}
    }

    /// 在当前求值环境及其各级父级环境中查找变量绑定
    pub fn find_binding(self: Rc<EvalEnv>, name: &String) -> Option<Value> {
        if self.symbol_map.borrow().contains_key(name) {
            self.symbol_map.borrow().get(name).cloned()
        }
        else {
            if self.parent.is_none() {
                None
            }
            else {
                self.parent.clone().unwrap().find_binding(name)
            }
        }
    }
    
    /// 解释器求值过程
    /// 拿到parse之后的"值"
    /// 一般来说, 一个表达式一定是一个字面量(直接返回本身即可)
    /// 或者是一个由括号表达式括起来的对子值(对这个PairValue进行求值即可)
    pub fn eval(self: Rc<EvalEnv>, expr: Value) -> Result<Value, ErrorEval> {
        match expr {
            Value::BooleanValue(_) => return Ok(expr),
            Value::NumericValue(_) => return Ok(expr),
            Value::StringValue(_) => return Ok(expr),
            Value::NilValue => return Err(ErrorEval{message: format!("{}: [eval]: evaluate NilValue is prohibited", 0), index: 0}),
            Value::ProcedureValue(_) => return Ok(expr),
            Value::LambdaValue(_, _, _) => return Ok(expr),
            Value::SymbolValue(s) => {
                let item1 =  self.clone().find_binding(&s);
                if item1.is_some() {
                    return Ok(item1.unwrap().clone());
                }
                else {
                    if s == "else".to_string() {
                        return Ok(Value::SymbolValue("else".to_string()));
                    }
                    let item2 = self.builtin_procs.get(&s);
                    if item2.is_some() {
                        return Ok(Value::ProcedureValue(Box::new(*item2.unwrap())));
                    }
                    else {
                        let item3 = self.special_forms.get(&s);
                        if item3.is_some() {
                            return Ok(Value::ProcedureValue(Box::new(*item3.unwrap())));
                        }
                        else {
                            return Err(ErrorEval{message: format!("{}: [eval]: Variable {s} not defined", 0), index: 0});
                        }
                    }
                }
            }
            
            // 对子值比较特殊, 需要展开求解
            exprs @ Value::PairValue(_, _) => {
                let v: Vec<Value> = exprs.to_vector().map_err(|error| ErrorEval{
                    message: format!("{}: [eval]: Fail to convert a value to vector\n{}", error.index + 1, error.message),
                    index: error.index + 1
                })?;
                match &v[0] {
                    Value::SymbolValue(s) => {
                        match self.clone().find_binding(s) {
                            None => {},
                            Some(Value::ProcedureValue(f)) => {
                                let result_args: Result<Vec<Value>, ErrorEval> = v[1..].iter().cloned().map(|value| self.clone().eval(value)).collect();
                                let args = result_args.map_err(|error| ErrorEval{
                                    message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                    index: error.index + 1
                                })?;
                                return f(args, Rc::clone(&self));
                            },
                            Some(Value::LambdaValue(params_in_lambda, body, env_in_lambda)) => {
                                let result_args: Result<Vec<Value>, ErrorEval >= v[1..].iter().map(|value| self.clone().eval(value.clone())).collect();
                                let args = result_args.map_err(|error| ErrorEval{
                                    message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                    index: error.index + 1
                                })?;
                                let env_derived: Rc<EvalEnv> = env_in_lambda.derive(*params_in_lambda, args).into();
                                let mut result: Value = Value::NilValue;
                                for bodyv in *body {
                                    result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                        message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                        index: error.index + 1
                                    })?;
                                }
                                return Ok(result);
                            },
                            _ => return Err(ErrorEval{message: format!("{}: [eval]: Invalid format", 0), index: 0}),
                        }
                        if self.special_forms.contains_key(s) {
                            if *s == "unquote".to_string() {
                                return Err(ErrorEval{message: format!("{}: [eval]: Calling unquote outside quasiquote is an undefined behavior", 0), index: 0});
                            }
                            return self.special_forms.get(s).unwrap()(v[1..].to_vec(), Rc::clone(&self));
                        }
                        else if self.builtin_procs.contains_key(s) {
                            let result_args: Result<Vec<Value>, ErrorEval> = v[1..].iter().map(|value| self.clone().eval(value.clone())).collect();
                            let args: Vec<Value> = result_args.map_err(|error| ErrorEval{
                                message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })?;
                            return self.builtin_procs.get(s).unwrap()(args, Rc::clone(&self));
                        }
                        else {
                            return Err(ErrorEval{message: format!("{}: [eval]: Name {s} not defined", 0), index: 0});
                        }

                    },
                    Value::PairValue(_, _) => {
                        let mut new_vec: Vec<Value> = Vec::new();
                        v.iter().try_for_each(|value| -> Result<(), ErrorEval>{
                            let result_arg: Value = self.clone().eval(value.clone()).map_err(|error| ErrorEval{
                                message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })?;
                            new_vec.push(result_arg);
                            Ok(())
                        })?;
                        let new_expr: Value = list(new_vec, Rc::clone(&self)).map_err(|error|ErrorEval{
                            message: format!("{}: [eval]: Fail to pack the value\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })?;
                        self.eval(new_expr)
                    },
                    Value::ProcedureValue(f) => {
                        f(v[1..].to_vec(), Rc::clone(&self)).map_err(|error| ErrorEval {
                            message: format!("{}: [eval]: Fail to call the given procedure\n{}", error.index + 1, error.message),
                            index: error.index + 1
                        })
                    },
                    Value::LambdaValue(params, body, env) => {
                        let env_derived: Rc<EvalEnv> = env.clone().derive(*params.clone(), v[1..].to_vec()).into();
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.clone().eval(bodyv).map_err(|error| ErrorEval{
                                message: format!("{}: [eval]: Fail to evaluate a value\n{}", error.index + 1, error.message),
                                index: error.index + 1
                            })?;
                        }
                        return Ok(result);
                    },
                    _ => {
                        return Err(ErrorEval {
                            message: format!("{}: [eval]: Invalid format. Cannot evaluate it as a symbol or procedure", 0),
                            index: 0
                        })
                    },
                }
            },
        }
    }
}