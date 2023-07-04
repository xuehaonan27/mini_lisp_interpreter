/// 定义错误类型
use std::error;
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
pub struct ErrorToVector {
    pub message: String,
}
impl fmt::Display for ErrorToVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for ErrorToVector {}
impl Clone for ErrorToVector {
    fn clone(&self) -> Self {
        ErrorToVector { message: self.message.clone() }
    }
}

#[derive(Debug)]
pub struct ErrorEval {
    message: String,
}
impl fmt::Display for ErrorEval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl error::Error for ErrorEval {}
impl Clone for ErrorEval {
    fn clone(&self) -> Self {
        ErrorEval { message: self.message.clone() }
    }
}
#[derive(Debug, PartialEq)]
pub enum ErrorRead {
    KeyboardInterrupt,
    FileOpenError,
    SyntaxFailure,
    EmptyLine,
    StreamFailure,
    Utf8ConversionError,
    EOF,
}