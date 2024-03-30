use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
    prelude::KVDBObject,
};

use self::map_command::{DeleteCommand, GetAllCommand, GetCommand, GetObjectCommand, SetCommand};

pub mod map_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStore(HashMap<String, KVDBObject>);

impl KVDBStore for MapStore {}

impl MapStore {
    pub fn set<'a>(&mut self, cmd: impl Into<SetCommand<'a>>) -> Result<()> {
        let SetCommand(k, v) = cmd.into();
        self.0.insert(k.to_owned(), v);
        Ok(())
    }

    pub fn get<'a>(&self, cmd: impl Into<GetCommand<'a>>) -> Result<Option<KVDBValue>> {
        let GetCommand(k, field) = cmd.into();
        if let Some(store) = self.0.get(k) {
            Ok(store.get(field).cloned())
        } else {
            Ok(None)
        }
    }

    pub fn get_all<'a>(
        &self,
        cmd: impl Into<GetAllCommand<'a>>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        let GetAllCommand(k) = cmd.into();
        if let Some(v) = self.0.get(k) {
            Ok(Some(v.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn get_object<'a, T: TryFrom<KVDBObject>>(
        &self,
        cmd: impl Into<GetObjectCommand<'a>>,
    ) -> Result<Option<T>> {
        let GetObjectCommand(k) = cmd.into();
        if let Some(obj) = self.0.get(k).cloned() {
            Ok(Some(
                obj.try_into().map_err(|_| MiniKVDBError::InvalidObject)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn delete<'a>(
        &mut self,
        cmd: impl Into<DeleteCommand<'a>>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        Ok(self.0.remove(cmd.into().0))
    }
}

impl MiniKVDB {
    pub fn hash_set<'a>(
        &self,
        key: impl Into<&'a str>,
        value: impl Into<HashMap<String, KVDBValue>>,
    ) -> Result<()> {
        self.map.write()?.set((key.into(), value.into()))
    }

    pub fn hash_get<'a>(
        &self,
        key: impl Into<&'a str>,
        field: impl Into<&'a str>,
    ) -> Result<Option<KVDBValue>> {
        self.map.read()?.get((key.into(), field.into()))
    }

    pub fn hash_get_all<'a>(
        &self,
        key: impl Into<&'a str>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        self.map.read()?.get_all(key.into())
    }

    pub fn hash_get_object<'a, T: TryFrom<KVDBObject>>(
        &self,
        key: impl Into<String>,
    ) -> Result<Option<T>> {
        self.map.read()?.get_object(&*key.into())
    }

    pub fn hash_delete<'a>(
        &self,
        key: impl Into<&'a str>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        self.map.write()?.delete(key.into())
    }
}
