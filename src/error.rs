use std::convert;
use std::fmt;
use std::io;

use crate::token::{Token, TokenType};

///
/// 输出报错信息
///
/// # 参数列表
/// * line: 报错行数
/// * message: 报错信息
///
/// # 返回值
/// * 无
///
pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
}

///
/// 检测报错位置
///
/// # 参数列表
/// * token: 报错位置
/// * message: 报错信息
///
/// # 返回值
/// * 无
///
pub fn parser_error(token: &Token, message: &str) {
    if token.tpe == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

///
/// 错误的枚举类型
///
#[derive(Debug)]
pub enum Error {
    /// 输入输出错误
    Io(io::Error),
    /// 语法错误
    Parse,
    /// 运行时错误
    Runtime { token: Token, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(underlying) => write!(f, "IoError {}", underlying),
            Error::Parse => write!(f, "ParseError"),
            Error::Runtime { message, .. } => write!(f, "RuntimeError {}", message),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Error"
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
