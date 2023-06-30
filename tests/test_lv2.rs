use mini_lisp_interpreter::eval_env::EvalEnv;
use mini_lisp_interpreter::tokenizer::Tokenizer;
use mini_lisp_interpreter::parse::Parser;

fn test_machine(param: (&str, &str)) -> () {
    let input: String = param.0.to_string();
    let right_answer = param.1.to_string();
    let eval_env: EvalEnv = EvalEnv::new();
    let output: String = eval_env.eval(Parser::new(Tokenizer::new(input).tokenize()).parse()).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        panic!()
    }).to_string();
    assert_eq!(output, right_answer);
}

#[test]fn lv2_case1() { let param = ("#f", "#f"); test_machine(param); }
#[test]fn lv2_case2() { let param = ("#t", "#t"); test_machine(param); }
#[test]fn lv2_case3() { let param = ("42", "42"); test_machine(param); }
#[test]fn lv2_case4() { let param = ("+42", "42"); test_machine(param); }
#[test]fn lv2_case5() { let param = ("-42", "-42"); test_machine(param); }
#[test]fn lv2_case6() { let param = ("3.14", "3.140000"); test_machine(param); }
#[test]fn lv2_case7() { let param = ("\"abc\"", "\"abc\""); test_machine(param); }
#[test]fn lv2_case8() { let param = ("\"ab\\\"c\"", "\"ab\\\"c\""); test_machine(param); }