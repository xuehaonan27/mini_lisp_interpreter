use std::collections::HashMap;
use std::rc::Rc;
use crate::Value;
pub struct EvalEnv{
    symbol_map: HashMap<String, Value>,
    parent: Rc<EvalEnv>,
}