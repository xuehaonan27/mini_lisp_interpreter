use std::collections::HashMap;
use std::rc::Rc;
use crate::Value;
use crate::special_forms::*;
use crate::builtins::*;
use crate::value::BuiltinFn;
#[derive(Clone)]
pub struct EvalEnv{
    symbol_map: HashMap<String, Value>,
    parent: Rc<Option<EvalEnv>>,
    pub special_forms: HashMap<String, SpecialForm>,
    builtin_procs: HashMap<String, BuiltinFn>
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
        ]);
        let symbol_map: HashMap<String, Value> = HashMap::new();
        let parent: Rc<Option<EvalEnv>> = Rc::new(None);
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    fn derive(&self) -> Self {
        let special_forms: HashMap<String, SpecialForm> = self.special_forms.clone();
        let builtin_procs: HashMap<String, BuiltinFn> = self.builtin_procs.clone();
        let parent: Rc<Option<EvalEnv>> = Rc::new(Some(self.clone()));
        let symbol_map: HashMap<String, Value> = HashMap::new();
        Self {symbol_map, parent, special_forms, builtin_procs}
    }
    
    fn eval(&self, expr: Value) -> Value {
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
            Value::PairValue(car, _cdr) => {
                // let v = expr.to_vector();
                // return Value::BooleanValue(true);
                match *car {
                    Value::SymbolValue(s) => {
                        todo!();
                    },
                    Value::PairValue(_, _) => {
                        todo!();
                    },
                    Value::ProcedureValue(f) => {
                        todo!();
                    },
                    Value::LambdaValue(params, body) => {
                        todo!();
                    },
                    _ => {
                        panic!("Invalid format. Cannot evaluate it as a symbol or procedure.");
                    },
                }
            },
        }
    }
}