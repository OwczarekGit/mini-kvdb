use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
    prelude::KVDBObject,
};

use self::map_command::{DeleteCommand, GetAllCommand, GetCommand, GetObjectCommand, SetCommand};

pub mod map_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStore(HashMap<Key, KVDBObject>);

impl KVDBStore for MapStore {}

impl MapStore {
    pub fn set(&mut self, cmd: impl Into<SetCommand>) -> Result<()> {
        let SetCommand(k, v) = cmd.into();
        self.0.insert(k.to_owned(), v);
        Ok(())
    }

    pub fn get(&self, cmd: impl Into<GetCommand>) -> Result<Option<KVDBValue>> {
        let GetCommand(k, field) = cmd.into();
        if let Some(store) = self.0.get(&k) {
            Ok(store.get(&field).cloned())
        } else {
            Ok(None)
        }
    }

    pub fn get_all(
        &self,
        cmd: impl Into<GetAllCommand>,
    ) -> Result<Option<HashMap<Key, KVDBValue>>> {
        let GetAllCommand(k) = cmd.into();
        if let Some(v) = self.0.get(&k) {
            Ok(Some(v.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn get_object<T: TryFrom<KVDBObject>>(
        &self,
        cmd: impl Into<GetObjectCommand>,
    ) -> Result<Option<T>> {
        let GetObjectCommand(k) = cmd.into();
        if let Some(obj) = self.0.get(&k).cloned() {
            Ok(Some(
                obj.try_into().map_err(|_| MiniKVDBError::InvalidObject)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&mut self, cmd: impl Into<DeleteCommand>) -> Result<Option<KVDBObject>> {
        Ok(self.0.remove(&cmd.into().0))
    }
}

impl MiniKVDB {
    pub fn hash_set(&self, key: impl Into<Key>, value: impl Into<KVDBObject>) -> Result<()> {
        self.map.write()?.set(SetCommand(key.into(), value.into()))
    }

    pub fn hash_get(
        &self,
        key: impl Into<Key>,
        field: impl Into<Key>,
    ) -> Result<Option<KVDBValue>> {
        self.map.read()?.get(GetCommand(key.into(), field.into()))
    }

    pub fn hash_get_all(&self, key: impl Into<Key>) -> Result<Option<KVDBObject>> {
        self.map.read()?.get_all(GetAllCommand(key.into()))
    }

    pub fn hash_get_object<T: TryFrom<KVDBObject>>(
        &self,
        key: impl Into<Key>,
    ) -> Result<Option<T>> {
        self.map.read()?.get_object(GetObjectCommand(key.into()))
    }

    pub fn hash_delete(&self, key: impl Into<Key>) -> Result<Option<KVDBObject>> {
        self.map.write()?.delete(DeleteCommand(key.into()))
    }
}
