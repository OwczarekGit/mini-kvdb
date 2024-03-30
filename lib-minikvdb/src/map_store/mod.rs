use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_value::KVDBValue, MiniKVDB},
    prelude::KVDBObject,
};

use self::map_command::{DeleteCommand, GetAllCommand, GetCommand, GetObjectCommand, SetCommand};

pub mod map_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStore(HashMap<String, HashMap<String, KVDBValue>>);

impl MapStore {
    pub fn set<'a>(store: &mut Self, cmd: impl Into<SetCommand<'a>>) -> Result<()> {
        let SetCommand(k, v) = cmd.into();
        store.0.insert(k.to_owned(), v);
        Ok(())
    }

    pub fn get<'a>(store: &Self, cmd: impl Into<GetCommand<'a>>) -> Result<Option<KVDBValue>> {
        let GetCommand(k, field) = cmd.into();
        if let Some(store) = store.0.get(k) {
            Ok(store.get(field).cloned())
        } else {
            Ok(None)
        }
    }

    pub fn get_all<'a>(
        store: &Self,
        cmd: impl Into<GetAllCommand<'a>>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        let GetAllCommand(k) = cmd.into();
        if let Some(v) = store.0.get(k) {
            Ok(Some(v.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn get_object<'a, T: TryFrom<KVDBObject>>(
        store: &Self,
        cmd: impl Into<GetObjectCommand<'a>>,
    ) -> Result<Option<T>> {
        let GetObjectCommand(k) = cmd.into();
        if let Some(obj) = store.0.get(k).cloned() {
            Ok(Some(
                obj.try_into().map_err(|_| MiniKVDBError::InvalidObject)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn delete<'a>(
        store: &mut Self,
        cmd: impl Into<DeleteCommand<'a>>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        Ok(store.0.remove(cmd.into().0))
    }
}

impl MiniKVDB {
    pub fn hash_set<'a>(
        &self,
        key: impl Into<&'a str>,
        value: impl Into<HashMap<String, KVDBValue>>,
    ) -> Result<()> {
        MapStore::set(&mut *self.map.write()?, (key.into(), value.into()))
    }

    pub fn hash_get<'a>(
        &self,
        key: impl Into<&'a str>,
        field: impl Into<&'a str>,
    ) -> Result<Option<KVDBValue>> {
        MapStore::get(&*self.map.read()?, (key.into(), field.into()))
    }

    pub fn hash_get_all<'a>(
        &self,
        key: impl Into<&'a str>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        MapStore::get_all(&*self.map.read()?, key.into())
    }

    pub fn hash_get_object<'a, T: TryFrom<KVDBObject>>(
        &self,
        key: impl Into<&'a str>,
    ) -> Result<Option<T>> {
        MapStore::get_object(&*self.map.read()?, key.into())
    }

    pub fn hash_delete<'a>(
        &self,
        key: impl Into<&'a str>,
    ) -> Result<Option<HashMap<String, KVDBValue>>> {
        MapStore::delete(&mut *self.map.write()?, key.into())
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, MapStore>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, MapStore>>) -> Self {
        Self::RWLockWritePoison
    }
}

impl From<PoisonError<RwLockReadGuard<'_, MapStore>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockReadGuard<'_, MapStore>>) -> Self {
        Self::RWLockReadPoison
    }
}
