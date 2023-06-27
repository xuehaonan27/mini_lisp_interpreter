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
            Value::ProcedureValue(f) => {
                format!("#<procedure>")
            },
            Value::LambdaValue(params, body) => {
                format!("#<procedure>")
            },
            Value::PairValue(boxCar, boxCdr) => {
                let mut s: String = format!("({} ", boxCar.to_string());
                match &**boxCdr {
                    v @ Value::BooleanValue(b) => {
                        format!("{}. {})", s, v.to_string())
                    },
                    v @ Value::NumericValue(n) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::StringValue(string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::SymbolValue(string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::ProcedureValue(string) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::LambdaValue(string1, string2) => {
                        format!("{}. {}", s, v.to_string())
                    }
                    v @ Value::PairValue(car, cdr) => {
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