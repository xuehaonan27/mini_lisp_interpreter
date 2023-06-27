use std::io;
mod tokenizer;
mod value;
use crate::tokenizer::Tokenizer;
use value::Value;
fn main() {
    let a = Value::NumericValue(42.0);
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
    println!("{}", f.to_string());
    
    /*let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    println!("{:?}", tokens);*/
     
}
