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
    pub fn set(&mut self, cmd: impl Into<SetCommand>) -> Result<Option<KVDBObject>> {
        let SetCommand(k, v) = cmd.into();
        Ok(self.0.insert(k.to_owned(), v))
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
    pub fn hash_set(
        &self,
        key: impl Into<Key>,
        value: impl Into<KVDBObject>,
    ) -> Result<Option<KVDBObject>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> MapStore {
        MapStore::default()
    }

    #[test]
    fn sets_value_on_empty_key() {
        let mut db = test_db();
        let ret = db
            .set(SetCommand(
                "a".into(),
                [("name".into(), "tom".into())].into(),
            ))
            .unwrap();
        assert!(ret.is_none());
    }

    #[test]
    fn sets_value_on_non_empty_key_and_returns_old() {
        let mut db = test_db();
        let _ = db
            .set(SetCommand(
                "a".into(),
                [("name".into(), "bob".into())].into(),
            ))
            .unwrap();

        let ret = db
            .set(SetCommand(
                "a".into(),
                [("name".into(), "tom".into())].into(),
            ))
            .unwrap();
        assert!(ret.is_some());
        assert_eq!(
            *ret.unwrap().get("name".into()).unwrap(),
            KVDBValue::String("bob".into())
        );

        assert_eq!(
            *db.0.get("a".into()).unwrap().get("name").unwrap(),
            KVDBValue::String("tom".into())
        );
    }

    fn seeded_db() -> MapStore {
        let mut db = test_db();
        let _ = db.set(SetCommand(
            "a".into(),
            [("name".into(), "tom".into()), ("age".into(), 22.into())].into(),
        ));

        let _ = db.set(SetCommand(
            "b".into(),
            [("name".into(), "bob".into()), ("age".into(), 42.into())].into(),
        ));

        let _ = db.set(SetCommand(
            "c".into(),
            [
                ("name".into(), "john".into()),
                ("balance".into(), 123.21.into()),
            ]
            .into(),
        ));

        db
    }

    #[test]
    fn gets_value_from_existing_key() {
        let db = seeded_db();
        let res = db.get_all(GetAllCommand("a".into())).unwrap();

        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(
            *res.get("name".into()).unwrap(),
            KVDBValue::String("tom".into())
        );

        assert_eq!(*res.get("age".into()).unwrap(), KVDBValue::Int(22.into()));
    }

    #[test]
    fn gets_none_when_called_on_empty_key() {
        let db = seeded_db();
        let res = db.get_all(GetAllCommand("asdf".into())).unwrap();

        assert!(res.is_none());
    }

    #[test]
    fn gets_value_when_deleted_existing_key() {
        let mut db = seeded_db();
        let res = db.delete(DeleteCommand("b".into())).unwrap();
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(
            *res.get("name".into()).unwrap(),
            KVDBValue::String("bob".into())
        );
        assert_eq!(*res.get("age".into()).unwrap(), KVDBValue::Int(42.into()));
        assert!(db.0.get("b".into()).is_none());
    }
}
