use crate::error::MiniKVDBError;
use serde::{Deserialize, Serialize};

pub type KVDBObject = std::collections::HashMap<String, KVDBValue>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KVDBValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

impl From<i32> for KVDBValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for KVDBValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for KVDBValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for KVDBValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for KVDBValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl TryFrom<KVDBValue> for i32 {
    type Error = MiniKVDBError;
    fn try_from(value: KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Int(v) = value {
            Ok(v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<KVDBValue> for f32 {
    type Error = MiniKVDBError;
    fn try_from(value: KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Float(v) = value {
            Ok(v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<KVDBValue> for bool {
    type Error = MiniKVDBError;
    fn try_from(value: KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Bool(v) = value {
            Ok(v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<KVDBValue> for String {
    type Error = MiniKVDBError;
    fn try_from(value: KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::String(v) = value {
            Ok(v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<&KVDBValue> for i32 {
    type Error = MiniKVDBError;
    fn try_from(value: &KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Int(v) = value {
            Ok(*v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<&KVDBValue> for f32 {
    type Error = MiniKVDBError;
    fn try_from(value: &KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Float(v) = value {
            Ok(*v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<&KVDBValue> for bool {
    type Error = MiniKVDBError;
    fn try_from(value: &KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::Bool(v) = value {
            Ok(*v)
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

impl TryFrom<&KVDBValue> for String {
    type Error = MiniKVDBError;
    fn try_from(value: &KVDBValue) -> Result<Self, Self::Error> {
        if let KVDBValue::String(v) = value {
            Ok(v.clone())
        } else {
            Err(MiniKVDBError::WrongFieldType)
        }
    }
}

#[macro_export]
macro_rules! values {
    ( $( $v:expr ),* ) => {
        {
            let mut vec = vec![];
            $(
                vec.push($v.into());
            )*
            vec
        }
    };
}
