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
    #[cfg(feature = "big-types")]
    Long(i64),
    #[cfg(feature = "big-types")]
    Double(f64),
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

#[cfg(feature = "big-types")]
mod big_types {
    use super::Increment;
    impl From<i64> for Increment {
        fn from(value: i64) -> Self {
            Self::Long(value)
        }
    }
    impl From<f64> for Increment {
        fn from(value: f64) -> Self {
            Self::Double(value)
        }
    }
}

impl From<Increment> for KVDBValue {
    fn from(value: Increment) -> Self {
        match value {
            Increment::Int(v) => KVDBValue::Int(v),
            Increment::Float(v) => KVDBValue::Float(v),
            #[cfg(feature = "big-types")]
            Increment::Long(v) => KVDBValue::Long(v),
            #[cfg(feature = "big-types")]
            Increment::Double(v) => KVDBValue::Double(v),
        }
    }
}
