/// 使用Token进行分析, 返回"值"

use crate::tokenizer::Token;
use crate::value::Value;
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    /// 新建一个parse机
    pub fn new(mut tokens: Vec<Token>) -> Self{
        tokens.reverse();
        Self {tokens}
    }

    /// 使用parse机进行parse
    pub fn parse(&mut self) -> Value {
        let token: Option<Token> = self.tokens.pop();
        match token {
            None => panic!("Unexpected end of string literal."),
            Some(Token::Numeric(n)) => return Value::NumericValue(n),
            Some(Token::Boolean(b)) => return Value::BooleanValue(b),
            Some(Token::String(s)) => return Value::StringValue(s),
            Some(Token::Identifier(i)) => return Value::SymbolValue(i),
            Some(Token::ParL) => {
                let value = self.parse_tails();
                return value;
            }
            Some(Token::ParR) => return Value::SymbolValue(")".to_string()),
            Some(Token::Quote) => return Value::PairValue(
                Box::new(Value::SymbolValue("quote".to_string())),
                Box::new(Value::PairValue(
                    Box::new(self.parse()),
                    Box::new(Value::NilValue)
                ))
            ),
            Some(Token::QuasiQuote) => return Value::PairValue(
                Box::new(Value::SymbolValue("quasiquote".to_string())),
                Box::new(Value::PairValue(
                    Box::new(self.parse()),
                    Box::new(Value::NilValue)
                ))
            ),
            Some(Token::Unquote) => return Value::PairValue(
                Box::new(Value::SymbolValue("unquote".to_string())),
                Box::new(Value::PairValue(
                    Box::new(self.parse()),
                    Box::new(Value::NilValue)
                ))
            ),
            Some(Token::Dot) => unimplemented!("Token::Dot unimplemented."),
        }
    }
    fn parse_tails(&mut self) -> Value {
        let token: Option<Token> = self.tokens.pop();
        match token {
            None => panic!("Unexpected end of string literal."),
            Some(Token::ParR) => return Value::NilValue,
            Some(t) => {
                self.tokens.push(t);
            }
        }
        let car: Value = self.parse();
        let token: Option<Token> = self.tokens.pop();
        match token {
            None => {
                panic!("Unexpected end of string literal.");
            }
            Some(Token::Dot) => {
                let cdr = self.parse();
                let final_check = self.parse();
                if final_check.to_string() != ')'.to_string() {
                    panic!("Unexpected end of string literal.");
                }
                return Value::PairValue(Box::new(car), Box::new(cdr))
            }
            Some(t) => {
                self.tokens.push(t);
                let cdr = self.parse_tails();
                return Value::PairValue(Box::new(car), Box::new(cdr));
            }
        }
    }
}