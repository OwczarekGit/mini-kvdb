use crate::minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue};

#[derive(Debug, Clone)]
pub struct SetCommand(pub Key, pub KVDBValue);

#[derive(Debug, Clone)]
pub struct GetCommand(pub Key);

#[derive(Debug, Clone)]
pub struct DeleteCommand(pub Key);

#[derive(Debug, Clone)]
pub struct IncrementCommand(pub Key, pub Increment);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Increment {
    Int(i32),
    Float(f32),
}

impl From<f32> for Increment {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for Increment {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<Increment> for KVDBValue {
    fn from(value: Increment) -> Self {
        match value {
            Increment::Int(v) => KVDBValue::Int(v),
            Increment::Float(v) => KVDBValue::Float(v),
        }
    }
}
