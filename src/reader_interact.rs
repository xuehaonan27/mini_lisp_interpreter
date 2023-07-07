/// 定义了交互模式, 是在命令行解析之后, 用户与解释器内核进行互动的工具之一
/// 允许类似Python IDLE的自动缩进
/// 具备良好的错误处理性能

use crate::error::ErrorRead;
use std::io;
use crate::tokenizer::Tokenizer;
use crate::parse::Parser;
use crate::eval_env::EvalEnv;
use crate::error::ErrorEval;
use std::io::Write;
use std::rc::Rc;

/// 定义了交互模式自动机
pub struct ReaderInteract {
    space_buffer: Vec<usize>,
    buffer_modify_pos: isize,
    is_inside_quote: bool,
    is_after_slash: bool,
    is_inside_comment: bool,
    templine: String,
    line: String,
    env: Rc<EvalEnv>,
}

impl ReaderInteract {
    /// 新建交互模式
    pub fn new() -> Self {
        Self {
            space_buffer: Vec::new(), 
            buffer_modify_pos: -1, 
            is_inside_quote: false, 
            is_after_slash: false,
            is_inside_comment: false,
            templine: String::new(),
            line: String::new(),
            env: Rc::new(EvalEnv::new()),
        }
    }

    /// 读入一行文本, 检测是否已经是一个完整的表达式, 并且处理自动缩进
    fn readline(&mut self) -> Result<(), ErrorRead> {
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
        for ch in self.templine.chars() {
            match ch {
                '\n' => {
                    if self.is_inside_comment { self.is_inside_comment = false; continue; }
                },
                '(' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else { 
                        self.buffer_modify_pos += 1; self.space_buffer.push(1); 
                    }
                },
                ')' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else {
                        if self.space_buffer.is_empty() {return Err(ErrorRead::SyntaxFailure)}
                        self.buffer_modify_pos -= 1;
                        self.space_buffer.pop();
                    }
                },
                ';' => {
                    if self.is_inside_comment { continue; } // Added 
                    else { self.is_inside_comment = true; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote { self.space_buffer[self.buffer_modify_pos as usize] += 1; } // 检查bound
                    else {
                        self.is_inside_comment = true;
                    }
                },
                '"' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    else {
                        if self.is_inside_quote { self.is_inside_quote = false; }
                        else { self.is_inside_quote = true; }
                    }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                '\\' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_inside_quote {
                        if self.is_after_slash { self.is_after_slash = false; }
                        else { self.is_after_slash = true; }
                    }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                'n' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_inside_quote && self.is_after_slash { self.is_after_slash = false; }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
                _ => {
                    if self.is_inside_comment { continue; } // Added
                    if self.buffer_modify_pos == -1 { break; }
                    self.space_buffer[self.buffer_modify_pos as usize] += 1; // 检查bound
                },
            }
        }
        self.line.push_str(&self.templine);
        self.line.push('\n');
        return Ok(());
    }

    /// 用于打印交互模式中所需要的行提示符'>>>'与'...'并打印正确数量的空格完成缩进
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

    /// 检测到一个完整表达式之后进行处理
    fn process(&self) -> Result<String, ErrorEval> {
        let mut tokenizer: Tokenizer = Tokenizer::new(self.line.clone());
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let value = parser.parse();
        let result = self.env.clone().eval(value)?;
        Ok(result.to_string())
    }

    /// 处理输出
    fn output(&self, result: String) -> () {
        if result == "()".to_string() {
            return;
        }
        println!("{}", result);
    }

    /// 清空交互模式自动机的状态
    fn flush(&mut self) -> () {
        self.line.clear();
        self.templine.clear();
        self.is_inside_quote = false;
        self.is_after_slash = false;
        self.is_inside_comment = false;
        self.space_buffer.clear();
        self.buffer_modify_pos = -1;
    }

    /// 调用交互模式
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
                        eprintln!("Error:\n{}", result.err().unwrap());
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