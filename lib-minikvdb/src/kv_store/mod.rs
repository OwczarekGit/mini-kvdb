use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
};

use self::kv_command::{GetCommand, IncrementCommand, SetCommand};

pub mod kv_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KVStore(HashMap<String, KVDBValue>);

impl KVDBStore for KVStore {}

impl KVStore {
    pub fn set<'a>(store: &mut Self, cmd: impl Into<SetCommand<'a>>) -> Result<()> {
        let cmd = cmd.into();
        store.0.insert(cmd.0.to_owned(), cmd.1);
        Ok(())
    }

    pub fn get<'a>(store: &Self, cmd: impl Into<GetCommand<'a>>) -> Result<Option<KVDBValue>> {
        Ok(store.0.get(cmd.into().0).cloned())
    }

    pub fn increment<'a>(store: &mut Self, cmd: impl Into<IncrementCommand<'a>>) -> Result<f32> {
        let IncrementCommand(k, v) = cmd.into();
        if let Some(value) = store.0.get_mut(k) {
            match value {
                KVDBValue::Int(val) => {
                    *val += v as i32;
                    Ok(*val as f32)
                }
                KVDBValue::Float(val) => {
                    *val += v;
                    Ok(*val)
                }
                _ => Err(MiniKVDBError::CannotIncrement),
            }
        } else {
            store.0.insert(k.to_owned(), KVDBValue::Float(v));
            Ok(v)
        }
    }
}

// Key-Value store.
impl MiniKVDB {
    pub fn set<'a>(&self, key: impl Into<&'a str>, value: impl Into<KVDBValue>) -> Result<()> {
        KVStore::set(&mut *self.kv.write()?, (key.into(), value.into()))
    }

    pub fn get<'a>(&self, cmd: impl Into<GetCommand<'a>>) -> Result<Option<KVDBValue>> {
        KVStore::get(&*self.kv.read()?, cmd)
    }

    pub fn increment<'a>(&self, key: impl Into<&'a str>, value: impl Into<f32>) -> Result<f32> {
        KVStore::increment(&mut *self.kv.write()?, (key.into(), value.into()))
    }
}
