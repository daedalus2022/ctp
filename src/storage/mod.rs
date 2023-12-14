use crate::{CtpError, Value};

pub mod memory;

/// 持久化数据
pub trait Storage {
    /// 插入数据
    fn hset(&self, table: &str, key: &str, value: Value) -> Result<Value, CtpError>;
}
