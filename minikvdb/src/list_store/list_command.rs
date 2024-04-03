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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ListRangeCommand(pub Key, pub ListRangeOption);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ListRangeOption {
    Whole,
    FromIndex(usize),
    FromIndexWithLen(usize, usize),
}

impl<K> From<(K,)> for ListRangeCommand
where
    K: Into<Key>,
{
    fn from(value: (K,)) -> Self {
        Self(value.0.into(), ListRangeOption::Whole)
    }
}

impl<K, V> From<(K, V)> for ListRangeCommand
where
    K: Into<Key>,
    V: Into<usize>,
{
    fn from(value: (K, V)) -> Self {
        Self(value.0.into(), ListRangeOption::FromIndex(value.1.into()))
    }
}

impl<K, C, V> From<(K, C, V)> for ListRangeCommand
where
    K: Into<Key>,
    C: Into<usize>,
    V: Into<usize>,
{
    fn from(value: (K, C, V)) -> Self {
        Self(
            value.0.into(),
            ListRangeOption::FromIndexWithLen(value.1.into(), value.2.into()),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ListLenCommmand(pub Key);

#[derive(Debug, Clone)]
pub struct ListContainsValueCommand(pub Key, pub KVDBValue);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ListRemoveCommand(pub Key, pub ListRemoveOption);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ListRemoveOption {
    All(KVDBValue),
    Count(usize, KVDBValue),
}

impl<K, V> From<(K, V)> for ListRemoveCommand
where
    K: Into<Key>,
    V: Into<KVDBValue>,
{
    fn from(value: (K, V)) -> Self {
        Self(value.0.into(), ListRemoveOption::All(value.1.into()))
    }
}

impl<K, C, V> From<(K, C, V)> for ListRemoveCommand
where
    K: Into<Key>,
    C: Into<usize>,
    V: Into<KVDBValue>,
{
    fn from(value: (K, C, V)) -> Self {
        Self(
            value.0.into(),
            ListRemoveOption::Count(value.1.into(), value.2.into()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_remove_all_command_from_key_and_value_tuple() {
        let cmd: ListRemoveCommand = ("some_key", 123).into();
        assert_eq!(
            cmd,
            ListRemoveCommand("some_key".into(), ListRemoveOption::All(123.into()))
        );
    }

    #[test]
    fn creates_remove_count_command_from_key_and_value_triple() {
        let cmd: ListRemoveCommand = ("some_key", 5_usize, 281).into();
        assert_eq!(
            cmd,
            ListRemoveCommand("some_key".into(), ListRemoveOption::Count(5, 281.into()))
        );
    }
}
