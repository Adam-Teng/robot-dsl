use crate::function::Function;

///
/// 定义 dsl 对象的枚举类型
///
#[derive(Debug, Clone)]
pub enum Object {
    /// 布尔值
    Boolean(bool),
    /// 函数
    Callable(Function),
    /// 空值
    Null,
    /// 数字
    Number(f64),
    /// 字符串
    String(String),
}

impl Object {
    ///
    /// 判断两个对象是否相等
    ///
    /// # 参数列表
    /// * other: 另一个对象
    ///
    /// # 返回值
    /// * 是否相等
    ///
    pub fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (_, Object::Null) => false,
            (Object::Null, _) => false,
            (Object::Boolean(left), Object::Boolean(right)) => left == right,
            (Object::Number(left), Object::Number(right)) => left == right,
            (Object::String(left), Object::String(right)) => left.eq(right),
            _ => false,
        }
    }
}
