///
/// 定义 dsl 运行的环境
///
pub mod env;
///
/// 定义 dsl 所输出的错误
///
pub mod error;
///
/// 定义 dsl 的函数调用方式
///
pub mod function;
///
/// 定义 dsl 的解释器
pub mod interpreter;
///
/// 定义 dsl 变量的对象类型
///
pub mod object;
///
/// 对 dsl 进行解析
///
pub mod parser;
///
/// 扫入源代码，进行词法分析，处理 token
///
pub mod scanner;
///
/// 定义 dsl 的语法树
///
pub mod syntax;
///
/// 定义 token 类，对词素进行包装，方便词法分析和错误处理
///
pub mod token;
