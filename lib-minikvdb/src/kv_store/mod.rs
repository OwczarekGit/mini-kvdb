use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
};

use self::kv_command::{GetCommand, IncrementCommand, SetCommand};

pub mod kv_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KVStore(HashMap<Key, KVDBValue>);

impl KVDBStore for KVStore {}

impl KVStore {
    pub fn set(&mut self, cmd: impl Into<SetCommand>) -> Result<()> {
        let SetCommand(k, v) = cmd.into();
        self.0.insert(k, v);
        Ok(())
    }

    pub fn get(&self, cmd: impl Into<GetCommand>) -> Result<Option<KVDBValue>> {
        let GetCommand(k) = cmd.into();
        Ok(self.0.get(&k).cloned())
    }

    pub fn increment(&mut self, cmd: impl Into<IncrementCommand>) -> Result<f32> {
        let IncrementCommand(k, v) = cmd.into();
        if let Some(value) = self.0.get_mut(&k) {
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
            self.0.insert(k.to_owned(), KVDBValue::Float(v));
            Ok(v)
        }
    }
}

// Key-Value store.
impl MiniKVDB {
    pub fn set(&self, key: impl Into<Key>, value: impl Into<KVDBValue>) -> Result<()> {
        self.kv.write()?.set(SetCommand(key.into(), value.into()))
    }

    pub fn get(&self, key: impl Into<Key>) -> Result<Option<KVDBValue>> {
        self.kv.read()?.get(GetCommand(key.into()))
    }

    pub fn increment(&self, key: impl Into<Key>, value: impl Into<f32>) -> Result<f32> {
        self.kv
            .write()?
            .increment(IncrementCommand(key.into(), value.into()))
    }
}
