use mini_lisp_interpreter::{test_machine::test_machine, eval_env::EvalEnv};

#[test]
fn lv3() {
    let eval_env: EvalEnv = EvalEnv::new();
    test_machine(("(define x 42)", "()"), &eval_env);
    test_machine(("x", "42"), &eval_env);
    test_machine(("(define y x)", "()"), &eval_env);
    test_machine(("y", "42"), &eval_env);
    test_machine(("(define x #t)", "()"), &eval_env);
    test_machine(("x", "#t"), &eval_env);
    test_machine(("y", "42"), &eval_env);
    test_machine(("(define y \"abc\")", "()"), &eval_env);
    test_machine(("y", "\"abc\""), &eval_env);
}