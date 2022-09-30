pub mod error;
pub mod interpreter;
pub mod parser;
/// 
/// 扫入源代码，进行词法分析，处理 token
/// 
pub mod scanner;
pub mod syntax;
///
/// 定义 token 类，对词素进行包装，方便词法分析和错误处理
/// 
pub mod token;
pub mod object;
pub mod env;
