use std::fmt;
extern crate phf;

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
    Branch,
    Step,
    Exit,
    Input,
    Var,
    True,
    False,

    EOF,
}

// Generated via phf_codegen until proc_macro_hygiene is stable.
include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tpe: TokenType,
    pub lexeme: String,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.tpe {
            TokenType::String { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            TokenType::Number { literal } => write!(f, "Number {:?} {:?}", self.lexeme, literal),
            _ => write!(f, "{:?} {:?}", self.tpe, self.lexeme),
        }
    }
}
