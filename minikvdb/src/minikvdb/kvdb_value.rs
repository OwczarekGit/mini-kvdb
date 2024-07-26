use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::kvdb_key::Key;

#[macro_export]
macro_rules! kvdb_map {
    ($v:ty, $typ:ident) => {
        impl From<$v> for $crate::prelude::KVDBValue {
            fn from(value: $v) -> Self {
                Self::$typ(value.to_owned())
            }
        }

        impl TryFrom<$crate::prelude::KVDBValue> for $v {
            type Error = $crate::error::MiniKVDBError;
            fn try_from(value: $crate::prelude::KVDBValue) -> Result<Self, Self::Error> {
                if let $crate::prelude::KVDBValue::$typ(ref v) = value {
                    Ok(v.to_owned())
                } else {
                    Err($crate::error::MiniKVDBError::WrongFieldType)
                }
            }
        }

        impl TryFrom<&$crate::prelude::KVDBValue> for $v {
            type Error = $crate::error::MiniKVDBError;
            fn try_from(value: &$crate::prelude::KVDBValue) -> Result<Self, Self::Error> {
                if let $crate::prelude::KVDBValue::$typ(ref v) = value {
                    Ok(v.to_owned())
                } else {
                    Err($crate::error::MiniKVDBError::WrongFieldType)
                }
            }
        }
    };
}

pub type KVDBObject = std::collections::HashMap<Key, KVDBValue>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum KVDBValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    #[cfg(feature = "chrono")]
    DateTimeUtc(::chrono::DateTime<::chrono::Utc>),
    #[cfg(feature = "big-types")]
    Long(i64),
    #[cfg(feature = "big-types")]
    Double(f64),
}

impl Display for KVDBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KVDBValue::Int(v) => write!(f, "{v}"),
            KVDBValue::Float(v) => write!(f, "{v}"),
            KVDBValue::Bool(v) => write!(f, "{v}"),
            KVDBValue::String(v) => write!(f, "{v}"),
            #[cfg(feature = "chrono")]
            KVDBValue::DateTimeUtc(v) => write!(f, "{v}"),
            #[cfg(feature = "big-types")]
            KVDBValue::Long(v) => write!(f, "{v}"),
            #[cfg(feature = "big-types")]
            KVDBValue::Double(v) => write!(f, "{v}"),
        }
    }
}

kvdb_map!(i32, Int);
kvdb_map!(f32, Float);
kvdb_map!(bool, Bool);
kvdb_map!(String, String);

impl From<&str> for KVDBValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

#[cfg(feature = "chrono")]
kvdb_map!(chrono::DateTime<chrono::Utc>, DateTimeUtc);

#[cfg(feature = "big-types")]
mod big_types {
    kvdb_map!(i64, Long);
    kvdb_map!(f64, Double);
}

#[macro_export]
macro_rules! values {
    ( $( $v:expr ),* ) => {
        {
            vec![$($v.into(),)*]
        }
    };
}
