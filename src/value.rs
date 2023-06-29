use std::hash::{Hash,Hasher};
pub type BuiltinFn = fn(Vec<Value>, env: &EvalEnv) -> Value;

use std::fmt::Debug;
#[derive(Clone)]
pub enum Value {
    BooleanValue(bool),
    NumericValue(f64),
    StringValue(String),
    NilValue,
    SymbolValue(String),
    PairValue(Box<Value>, Box::<Value>),
    ProcedureValue(Box<BuiltinFn>),
    LambdaValue(Box<Vec<String>>, Box<Vec<Value>>, EvalEnv),
}
pub fn is_integer(num: &f64) -> bool {
    num.abs() < std::f64::EPSILON ||
    (num - num.floor()).abs() < std::f64::EPSILON ||
    (num - num.ceil()).abs() < std::f64::EPSILON
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BooleanValue(b) => write!(f, "BooleanValue {b}"),
            Self::NumericValue(n) => write!(f, "NumericValue {n}"),
            Self::StringValue(s) => write!(f, "StringValue {s}"),
            Self::NilValue => write!(f, "NilValue"),
            Self::SymbolValue(s) => write!(f, "SymbolValue {s}"),
            Self::PairValue(_, _) => write!(f, "PairValue {}", self.to_string()),
            Self::ProcedureValue(_) => write!(f, "ProcedureValue"),
            Self::LambdaValue(_, _, _) => write!(f, "LambdaValue"),
        }
    }
}
impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::BooleanValue(true) => format!("#t"),
            Value::BooleanValue(false) => format!("#f"),
            Value::NumericValue(n) => {
                if is_integer(n) {
                    format!("{}", *n as i64)
                }
                else {
                    format!("{:6}", n)
                }
            },
            Value::StringValue(s) => {
                format!("\"{}\"", s)
            },
            Value::NilValue => {
                format!("()")
            },
            Value::SymbolValue(s) => {
                format!("{}", s)
            },
            Value::ProcedureValue(_f) => {
                format!("#<procedure>")
            },
            Value::LambdaValue(params, body, env) => {
                let mut params_string: String = String::new();
                let mut body_string: String = String::new();
                let mut env_string: String = String::new();
                for param in &**params {
                    params_string += param.clone().as_str();
                    params_string.push(' ');
                }
                for bodyv in &**body {
                    body_string += bodyv.to_string().as_str();
                    body_string.push(' ');
                }
                for bind in env.symbol_map.borrow().clone() {
                    env_string += format!("({}, {})", bind.0, bind.1.to_string()).as_str();
                    env_string.push('\n');
                }
                format!("Lambda Expression with\nparam: {}\nbody: {}\nenv: {}", params_string, body_string, env_string)
            },
            Value::PairValue(box_car, box_cdr) => {
                let mut s: String = format!("({} ", box_car.to_string());
                match &**box_cdr {
                    v @ Value::BooleanValue(_) => {
                        format!("{}. {})", s, v.to_string())
                    },
                    v @ Value::NumericValue(_) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::StringValue(_) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::SymbolValue(_) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::ProcedureValue(_) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::LambdaValue(_, _, _) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::PairValue(_, _) => {
                        let mut rs = v.to_string();
                        rs.remove(0);
                        format!("{}{}", s, rs)
                    },
                    Value::NilValue => {
                        s.pop();
                        format!("{})", s)
                    },
                }
            },
        }
    }
}

/*
很多类型，如bool、Rc的指针等，都已经实现了哈希方法，
但浮点类型f64并没有，原因是Nan，
Lua禁止使用Nan作为索引，我们就可以忽略Nan而默认浮点类型可以做哈希。
选择了转换为更简单的整型i64来做哈希。
 */
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::NilValue => (),
            Value::BooleanValue(b) => b.hash(state),
            Value::NumericValue(f) => // TODO try to convert to integer
                unsafe {
                    std::mem::transmute::<f64, i64>(*f).hash(state)
                }
            Value::StringValue(s) => s.hash(state),
            Value::SymbolValue(s) => s.hash(state),
            v @ Value::PairValue(_car, _cdr) => v.to_string().hash(state),
            Value::ProcedureValue(f) => (**f as *const usize).hash(state),
            v @ Value::LambdaValue(_, _, _) => v.to_string().hash(state),
        }
    }
}

/*impl Value {
    pub fn to_vector(&self) -> Vec<Self> {
        let mut vec: Vec<Self> = Vec::new();
        let mut my_value = self.clone();
        loop {
            match my_value {
                v @ Self::BooleanValue(_) => vec.push(v.clone()),
                v @ Self::NumericValue(_) => vec.push(v.clone()),
                v @ Self::StringValue(_) => vec.push(v.clone()),
                Self::NilValue => return vec,
                v @ Self::SymbolValue(_) => vec.push(v.clone()),
                Self::PairValue(car, cdr) => {
                    vec.push(*car.clone());
                    my_value = (*cdr).clone();
                },
                _ => panic!("Invalid format when converting pairvalue to vector."),
            }
        }
    }
}*/
use std::rc::Rc;

use crate::eval_env::EvalEnv;
impl Value {
    pub fn to_vector(&self) -> Vec<Self> {
        fn to_vector_recursive(expr: &Value, vec: &mut Vec<Rc<Value>>) {
            match expr {
                Value::BooleanValue(_) => vec.push(Rc::new(expr.clone())),
                Value::NumericValue(_) => vec.push(Rc::new(expr.clone())),
                Value::StringValue(_) => vec.push(Rc::new(expr.clone())),
                Value::NilValue => (),
                Value::SymbolValue(_) => vec.push(Rc::new(expr.clone())),
                Value::PairValue(car, cdr) => {
                    vec.push(Rc::new(*(*car).clone()));
                    to_vector_recursive(cdr, vec);
                }
                _ => panic!("Invalid format when converting pairvalue to vector."),
            }
        }
        let mut vec: Vec<Rc<Value>> = Vec::new();
        to_vector_recursive(self, &mut vec);
        vec.iter().cloned().map(|v| (*v).clone()).collect()
    }
}