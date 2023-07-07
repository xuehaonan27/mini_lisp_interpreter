#![allow(dead_code)]

/// 定义错误类型

use std::error;
use std::fmt;

/// 求值错误类型
#[derive(Debug)]
pub struct ErrorEval {
    pub message: String,
    pub index: usize,
}
impl fmt::Display for ErrorEval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for ErrorEval {}
impl Clone for ErrorEval {
    fn clone(&self) -> Self {
        ErrorEval { message: self.message.clone(), index: self.index}
    }
}

/// 读写错误类型
#[derive(Debug, PartialEq)]
pub enum ErrorRead {
    KeyboardInterrupt,
    FileOpenError,
    SyntaxFailure,
    EmptyLine,
    StreamFailure,
    Utf8ConversionError,
    // FileWriteError,
    EOF,
}