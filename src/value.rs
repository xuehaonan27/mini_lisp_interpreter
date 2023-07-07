/// 定义解释器内"值"的概念
/// 解释器执行的时候内部所有表达都是以"值"的形式进行传递的

use std::hash::{Hash,Hasher};
use std::fmt::Debug;
use std::rc::Rc;
use crate::error::ErrorEval;
use crate::eval_env::EvalEnv;
pub type BuiltinFn = fn(Vec<Value>, Rc<EvalEnv>) -> Result<Value, ErrorEval>;

/// 值类型
/// 布尔字面量, 数字字面量, 字符串字面量
/// 空字面量, 符号, 对子
/// 过程(内置过程与特殊形式), lambda表达式(外部定义)
#[derive(Clone)]
pub enum Value {
    BooleanValue(bool),
    NumericValue(f64),
    StringValue(String),
    NilValue,
    SymbolValue(String),
    PairValue(Box<Value>, Box::<Value>),
    ProcedureValue(Box<BuiltinFn>),
    LambdaValue(Box<Vec<String>>, Box<Vec<Value>>, Rc<EvalEnv>),
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

/// 将"值"类型用字符串的方式表达出来
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
                    format!("{}", n)
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
                    match bind.1 {
                        Value::ProcedureValue(_) => continue,
                        Value::LambdaValue(_, _, _) => continue,
                        _ => {
                            env_string += format!("({}, {})", bind.0, bind.1.to_string()).as_str();
                            env_string.push('\n');
                        },
                    }
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

/// 允许值进行哈希
/// 用于加入哈希表
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



impl Value {
    /// 将值转化为向量
    /// 其它值转化还是本身, 但是会将对子值展开.
    pub fn to_vector(&self) -> Result<Vec<Self>, ErrorEval> {
        fn to_vector_recursive(expr: &Value, vec: &mut Vec<Rc<Value>>) -> Result<(), ErrorEval>{
            match expr {
                Value::BooleanValue(_) => { vec.push(Rc::new(expr.clone())); Ok(()) },
                Value::NumericValue(_) => { vec.push(Rc::new(expr.clone())); Ok(()) },
                Value::StringValue(_) => { vec.push(Rc::new(expr.clone())); Ok(()) },
                Value::NilValue => Ok(()),
                Value::SymbolValue(_) => { vec.push(Rc::new(expr.clone())); Ok(()) },
                Value::PairValue(car, cdr) => {
                    vec.push(Rc::new(*(*car).clone()));
                    to_vector_recursive(cdr, vec)?;
                    Ok(())
                }
                // _ => panic!("Invalid format when converting pairvalue to vector."),
                _ => Err(ErrorEval{message: format!("{}: [to_vector]: Invalid format when converting pairvalue to vector", 0), index: 0}),
            }
        }
        let mut vec: Vec<Rc<Value>> = Vec::new();
        let result = to_vector_recursive(self, &mut vec);
        match result {
            Ok(_) => return Ok(vec.iter().cloned().map(|v| (*v).clone()).collect()),
            Err(err) => return Err(err),
        }
    }
}