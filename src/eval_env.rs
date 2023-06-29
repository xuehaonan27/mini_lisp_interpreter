use std::collections::HashMap;
use std::rc::Rc;
use std::cell::Cell;
use crate::value::Value;
use crate::builtins;
use crate::special_forms::*;
use crate::builtins::*;
use crate::value::BuiltinFn;
#[derive(Clone)]
pub struct EvalEnv{
    pub symbol_map: HashMap<String, Value>,
    pub parent: Rc<Option<EvalEnv>>,
    pub special_forms: HashMap<String, SpecialForm>,
    pub builtin_procs: HashMap<String, BuiltinFn>
}

impl EvalEnv {
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
        let symbol_map: HashMap<String, Value> = HashMap::new();
        let parent: Rc<Option<EvalEnv>> = Rc::new(None);
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    pub fn derive(&self, params: Vec<String>, args: Vec<Value>) -> Self {
        if params.len() < args.len() {
            panic!("Too many parameters.");
        }
        else if params.len() > args.len() {
            panic!("Missing parameters.");
        }
        let special_forms: HashMap<String, SpecialForm> = self.special_forms.clone();
        let builtin_procs: HashMap<String, BuiltinFn> = self.builtin_procs.clone();
        let parent: Rc<Option<EvalEnv>> = Rc::new(Some(self.clone()));
        let mut symbol_map: HashMap<String, Value> = HashMap::new();
        for (key, value) in params.iter().zip(args.iter()) {
            symbol_map.insert(key.clone(), value.clone());
        }
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    pub fn find_binding(&self, name: &String) -> Option<&Value> {
        if self.symbol_map.contains_key(name) {
            self.symbol_map.get(name)
        }
        else {
            if self.parent.is_none() {
                None
            }
            else {
                (*self.parent).as_ref().unwrap().find_binding(name)
            }
        }
    }
    /// 将待传入参数由Value::PairValue类型转为向量并先在当前环境中逐个求值
    /// 由于其内部调用的eval是求值务必求到尽头
    /// 所以最终返回的向量包含的值必然都是
    fn eval_list(&self, expr: Value) -> Vec<Value> {
        let mut result: Vec<Value> = Vec::new();
        let v: Vec<Value> = expr.to_vector();
        v.iter().for_each(|value| { result.push(self.eval(value.clone()));});
        result
    }
    pub fn eval(&self, expr: Value) -> Value {
        match expr {
            Value::BooleanValue(_) => return expr,
            Value::NumericValue(_) => return expr,
            Value::StringValue(_) => return expr,
            Value::NilValue => panic!("Evaluating nil is prohibited."),
            Value::ProcedureValue(_) => return expr,
            Value::LambdaValue(_, _, _) => return expr,
            Value::SymbolValue(s) => {
                // return self.find_binding(&s).expect(&format!("Variable {} not defined.", s)).clone();

                let item1 =  self.find_binding(&s);
                if item1.is_some() {
                    return item1.unwrap().clone();
                }
                else {
                    let item2 = self.builtin_procs.get(&s);
                    if item2.is_some() {
                        return Value::ProcedureValue(Box::new(*item2.unwrap()));
                    }
                    else {
                        let item3 = self.special_forms.get(&s);
                        if item3.is_some() {
                            return Value::ProcedureValue(Box::new(*item3.unwrap()));
                        }
                        else {
                            panic!("Variable {s} not defined.");
                        }
                    }
                }
            }
            exprs @ Value::PairValue(_, _) => {
                let v: Vec<Value> = exprs.to_vector();
                match &v[0] {
                    Value::SymbolValue(s) => {
                        match self.find_binding(s) {
                            None => {},
                            Some(Value::ProcedureValue(f)) => {
                                let args: Vec<Value> = v[1..].iter().cloned().map(|value| self.eval(value)).collect();
                                return f(args, self);
                            },
                            Some(Value::LambdaValue(_, _, _)) => {
                                todo!();
                            },
                            _ => panic!("Invalid format."),
                        }
                        if self.special_forms.contains_key(s) {
                            if *s == "unquote".to_string() {
                                panic!("Calling unquote outside quasiquote is an undefined behavior.");
                            }
                            return self.special_forms.get(s).unwrap()(v[1..].to_vec(), self);
                        }
                        else if self.builtin_procs.contains_key(s) {
                            let args: Vec<Value> = v[1..].iter().cloned().map(|value| self.eval(value)).collect();
                            return self.builtin_procs.get(s).unwrap()(args, self);
                        }
                        else {
                            panic!("Name {s} not defined.");
                        }

                    },
                    Value::PairValue(_, _) => {
                        let mut new_vec: Vec<Value> = Vec::new();
                        v.iter().for_each(|value| {
                            new_vec.push(self.eval(value.clone()));
                        });
                        let new_expr: Value = list(new_vec, self);
                        self.eval(new_expr)
                    },
                    Value::ProcedureValue(f) => {
                        // let proc: Value = self.eval(v[0].clone());
                        // self.apply(proc, v[1..].to_vec())
                        f(v[1..].to_vec(), self)
                    },
                    Value::LambdaValue(params, body, env) => {
                        let env_derived = env.derive(*params.clone(), v[1..].to_vec());
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.eval(bodyv);
                        }
                        return result;
                    },
                    _ => {
                        panic!("Invalid format. Cannot evaluate it as a symbol or procedure.");
                    },
                }
            },
        }
    }
    fn apply(&self, proc: Value, args: Vec<Value>) -> Value {
        todo!();
    }
}