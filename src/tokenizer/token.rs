/// 定义了token的type
pub enum Token {
    ParL,
    ParR,
    Quote,
    QuasiQuote,
    Unquote,
    Dot,
    Boolean(bool),
    Numeric(f64),
    String(String),
    Identifier(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Boolean_literal_token(b) => format!("(BOOLEAN_LITERAL {} )", self.1),
            Numeric_literal_token(f) => format!("NUMERIC_LITERAL {})", self.1),
            String_literal_token(s) => format!("STRING_LITERAL {:?})", self.1),
            Identifier_token(s) => format!("IDENTIFIER {})", self.1),
            
        }
    }
    fn get_type(&self) -> TokenType {
        self.0
    }
}
impl Token::Boolean_literal_token {
    fn get_value(&self) -> bool {
        self.1
    }
}
impl Token::Numeric_literal_token {
    fn get_value(&self) -> f64 {
        self.1
    }
}
impl Token::String_literal_token {
    fn get_value(&self) -> String {
        self.1
    }
}
impl Token::Identifier_token {
    fn get_value(&self) -> String {
        self.1
    }
}