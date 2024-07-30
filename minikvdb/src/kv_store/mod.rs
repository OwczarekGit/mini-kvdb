use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! impl_increment {
    ($val:ident, $cmd_val:ident, $in:ident, $cast:ty) => {{
        *$val += match $cmd_val {
            Increment::Int(v) => v as $cast,
            Increment::Float(v) => v as $cast,
            #[cfg(feature = "big-types")]
            Increment::Long(v) => v as $cast,
            #[cfg(feature = "big-types")]
            Increment::Double(v) => v as $cast,
        };

        Ok(Increment::$in(*$val))
    }};
}

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
};

use self::kv_command::{DeleteCommand, GetCommand, Increment, IncrementCommand, SetCommand};

pub mod kv_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KVStore(HashMap<Key, KVDBValue>);

impl KVDBStore for KVStore {}

impl KVStore {
    pub fn set(&mut self, cmd: impl Into<SetCommand>) -> Option<KVDBValue> {
        let SetCommand(k, v) = cmd.into();
        self.0.insert(k, v)
    }

    pub fn get(&self, cmd: impl Into<GetCommand>) -> Option<KVDBValue> {
        let GetCommand(k) = cmd.into();
        self.0.get(&k).cloned()
    }

    pub fn delete(&mut self, cmd: impl Into<DeleteCommand>) -> Option<KVDBValue> {
        let DeleteCommand(k) = cmd.into();
        self.0.remove(&k)
    }

    pub fn increment(&mut self, cmd: impl Into<IncrementCommand>) -> Result<Increment> {
        let IncrementCommand(k, v) = cmd.into();
        if let Some(value) = self.0.get_mut(&k) {
            match value {
                KVDBValue::Int(val) => impl_increment!(val, v, Int, i32),
                KVDBValue::Float(val) => impl_increment!(val, v, Float, f32),
                #[cfg(feature = "big-types")]
                KVDBValue::Long(val) => impl_increment!(val, v, Long, i64),
                #[cfg(feature = "big-types")]
                KVDBValue::Double(val) => impl_increment!(val, v, Double, f64),
                _ => Err(MiniKVDBError::CannotIncrement),
            }
        } else {
            self.0.insert(k.to_owned(), v.into());
            Ok(v)
        }
    }
}

// Key-Value store.
impl MiniKVDB {
    pub fn set(
        &self,
        key: impl Into<Key>,
        value: impl Into<KVDBValue>,
    ) -> Result<Option<KVDBValue>> {
        Ok(self.kv.write()?.set(SetCommand(key.into(), value.into())))
    }

    pub fn get(&self, key: impl Into<Key>) -> Result<Option<KVDBValue>> {
        Ok(self.kv.read()?.get(GetCommand(key.into())))
    }

    pub fn del(&self, key: impl Into<Key>) -> Result<Option<KVDBValue>> {
        Ok(self.kv.write()?.delete(DeleteCommand(key.into())))
    }

    pub fn increment(&self, key: impl Into<Key>, value: impl Into<Increment>) -> Result<Increment> {
        self.kv
            .write()?
            .increment(IncrementCommand(key.into(), value.into()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_db() -> KVStore {
        KVStore::default()
    }

    #[test]
    fn sets_value() {
        let mut db = test_db();

        let ins = db.set(SetCommand("name".into(), "tom".into()));
        let replaced = db.set(SetCommand("name".into(), "bob".into()));

        assert!(replaced.is_some());
        assert_eq!(replaced.unwrap(), KVDBValue::String("tom".into()));

        assert_eq!(ins, None);
        assert_eq!(
            *db.0.get("name").unwrap(),
            KVDBValue::String("bob".to_string())
        );
    }

    #[test]
    fn gets_value() {
        let mut db = test_db();
        let _ = db.set(SetCommand("name".into(), "tom".into()));

        let e = db.get(GetCommand("name".into()));

        assert!(e.is_some());
        assert_eq!(e.unwrap(), KVDBValue::String("tom".into()));
    }

    #[test]
    fn deletes_value() {
        let mut db = test_db();

        let _ = db.set(SetCommand("name".into(), "tom".into()));
        let _ = db.set(SetCommand("name1".into(), "tom1".into()));
        let _ = db.set(SetCommand("name2".into(), "tom2".into()));

        let deleted = db.delete(DeleteCommand("name1".into()));
        let empty = db.delete(DeleteCommand("name10".into()));

        assert_eq!(empty, None);

        assert!(deleted.is_some());
        assert_eq!(deleted.unwrap(), KVDBValue::String("tom1".into()));
    }

    #[test]
    fn increment_empty_keys() {
        let mut db = test_db();
        let inc_int = db
            .increment(IncrementCommand("a".into(), 5.into()))
            .unwrap();

        let inc_float = db
            .increment(IncrementCommand("b".into(), 8.5f32.into()))
            .unwrap();

        assert_eq!(inc_int, Increment::Int(5));
        assert_eq!(inc_float, Increment::Float(8.5));
    }

    #[test]
    fn increments_values_with_correct_types() {
        let mut db = test_db();
        let _ = db.set(SetCommand("a".into(), 1.into()));
        let _ = db.set(SetCommand("b".into(), 10.0f32.into()));
        let _ = db.set(SetCommand("c".into(), "john".into()));

        let inc_int_with_int = db
            .increment(IncrementCommand("a".into(), 4.into()))
            .unwrap();

        let inc_int_with_float = db
            .increment(IncrementCommand("a".into(), 4.9.into()))
            .unwrap();

        let inc_float_with_float = db
            .increment(IncrementCommand("b".into(), 4.9.into()))
            .unwrap();

        let inc_float_with_int = db
            .increment(IncrementCommand("b".into(), 5.into()))
            .unwrap();

        let inc_wrong_type = db.increment(IncrementCommand("c".into(), 1.into()));

        assert!(inc_wrong_type.is_err());

        assert_eq!(inc_int_with_int, Increment::Int(5));
        assert_eq!(inc_int_with_float, Increment::Int(9));

        assert_eq!(inc_float_with_float, Increment::Float(14.9));
        assert_eq!(inc_float_with_int, Increment::Float(19.9));
    }
}
