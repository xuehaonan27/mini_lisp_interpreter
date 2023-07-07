use mini_lisp_interpreter::{test_machine::test_machine, eval_env::EvalEnv};
use std::rc::Rc;
#[test]
fn lv2() {
    let eval_env: EvalEnv = EvalEnv::new();
    test_machine(("#f", "#f"), Rc::new(eval_env.clone()));
    test_machine(("#t", "#t"), Rc::new(eval_env.clone()));
    test_machine(("42", "42"), Rc::new(eval_env.clone()));
    test_machine(("+42", "42"), Rc::new(eval_env.clone()));
    test_machine(("-42", "-42"), Rc::new(eval_env.clone()));
    test_machine(("3.14", "3.14"), Rc::new(eval_env.clone()));
    test_machine(("\"abc\"", "\"abc\""), Rc::new(eval_env.clone()));
    test_machine(("\"ab\\\"c\"", "\"ab\"c\""), Rc::new(eval_env.clone()));
}