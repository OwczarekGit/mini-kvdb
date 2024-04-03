use std::{borrow::Borrow, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Key(pub String);

impl Key {
    pub fn namespaced(ns: impl Into<Key>, key: impl Into<Key>) -> Self {
        Self(format!("{}:{}", ns.into().0, key.into().0))
    }
}

impl Borrow<str> for Key {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<String> for Key {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl<T: Display> From<T> for Key {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
