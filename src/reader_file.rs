use std::fs::File;
use crate::error::ErrorRead;
use crate::EvalEnv;
use crate::tokenizer::Tokenizer;
use crate::Parser;
use crate::error::ErrorEval;
use crate::io::BufReader;
use std::io::BufRead;
pub struct ReaderFile {
    left_parenthesis_count: isize,
    is_inside_quote: bool,
    is_after_slash: bool,
    is_inside_comment: bool,
    templine: String,
    line: String,
    env: EvalEnv,
    enable: bool,
    have_output_file: bool,
}

impl ReaderFile{
    pub fn new() -> Self {
        Self {
            left_parenthesis_count: 0,
            is_inside_quote: false, 
            is_after_slash: false,
            is_inside_comment: false,
            templine: String::new(),
            line: String::new(),
            env: EvalEnv::new(),
            enable: false,
            have_output_file: false,
        }
    }
    fn open_input_file(&self) -> Result<BufReader<File>, ErrorRead> {
        let file = File::open("file/test.txt").map_err(|_| ErrorRead::FileOpenError)?;
        let reader = BufReader::new(file);
        return Ok(reader);
    }
    fn open_output_file(&mut self) -> Result<File, ErrorRead> {
        let file = File::create("file/output.txt").map_err(|_| ErrorRead::FileOpenError)?;
        self.have_output_file = true;
        return Ok(file);
    }
    /*pub fn readline() -> Result<(), ErrorRead> {
        let mut file = File::open("file/test.txt").map_err(|_| ErrorRead::FileOpenError)?;
        let mut buffer = [0; 1024]; // 设置缓冲区大小

        loop {
            match file.read(&mut buffer) {
                Ok(0) => {
                    // 到达文件末尾
                    break;
                }
                Ok(n) => {
                    // 处理读取的数据
                    let data = &buffer[..n];

                    if let Ok(utf8_str) = std::str::from_utf8(data) {
                        // 将字节数组转换为 UTF-8 字符串
                        let string_data = utf8_str.to_string();
                        println!("{}", string_data);
                    } else {
                        return Err(ErrorRead::Utf8ConversionError);
                    }
                }
                Err(_) => return Err(ErrorRead::StreamFailure),
            }
        }

        Ok(())
    }*/
    fn process_line(&mut self, mut templine: String) -> Result<(), ErrorRead> {
        // println!("Process_line.");
        templine = templine.trim().to_string();
        if templine.is_empty() {
            return Err(ErrorRead::EmptyLine);
        }
        let mut lcount: isize = 0;
        let mut rcount: isize = 0;
        // println!("templine: {}" ,templine);
        for ch in templine.chars() {
            // println!("{ch}");
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
                    // else { self.is_inside_comment = true; } // Added
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
        // println!("{}, {}", lcount, rcount);
        self.left_parenthesis_count += lcount - rcount; // Unstable
        if self.left_parenthesis_count < 0 {
            return Err(ErrorRead::SyntaxFailure);
        }
        self.line += templine.as_str();
        // if self.left_parenthesis_count != 0 { self.line.push('\n'); }
        self.line.push('\n');
        Ok(())
    }
    fn readline(&mut self, reader: &mut BufReader<File>) -> Result<(), ErrorRead> {
        let mut buffer: String = String::new();
        let num_bytes = reader.read_line(&mut buffer);
        // println!("{}", buffer);
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
    fn process(&mut self) -> Result<String, ErrorEval> {
        // println!("{}", self.line);
        // self.line = self.line.trim().to_string();
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
        self.left_parenthesis_count = 0;
    }
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
        let open_output_result = self.open_output_file();
        let mut line_count: isize = 0;
        if self.enable {
            println!("Execute step by step. Press ENTER to continue.");
        }
        loop {
            line_count += 1;
            // println!("{}", line_count);
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
                    // println!("{}", self.line);   
                    // println!("{}", self.left_parenthesis_count);              
                    if self.left_parenthesis_count == 0 {
                        /*if self.enable {
                            todo!();
                        }
                        if self.have_output_file {
                            todo!();
                        }*/
                        let result = self.process();
                        match result {
                            Err(e) => {
                                eprintln!("{:?}", e);
                                self.flush();
                                std::process::exit(127);
                            },
                            Ok(s) => {
                                self.output(s);
                            },
                        }
                        /*if self.have_output_file {
                            todo!();
                        }
                        if self.enable {
                            todo!();
                        }*/
                        self.flush();
                    }
                },
            }
        }
    }
}