use crate::error::ErrorRead;
use std::io;
use crate::tokenizer::Tokenizer;
use crate::Parser;
use crate::EvalEnv;
use crate::error::ErrorEval;
use std::io::Write;
pub struct ReaderInteract {
    space_buffer: Vec<usize>,
    buffer_modify_pos: isize,
    is_inside_quote: bool,
    is_after_slash: bool,
    is_inside_comment: bool,
    templine: String,
    line: String,
    env: EvalEnv,
    // is_after_sharp: bool,
    // is_inside_multi_line_note: bool,
}

impl ReaderInteract {
    pub fn new() -> Self {
        Self{
            space_buffer: Vec::new(), 
            buffer_modify_pos: -1, 
            is_inside_quote: false, 
            is_after_slash: false,
            is_inside_comment: false,
            templine: String::new(),
            line: String::new(),
            env: EvalEnv::new(),
        }
    }
    fn readline(&mut self) -> Result<(), ErrorRead> {
        // let mut input = String::new();
        self.templine.clear();
        let code = io::stdin().read_line(&mut self.templine);
        // println!("LINE: {}", self.templine);
        match code {
            Ok(n) if n == 0 => std::process::exit(0),
            Ok(_) => (),
            Err(_) => return Err(ErrorRead::StreamFailure),
        }
        if self.templine.len() == 1 && self.templine.clone().pop().unwrap() == '\n' {
            return Err(ErrorRead::KeyboardInterrupt);
        }
        // println!("LINE: {}", self.templine);
        // for (index, ch) in self.templine.chars().enumerate() {
        for ch in self.templine.chars() {
            // println!("{}", ch);
            match ch {
                /*'\n' => {
                    if self.is_inside_comment { self.is_inside_comment = false; continue; }
                },*/
                '(' => {
                    // if self.is_inside_comment { continue; }
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else { 
                        // println!("Before push: {:?}", self.space_buffer);
                        self.buffer_modify_pos += 1; self.space_buffer.push(1); 
                        // println!("After push: {:?}", self.space_buffer);
                    }
                },
                ')' => {
                    // if self.is_inside_comment { continue; }
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else {
                        if self.space_buffer.is_empty() {return Err(ErrorRead::SyntaxFailure)}
                        /*if self.space_buffer.pop().is_some() {
                            self.buffer_modify_pos -= 1;
                        }
                        else {
                            return Err(ErrorRead::SyntaxFailure);
                        }*/
                        // println!("Before Pop: {:?}", self.space_buffer);
                        self.buffer_modify_pos -= 1;
                        self.space_buffer.pop();
                        // println!("After Pop: {:?}", self.space_buffer);
                    }
                },
                ';' => {
                    // if self.is_inside_comment { continue; }
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else { todo!(); }
                },
                '"' => {
                    if self.is_after_slash { self.is_after_slash = false; }
                    else {
                        if self.is_inside_quote { self.is_inside_quote = false; }
                        else { self.is_inside_quote = true; }
                    }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                '\\' => {
                    if self.is_inside_quote {
                        if self.is_after_slash { self.is_after_slash = false; }
                        else { self.is_after_slash = true; }
                    }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                'n' => {
                    if self.is_inside_quote && self.is_after_slash { self.is_after_slash = false; }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                _ => {
                    if self.buffer_modify_pos == -1 { break; }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
            }
            // println!("Here it is: {:?}", self.space_buffer);
        }
        self.line.push_str(&self.templine);
        self.line.push('\n');
        return Ok(());
    }
    fn printline(&mut self) -> () {
        let mut result: usize = 0;
        for s in &self.space_buffer {
            result += s;
        }
        if result == 0 {
            print!(">>> ");
            io::stdout().flush().unwrap();
        }
        else {
            print!("... ");
            let spaces: String = std::iter::repeat(' ').take(result).collect();
            print!("{}", spaces);
            io::stdout().flush().unwrap();
        }
    }
    fn process(&self) -> Result<String, ErrorEval> {
        let mut tokenizer: Tokenizer = Tokenizer::new(self.line.clone());
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let value = parser.parse();
        let result = self.env.eval(value)?;
        Ok(result.to_string())
    }
    fn output(&self, result: String) -> () {
        if result == "()".to_string() {
            return;
        }
        println!("{}", result);
    }
    fn flush(&mut self) -> () {
        self.line.clear();
        self.templine.clear();
        self.is_inside_quote = false;
        self.is_after_slash = false;
        self.is_inside_comment = false;
        self.space_buffer.clear();
        self.buffer_modify_pos = -1;
    }
    pub fn call(&mut self) -> () {
        loop {
            self.printline();
            let read_status = self.readline();
            if read_status.is_ok() {
                if self.space_buffer.is_empty() {
                    let result = self.process();
                    if result.is_ok() {
                        self.output(result.ok().unwrap());
                        self.flush();
                    }
                    else {
                        eprintln!("Error: {:?}", result.err().unwrap());
                        self.flush();
                    }
                }
            }
            else {
                let err: ErrorRead = read_status.err().unwrap();
                if err == ErrorRead::KeyboardInterrupt {
                    println!("Input interrupted with a pure return.");
                    self.flush();
                    continue;
                }
                eprintln!("Error: {:?}", err);
                self.flush();
            }
        }
    }
}