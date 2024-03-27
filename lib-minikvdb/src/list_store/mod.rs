use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_value::KVDBValue, MiniKVDB},
};

use self::list_command::{
    ListLenCommmand, PopBackCommand, PopFrontCommand, PushBackCommand, PushFrontCommand,
};

pub mod list_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListStore(HashMap<String, VecDeque<KVDBValue>>);

impl ListStore {
    pub fn push_front<'a>(store: &mut Self, cmd: impl Into<PushFrontCommand<'a>>) -> Result<usize> {
        let PushFrontCommand(k, v) = cmd.into();
        if let Some(list) = store.0.get_mut(k) {
            for value in v {
                list.push_front(value.to_owned());
            }
            Ok(list.len())
        } else {
            store.0.insert(k.to_owned(), v.into());
            Ok(1)
        }
    }

    pub fn pop_front<'a>(
        store: &mut Self,
        cmd: impl Into<PopFrontCommand<'a>>,
    ) -> Result<Option<KVDBValue>> {
        let PopFrontCommand(k) = cmd.into();
        if let Some(list) = store.0.get_mut(k) {
            Ok(list.pop_front())
        } else {
            Ok(None)
        }
    }

    pub fn push_back<'a>(store: &mut Self, cmd: impl Into<PushBackCommand<'a>>) -> Result<usize> {
        let PushBackCommand(k, v) = cmd.into();
        if let Some(list) = store.0.get_mut(k) {
            for value in v {
                list.push_back(value);
            }
            Ok(list.len())
        } else {
            store.0.insert(k.to_owned(), v.into());
            Ok(1)
        }
    }

    pub fn pop_back<'a>(
        store: &mut Self,
        cmd: impl Into<PopBackCommand<'a>>,
    ) -> Result<Option<KVDBValue>> {
        let PopBackCommand(k) = cmd.into();
        if let Some(list) = store.0.get_mut(k) {
            Ok(list.pop_back())
        } else {
            Ok(None)
        }
    }

    pub fn len<'a>(store: &Self, cmd: impl Into<ListLenCommmand<'a>>) -> Result<usize> {
        let ListLenCommmand(k) = cmd.into();
        Ok(store.0.get(k).map(|l| l.len()).unwrap_or(0))
    }
}

impl MiniKVDB {
    pub fn push_front<'a>(
        &self,
        key: impl Into<&'a str>,
        values: impl Into<Vec<KVDBValue>>,
    ) -> Result<usize> {
        ListStore::push_front(&mut *self.list.write()?, (key.into(), values.into()))
    }

    pub fn pop_front<'a>(&self, cmd: impl Into<PopFrontCommand<'a>>) -> Result<Option<KVDBValue>> {
        ListStore::pop_front(&mut *self.list.write()?, cmd)
    }

    pub fn push_back<'a>(
        &self,
        key: impl Into<&'a str>,
        values: impl Into<Vec<KVDBValue>>,
    ) -> Result<usize> {
        ListStore::push_back(&mut *self.list.write()?, (key.into(), values.into()))
    }

    pub fn pop_back<'a>(&self, cmd: impl Into<PopBackCommand<'a>>) -> Result<Option<KVDBValue>> {
        ListStore::pop_back(&mut *self.list.write()?, cmd)
    }

    pub fn list_len<'a>(&self, cmd: impl Into<ListLenCommmand<'a>>) -> Result<usize> {
        ListStore::len(&*self.list.read()?, cmd)
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, ListStore>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, ListStore>>) -> Self {
        Self::RWLockWritePoison
    }
}

impl From<PoisonError<RwLockReadGuard<'_, ListStore>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockReadGuard<'_, ListStore>>) -> Self {
        Self::RWLockReadPoison
    }
}
