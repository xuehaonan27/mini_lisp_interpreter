use crate::eval_env::EvalEnv;
use crate::tokenizer::Tokenizer;
use crate::parse::Parser;
use std::rc::Rc;
pub fn test_machine(param: (&str, &str), eval_env: Rc<EvalEnv>) -> () {
    let input: String = param.0.to_string();
    let right_answer = param.1.to_string();
    // let eval_env: EvalEnv = EvalEnv::new();
    let output: String = eval_env.eval(Parser::new(Tokenizer::new(input).tokenize()).parse()).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        panic!()
    }).to_string();
    assert_eq!(output, right_answer);
}