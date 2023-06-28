use std::collections::HashMap;
use std::rc::Rc;
use crate::Value;
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
            ("+".to_string(), add as BuiltinFn),
        ]);
        let symbol_map: HashMap<String, Value> = HashMap::new();
        let parent: Rc<Option<EvalEnv>> = Rc::new(None);
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    pub fn derive(&self) -> Self {
        let special_forms: HashMap<String, SpecialForm> = self.special_forms.clone();
        let builtin_procs: HashMap<String, BuiltinFn> = self.builtin_procs.clone();
        let parent: Rc<Option<EvalEnv>> = Rc::new(Some(self.clone()));
        let symbol_map: HashMap<String, Value> = HashMap::new();
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    fn find_binding(&self, name: &String) -> Option<&Value> {
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
    /// 将待传入参数由Value::PairValue类型转为向量并先逐个求值
    fn eval_list(&self, expr: Value) -> Vec<Value> {
        let mut result: Vec<Value> = Vec::new();
        let v: Vec<Value> = expr.to_vector();
        v.iter().for_each(|value| { result.push(self.eval(value.clone()));});
        result
    }
    pub fn eval(&self, expr: Value) -> Value {
        match expr {
            Value::SymbolValue(s) => {
                let item = self.symbol_map.get(&s).expect(&format!("Variable {} not defined.", s));
                return item.clone();
            }
            Value::BooleanValue(_) => return expr,
            Value::NumericValue(_) => return expr,
            Value::StringValue(_) => return expr,
            Value::NilValue => panic!("Evaluating nil is prohibited."),
            Value::ProcedureValue(_) => return expr,
            Value::LambdaValue(_, _) => return expr,
            exprs @ Value::PairValue(_, _) => {
                let v: Vec<Value> = exprs.to_vector();
                match &v[0] {
                    // 注意这里的顺序需要调换! 应该首先尝试匹配symbol_map而不是特殊形式和内置过程!
                    Value::SymbolValue(s) => {
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
                            match self.find_binding(s) {
                                None => panic!("Name {s} not defined."),
                                Some(Value::ProcedureValue(f)) => {
                                    let args: Vec<Value> = v[1..].iter().cloned().map(|value| self.eval(value)).collect();
                                    return f(args, self);
                                },
                                Some(Value::LambdaValue(_, _)) => {
                                    todo!();
                                },
                                _ => panic!("Invalid format."),
                            }
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
                    Value::LambdaValue(params, body) => {
                        let proc: Value = self.eval(v[0].clone());
                        self.apply(proc, v[1..].to_vec())
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