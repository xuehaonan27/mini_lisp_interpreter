use crate::Value;
use crate::eval_env::EvalEnv;

pub type SpecialForm = fn(Vec<Value>, &EvalEnv) -> Value;
/*static SPECIAL_FORMS: HashMap<String, SpecialForm> = HashMap::from([
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
]);*/

pub fn define_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn quote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn if_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn and_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn or_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn lambda_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn cond_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn begin_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn let_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn quasiquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn unquote_form(args: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }