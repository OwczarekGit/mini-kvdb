use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
    vec,
};

use crate::{
    error::{MiniKVDBError, Result},
    minikvdb::{kvdb_value::KVDBValue, MiniKVDB},
};

use self::list_command::{
    ListLenCommmand, ListRangeCommand, ListRemoveCommand, PopBackCommand, PopFrontCommand,
    PushBackCommand, PushFrontCommand,
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

    pub fn list_range<'a>(
        store: &Self,
        cmd: impl Into<ListRangeCommand<'a>>,
    ) -> Result<Vec<KVDBValue>> {
        let ListRangeCommand(k, start, mut count) = cmd.into();
        if let Some(list) = store.0.get(k) {
            if list.is_empty() {
                return Ok(vec![]);
            }

            let list: Vec<KVDBValue> = list.clone().into();

            if count < 0 {
                return Ok(list[start as usize..].to_vec());
            }

            return Ok(list[start as usize..((start + count) as usize).min(list.len())].to_vec());
        } else {
            Ok(vec![])
        }
    }

    pub fn list_len<'a>(store: &Self, cmd: impl Into<ListLenCommmand<'a>>) -> Result<usize> {
        let ListLenCommmand(k) = cmd.into();
        Ok(store.0.get(k).map(|l| l.len()).unwrap_or(0))
    }

    pub fn list_remove<'a>(
        store: &mut Self,
        cmd: impl Into<ListRemoveCommand<'a>>,
    ) -> Result<usize> {
        let ListRemoveCommand(k, mut c, v) = cmd.into();
        if let Some(list) = store.0.get_mut(k) {
            let mut dc = 0;
            if c == 0 {
                list.retain(|el| {
                    if *el != v || dc > 0 {
                        true
                    } else {
                        dc += 1;
                        false
                    }
                });

                Ok(dc)
            } else {
                list.retain(|el| {
                    if *el != v || c <= 0 {
                        true
                    } else {
                        c -= 1;
                        dc += 1;
                        false
                    }
                });

                Ok(dc)
            }
        } else {
            Ok(0)
        }
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

    pub fn list_range<'a>(&self, cmd: impl Into<ListRangeCommand<'a>>) -> Result<Vec<KVDBValue>> {
        ListStore::list_range(&*self.list.read()?, cmd)
    }

    pub fn list_len<'a>(&self, cmd: impl Into<ListLenCommmand<'a>>) -> Result<usize> {
        ListStore::list_len(&*self.list.read()?, cmd)
    }

    pub fn list_remove<'a>(&self, cmd: impl Into<ListRemoveCommand<'a>>) -> Result<usize> {
        ListStore::list_remove(&mut *self.list.write()?, cmd)
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
