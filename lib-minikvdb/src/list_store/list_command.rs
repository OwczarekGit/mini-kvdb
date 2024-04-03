use crate::minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue};

#[derive(Debug, Clone)]
pub struct PushFrontCommand(pub Key, pub Vec<KVDBValue>);

#[derive(Debug, Clone)]
pub struct PopFrontCommand(pub Key);

impl<T: Into<Key>> From<T> for PopFrontCommand {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct PushBackCommand(pub Key, pub Vec<KVDBValue>);

#[derive(Debug, Clone)]
pub struct PopBackCommand(pub Key);

impl<T: Into<Key>> From<T> for PopBackCommand {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct ListRangeCommand(pub Key, pub i32, pub i32);

impl From<Key> for ListRangeCommand {
    fn from(value: Key) -> Self {
        Self(value, 0, -1)
    }
}

impl<K: Into<Key>> From<(K,)> for ListRangeCommand {
    fn from(value: (K,)) -> Self {
        Self(value.0.into(), 0, -1)
    }
}

impl<K, V> From<(K, V)> for ListRangeCommand
where
    K: Into<Key>,
    V: Into<i32>,
{
    fn from(value: (K, V)) -> Self {
        Self(value.0.into(), value.1.into(), -1)
    }
}

impl<K, C, V> From<(K, C, V)> for ListRangeCommand
where
    K: Into<Key>,
    C: Into<i32>,
    V: Into<i32>,
{
    fn from(value: (K, C, V)) -> Self {
        Self(value.0.into(), value.1.into(), value.2.into())
    }
}

#[derive(Debug, Clone)]
pub struct ListLenCommmand(pub Key);

#[derive(Debug, Clone)]
pub struct ListContainsValueCommand(pub Key, pub KVDBValue);

#[derive(Debug, Clone)]
pub struct ListRemoveCommand(pub Key, pub i32, pub KVDBValue);

impl<K, V> From<(K, V)> for ListRemoveCommand
where
    K: Into<Key>,
    V: Into<KVDBValue>,
{
    fn from(value: (K, V)) -> Self {
        Self(value.0.into(), 0, value.1.into())
    }
}

impl<K, C, V> From<(K, C, V)> for ListRemoveCommand
where
    K: Into<Key>,
    C: Into<i32>,
    V: Into<KVDBValue>,
{
    fn from(value: (K, C, V)) -> Self {
        Self(value.0.into(), value.1.into(), value.2.into())
    }
}
