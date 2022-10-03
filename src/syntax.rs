use crate::error::Error;
use crate::token::Token;
use std::fmt;

///
/// 语法树中表达式的枚举类型，方便递归下降分析
///
/// 共有如下表达式：
/// - 变量
/// - 一元表达式
/// - 二元表达式
/// - 函数调用
/// - 字面量
/// - 赋值语句
///
#[derive(Debug, Clone)]
pub enum Expr {
    /// 赋值表达式
    Assign {
        /// 赋值的变量
        name: Token,
        /// 赋值的值
        value: Box<Expr>,
    },
    /// 函数调用表达式
    Call {
        /// 表达式的值
        callee: Box<Expr>,
        /// 函数名
        paren: Token,
        /// 参数列表
        arguments: Vec<Expr>,
    },
    /// 二元表达式
    Binary {
        /// 左操作数
        left: Box<Expr>,
        /// 运算符
        operator: Token,
        /// 右操作数
        right: Box<Expr>,
    },
    /// 字面量
    Literal {
        /// 字面量的值
        value: LiteralValue,
    },
    /// 一元表达式
    Unary {
        /// 运算符
        operator: Token,
        /// 操作数
        right: Box<Expr>,
    },
    /// 变量
    Variable {
        /// 变量名
        name: Token,
    },
}

///
/// 字面量枚举类型，作为表达式中的字面量类型使用
///
/// 共有如下类型：
/// - 数字
/// - 字符串
/// - 布尔值
/// - 空值
///
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
        }
    }
}

///
/// 表达式模块的访问者接口
///
pub mod expr {
    use super::{Expr, LiteralValue};
    use crate::error::Error;
    use crate::token::Token;

    pub trait Visitor<R> {
        fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<R, Error>;
        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<R, Error>;

        fn visit_literal_expr(&self, value: &LiteralValue) -> Result<R, Error>;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<R, Error>;
        fn visit_variable_expr(&mut self, name: &Token) -> Result<R, Error>;
        fn visit_call_expr(
            &mut self,
            callee: &Expr,
            paren: &Token,
            arguments: &Vec<Expr>,
        ) -> Result<R, Error>;
    }
}

///
/// 语法树中语句的枚举类型
///
/// 共有如下语句：
/// - 表达式语句
/// - 打印语句
/// - 变量声明语句
/// - 块语句
/// - 输入字符串语句
/// - 输入数字语句
/// - 循环语句
/// - 条件语句
/// - 函数声明语句
/// - 等待语句
/// - 退出语句
///
#[derive(Clone)]
pub enum Stmt {
    /// 块语句
    Block {
        /// 块语句中的语句列表
        statements: Vec<Stmt>,
    },
    /// 表达式语句
    Expression {
        /// 表达式语句中的表达式
        expression: Expr,
    },
    /// 分支语句
    Branch {
        /// 分支语句中的条件表达式
        condition: Expr,
        /// 分支语句中的执行语句
        then: Box<Stmt>,
    },
    /// 循环语句，无限循环
    Loop {
        /// 循环语句中的执行语句
        body: Box<Stmt>,
    },
    /// 函数声明语句
    Function {
        /// 函数声明语句中的函数名
        name: Token,
        /// 函数声明语句中的参数列表
        params: Vec<Token>,
        /// 函数声明语句中的函数体
        body: Vec<Stmt>,
    },
    /// 打印语句
    Speak {
        /// 打印语句中的表达式
        expression: Expr,
    },
    /// 输入字符串语句
    Input {
        /// 输入字符串语句中的变量名
        input: Token,
    },
    /// 输入数字语句
    Inputn {
        /// 输入数字语句中的变量名
        input: Token,
    },
    /// 停止语句
    Listen {
        /// 停止时间表达式
        time: Expr,
    },
    /// 变量声明语句
    Var {
        /// 变量声明语句中的变量名
        name: Token,
        /// 变量声明语句中的变量值
        initializer: Option<Expr>,
    },
    /// 退出语句
    Exit,
    /// 空语句
    Null,
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn stmt::Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::Branch { condition, then } => visitor.visit_branch_stmt(condition, then),
            Stmt::Loop { body } => visitor.visit_loop_stmt(body),
            Stmt::Speak { expression } => visitor.visit_speak_stmt(expression),
            Stmt::Input { input } => visitor.visit_input_stmt(input),
            Stmt::Inputn { input } => visitor.visit_inputn_stmt(input),
            Stmt::Listen { time } => visitor.visit_listen_stmt(time),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Exit => visitor.visit_exit_stmt(),
            Stmt::Null => unimplemented!(),
        }
    }
}

///
/// 语句模块的访问者接口
///
pub mod stmt {
    use super::{Expr, Stmt};
    use crate::{error::Error, token::Token};

    pub trait Visitor<R> {
        fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<R, Error>;
        fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_function_stmt(
            &mut self,
            name: &Token,
            params: &Vec<Token>,
            body: &Vec<Stmt>,
        ) -> Result<R, Error>;
        fn visit_branch_stmt(&mut self, condition: &Expr, then: &Stmt) -> Result<R, Error>;
        fn visit_loop_stmt(&mut self, body: &Stmt) -> Result<R, Error>;
        fn visit_speak_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_input_stmt(&mut self, name: &Token) -> Result<R, Error>;
        fn visit_inputn_stmt(&mut self, name: &Token) -> Result<R, Error>;
        fn visit_listen_stmt(&mut self, time: &Expr) -> Result<R, Error>;
        fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<R, Error>;
        fn visit_exit_stmt(&mut self) -> Result<R, Error>;
    }
}
