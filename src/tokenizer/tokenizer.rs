use regex::Regex;
pub const TOKEN_END: Tuple<str> = ('(', ')', '\'', '`', ',', '"');


struct Tokenizer {
    content: String,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(content: String) -> Self{
        Tokenizer(content, Vec::new());
    }
    fn next_token(&self, pos: isize) -> Token {
        
    }
    fn tokenize(&self) -> () {

    }
}