use mini_lisp_interpreter::{test_machine::test_machine, eval_env::EvalEnv};

#[test]
fn lv4() {
    let eval_env: EvalEnv = EvalEnv::new();
    test_machine(("+", "#<proc>"), &eval_env);
    test_machine(("#t", "#t"), &eval_env);
    test_machine(("42", "42"), &eval_env);
    test_machine(("+42", "+42"), &eval_env);
    test_machine(("-42", "-42"), &eval_env);
    test_machine(("3.14", "3.14"), &eval_env);
    test_machine(("\"abc\"", "\"abc\""), &eval_env);
    test_machine(("\"ab\\\"c\"", "\"ab\\\"c\""), &eval_env);
}