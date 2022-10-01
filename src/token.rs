use std::fmt;
extern crate phf;

/// 
/// 对 token 字段类型的枚举定义
/// 
/// 分为如下四类：
/// - 单符号token
/// - 多符号token
/// - 字面量
/// - 关键字
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    Minus,
    Plus,
    SemiColon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,

    // Literals.
    Identifier,
    String { literal: String },
    Number { literal: f64 },

    // Keywords.
    Speak,
    Listen,
    Inputn,
    Branch,
    Loop,
    Step,
    Exit,
    Input,
    Var,
    Nil,
    True,
    False,

    EOF,
}

// Generated via phf_codegen until proc_macro_hygiene is stable.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

/// 
/// Token 类型，对词素进行打包
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token 字段类型
    pub tpe: TokenType,
    /// 词素，储存具体词素内容
    pub lexeme: String,
    /// 该词素所在行，方便定位错误位置
    pub line: i32,
}

impl Token {
    pub fn new(tpe: TokenType, lexeme: &str, line: i32) -> Self {
        Self {
            tpe,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

/// 
/// 打印 token 中特定类型的词素的内容
/// 
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.tpe {
            TokenType::String { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            TokenType::Number { literal } => write!(f, "Number {:?} {:?}", self.lexeme, literal),
            _ => write!(f, "{:?} {:?}", self.tpe, self.lexeme),
        }
    }
}
