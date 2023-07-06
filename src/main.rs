use std::env;
mod tokenizer;
mod value;
mod parse;
mod eval_env;
mod special_forms;
mod builtins;
mod error;
mod reader_interact;
mod reader_file;
mod command_line;

fn main() {
    /*let a = Value::NumericValue(42.0);
    let b = Value::BooleanValue(false);
    let c = Value::SymbolValue("eq?".to_string());
    let d = Value::StringValue("Hello".to_string());
    let e = Value::NilValue;
    let f = Value::PairValue(
        Box::new(c.clone()),
        Box::new(Value::PairValue(
            Box::new(a.clone()),
            Box::new(Value::PairValue(
                Box::new(d.clone()),
                Box::new(e.clone())
            ))
        ))
    );
    println!("{}", a.to_string());
    println!("{}", b.to_string());
    println!("{}", c.to_string());
    println!("{}", d.to_string());
    println!("{}", e.to_string());
    // println!("{}", f.to_string());

    let vec = f.to_vector();*/
    /*let eval_env: EvalEnv = EvalEnv::new();
    // println!("{:?}", vec);
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let value = parser.parse();
        //println!("{}", value.to_string());
        //println!("{:?}", value.to_vector());
        let result = eval_env.eval(value);
        println!("{}", result.unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            panic!()
        }).to_string());
    }*/

    /*let mut reader_interact: ReaderInteract = ReaderInteract::new();
    reader_interact.call();
    // readline();
    let mut reader_file: ReaderFile = ReaderFile::new(None, None);
    reader_file.call();*/

    
    let config = command_line::Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = command_line::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
