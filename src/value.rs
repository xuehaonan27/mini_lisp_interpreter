use std::hash::{Hash,Hasher};
type BuiltinFn = fn(Vec<Value>) -> Value;

#[derive(Clone)]
pub enum Value {
    BooleanValue(bool),
    NumericValue(f64),
    StringValue(String),
    NilValue,
    SymbolValue(String),
    PairValue(Box<Value>, Box::<Value>),
    ProcedureValue(Box<BuiltinFn>),
    LambdaValue(Box<Vec<String>>, Box<Vec<Value>>),
}
pub fn is_integer(num: &f64) -> bool {
    num.abs() < std::f64::EPSILON ||
    (num - num.floor()).abs() < std::f64::EPSILON ||
    (num - num.ceil()).abs() < std::f64::EPSILON
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
                    format!("{:.6}", n)
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
            Value::LambdaValue(params, body) => {
                let mut params_string: String = String::new();
                let mut body_string: String = String::new();
                for param in &**params {
                    params_string += param.clone().as_str();
                    params_string.push(' ');
                }
                for bodyv in &**body {
                    body_string += bodyv.to_string().as_str();
                    body_string.push(' ');
                }
                format!("Lambda Expression with param: {} and body: {}", params_string, body_string)
            },
            Value::PairValue(box_car, box_cdr) => {
                let mut s: String = format!("({} ", box_car.to_string());
                match &**box_cdr {
                    v @ Value::BooleanValue(_b) => {
                        format!("{}. {})", s, v.to_string())
                    },
                    v @ Value::NumericValue(_n) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::StringValue(_string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::SymbolValue(_string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::ProcedureValue(_string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::LambdaValue(_string1, _string2) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::PairValue(_car, _cdr) => {
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
            v @ Value::LambdaValue(_params, _body) => v.to_string().hash(state),
        }
    }
}
