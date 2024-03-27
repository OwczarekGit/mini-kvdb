use std::fmt::Display;

use crate::error::MiniKVDBError;
use serde::{Deserialize, Serialize};

pub type KVDBObject = std::collections::HashMap<String, KVDBValue>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KVDBValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    #[cfg(feature = "chrono")]
    DateTimeUtc(::chrono::DateTime<::chrono::Utc>),
}

impl Display for KVDBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KVDBValue::Int(v) => write!(f, "{v}"),
            KVDBValue::Float(v) => write!(f, "{v}"),
            KVDBValue::Bool(v) => write!(f, "{v}"),
            KVDBValue::String(v) => write!(f, "\"{v}\""),
            #[cfg(feature = "chrono")]
            KVDBValue::DateTimeUtc(v) => write!(f, "{v}"),
        }
    }
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

#[cfg(feature = "chrono")]
mod chrono {
    use chrono::{DateTime, Utc};

    use super::KVDBValue;
    use crate::error::MiniKVDBError;

    impl From<chrono::DateTime<chrono::Utc>> for KVDBValue {
        fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
            Self::DateTimeUtc(value)
        }
    }
    impl TryFrom<KVDBValue> for DateTime<Utc> {
        type Error = MiniKVDBError;
        fn try_from(value: KVDBValue) -> Result<Self, Self::Error> {
            if let KVDBValue::DateTimeUtc(v) = value {
                Ok(v)
            } else {
                Err(MiniKVDBError::WrongFieldType)
            }
        }
    }

    impl TryFrom<&KVDBValue> for DateTime<Utc> {
        type Error = MiniKVDBError;
        fn try_from(value: &KVDBValue) -> Result<Self, Self::Error> {
            if let KVDBValue::DateTimeUtc(v) = value {
                Ok(v.to_owned())
            } else {
                Err(MiniKVDBError::WrongFieldType)
            }
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
