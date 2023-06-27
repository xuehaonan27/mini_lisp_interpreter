use crate::tokenizer::token::Token;
pub const TOKEN_END: [char; 6] = ['(', ')', '\'', '`', ',', '"'];
pub const TOKEN_SPACE: [char; 4] = [' ','\n','\r','\t'];
#[derive(Debug)]
pub struct Tokenizer {
    content_vec: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        let content_vec: Vec<char> = content.chars().collect();
        Self { content_vec, pos: 0}
    }
    fn next_token(&mut self) -> Option<Token> {
        while self.pos < self.content_vec.len() {
            let c = self.content_vec[self.pos];
            match c {
                ';' => {
                    while self.pos < self.content_vec.len() && self.content_vec[self.pos] != '\n' {
                        self.pos += 1;
                    }
                },
                ' '|'\n'|'\r'|'\t' => {self.pos += 1;}
                '(' => { self.pos += 1; return Some(Token::ParL); }
                ')' => { self.pos += 1; return Some(Token::ParR); }
                '\'' => { self.pos += 1; return Some(Token::Quote);}
                '`' => { self.pos += 1; return Some(Token::QuasiQuote);}
                '#' => {
                    self.pos += 1;
                    match self.content_vec[self.pos] {
                        't' => { self.pos += 1; return Some(Token::Boolean(true)) },
                        'f' => { self.pos += 1; return Some(Token::Boolean(false))},
                        _ => panic!("Unexpected end of literal string"),
                    }
                },
                '"' => {
                    let mut string: String = String::new();
                    self.pos += 1;
                    while self.pos < self.content_vec.len() {
                        match self.content_vec[self.pos] {
                            '"' => {
                                self.pos += 1;
                                return Some(Token::String(string));
                            },
                            '\\' => {
                                if self.pos + 1 >= self.content_vec.len() {
                                    panic!("Unexpected end of string literal.");
                                }
                                let c: char = self.content_vec[self.pos + 1];
                                if c == 'n' {
                                    string.push('\n');
                                }
                                else {
                                    string.push(c);
                                }
                                self.pos += 2;
                            },
                            _ => {
                                string.push(self.content_vec[self.pos]);
                                self.pos += 1;
                            },
                        }
                    }
                    panic!("Unexpected end of string literal.");
                },
                _ => {
                    let mut text: String = String::new();
                    let first_char:char = self.content_vec[self.pos];
                    while self.pos < self.content_vec.len() && !TOKEN_SPACE.contains(&self.content_vec[self.pos]) && !TOKEN_END.contains(&self.content_vec[self.pos]) {
                        text.push(self.content_vec[self.pos]);
                        self.pos += 1;
                    }
                    if text == String::from('.') {
                        return Some(Token::Dot);
                    }
                    if first_char.is_digit(10) || first_char == '+' || first_char == '-' || first_char == '.' {
                        match text.parse::<f64>() {
                            Ok(n) => return Some(Token::Numeric(n)),
                            Err(_e) => {},
                        }
                    }
                    return Some(Token::Identifier(text));
                },
            }
        }
        None
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut v: Vec<Token> = Vec::new();
        loop {
            let b = self.next_token();
            if b.is_none() {
                break;
            } else {
                v.push(b.unwrap());
            }
        }
        v
    }
}