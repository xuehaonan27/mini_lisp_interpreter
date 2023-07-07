/// 定义了文件模式, 是在命令行解析之后, 用户与解释器内核进行互动的工具之一

use std::fs::File;
use std::io::Write;
use crate::error::ErrorRead;
use crate::eval_env::EvalEnv;
use crate::tokenizer::Tokenizer;
use crate::parse::Parser;
use crate::error::ErrorEval;
use std::io::{BufReader, BufRead, BufWriter};
use std::rc::Rc;
use std::io;

/// 定义了文件模式自动机
pub struct ReaderFile {
    left_parenthesis_count: isize,
    is_inside_quote: bool,
    is_after_slash: bool,
    is_inside_comment: bool,
    templine: String,
    line: String,
    env: Rc<EvalEnv>,
    have_output_file: bool,
    input_file_name: Option<String>,
    output_file_name: Option<String>,
}

impl ReaderFile{
    /// 新建文件模式
    pub fn new(input_file_name: Option<String>, output_file_name: Option<String>) -> Self {
        let have_output_file: bool;
        if output_file_name.is_none() {
            have_output_file = false;
        }
        else {
            have_output_file = true;
        }
        Self {
            left_parenthesis_count: 0,
            is_inside_quote: false, 
            is_after_slash: false,
            is_inside_comment: false,
            templine: String::new(),
            line: String::new(),
            env: Rc::new(EvalEnv::new()),
            have_output_file,
            input_file_name,
            output_file_name,
        }
    }

    /// 打开输入文件
    fn open_input_file(&self) -> Result<BufReader<File>, ErrorRead> {
        let file = File::open(self.input_file_name.as_ref().unwrap()).map_err(|_| ErrorRead::FileOpenError)?;
        let reader = BufReader::new(file);
        return Ok(reader);
    }

    /// 打开输出文件
    fn open_output_file(&mut self) -> Result<BufWriter<File>, ErrorRead> {
        let file = File::create(self.output_file_name.as_ref().unwrap()).map_err(|_| ErrorRead::FileOpenError)?;
        let writer = BufWriter::new(file);
        return Ok(writer);
    }
    
    /// 对读入的一行文本进行处理, 检查是否已经是一个完整的表达式
    fn process_line(&mut self, mut templine: String) -> Result<(), ErrorRead> {
        templine = templine.trim().to_string();
        if templine.is_empty() {
            return Err(ErrorRead::EmptyLine);
        }
        let mut lcount: isize = 0;
        let mut rcount: isize = 0;
        for ch in templine.chars() {
            match ch {
                '\n' => {
                    if self.is_inside_comment { self.is_inside_comment = false; continue; }
                },
                '(' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote == false { lcount += 1; } // 检查bound
                },
                ')' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote == false { rcount += 1; } // 检查bound
                },
                ';' => {
                    if self.is_inside_comment { continue; } // Added 
                    if self.is_after_slash { self.is_after_slash = false; }
                    if self.is_inside_quote == false { self.is_inside_comment = true; } // 检查bound
                },
                '"' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_after_slash { self.is_after_slash = false; }
                    else {
                        if self.is_inside_quote { self.is_inside_quote = false; }
                        else { self.is_inside_quote = true; }
                    }
                },
                '\\' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_inside_quote {
                        if self.is_after_slash { self.is_after_slash = false; }
                        else { self.is_after_slash = true; }
                    }
                },
                'n' => {
                    if self.is_inside_comment { continue; } // Added
                    if self.is_inside_quote && self.is_after_slash { self.is_after_slash = false; }
                },
                _ => {
                    if self.is_after_slash { self.is_after_slash = false; }
                },
            }
        }
        self.left_parenthesis_count += lcount - rcount;
        if self.left_parenthesis_count < 0 {
            return Err(ErrorRead::SyntaxFailure);
        }
        self.line += templine.as_str();

        self.line.push('\n');
        Ok(())
    }

    /// 读入一行文本
    fn readline(&mut self, reader: &mut BufReader<File>) -> Result<(), ErrorRead> {
        let mut buffer: String = String::new();
        let num_bytes = reader.read_line(&mut buffer);
  
        match num_bytes {
            Ok(0) => {
                // 到达文件末尾
                return Err(ErrorRead::EOF);
            },
            Ok(_) => {
                self.process_line(buffer)?
            },
            Err(_) => {
                return Err(ErrorRead::Utf8ConversionError);
            },
        }
        Ok(())
    }

    /// 在检测到一个完整表达式之后进行处理
    fn process(&mut self) -> Result<String, ErrorEval> {
        let mut tokenizer: Tokenizer = Tokenizer::new(self.line.clone());
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let value = parser.parse();
        let result = self.env.clone().eval(value)?;
        Ok(result.to_string())
    }

    /// 处理输出
    fn output(&self, result: String, writer: &mut Option<BufWriter<File>>) -> io::Result<()> {
        if result == "()".to_string() {
            return Ok(());
        }
        println!("{}",self.have_output_file);
        if self.have_output_file {
            
            let writer: &mut BufWriter<File> = writer.as_mut().unwrap();
            
            write!(writer, "{result}")?;
            writer.flush()?;
            return Ok(());
        }
        else {
            println!("{}", result);
        }
        return Ok(())
    }
    
    /// 清空文件模式自动机的状态
    fn flush(&mut self) -> () {
        self.line.clear();
        self.templine.clear();
        self.is_inside_quote = false;
        self.is_after_slash = false;
        self.is_inside_comment = false;
        self.left_parenthesis_count = 0;
    }

    /// 调用文件模式
    pub fn call(&mut self) -> () {
        let open_input_result = self.open_input_file();
        let mut reader: BufReader<File>;
        match open_input_result {
            Err(e) => {
                eprintln!("{:?}", e);
                self.flush();
                std::process::exit(127);
            },
            Ok(r) => {
                reader = r;
            }
        }
        let mut writer: Option<BufWriter<File>> = None;
        if self.have_output_file {
            let open_output_result = self.open_output_file();
            match open_output_result {
                Err(e) => {
                    eprintln!("{:?}", e);
                    self.flush();
                    std::process::exit(127);
                }
                Ok(w) => {
                    writer = Some(w);
                }
            }
        }
        
        loop {
            let read_status = self.readline(&mut reader);
            match read_status {
                Err(ErrorRead::EOF) => {
                    return;
                },
                Err(ErrorRead::EmptyLine) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    self.flush();
                    std::process::exit(127);
                },
                Ok(()) => {              
                    if self.left_parenthesis_count == 0 {
                        let result = self.process();
                        match result {
                            Err(e) => {
                                eprintln!("{}", e);
                                self.flush();
                                std::process::exit(127);
                            },
                            Ok(s) => {
                                self.output(s, &mut writer).unwrap_or_else(|e|{
                                    eprintln!("{}", e);
                                    self.flush();
                                    std::process::exit(127);
                                });
                            },
                        }
                        self.flush();
                    }
                },
            }
        }
    }
}