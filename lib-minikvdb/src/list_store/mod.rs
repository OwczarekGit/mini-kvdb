use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use crate::{
    error::Result,
    minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue, KVDBStore, MiniKVDB},
};

use self::list_command::{
    ListContainsValueCommand, ListLenCommmand, ListRangeCommand, ListRemoveCommand, PopBackCommand,
    PopFrontCommand, PushBackCommand, PushFrontCommand,
};

pub mod list_command;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListStore(HashMap<Key, VecDeque<KVDBValue>>);

impl KVDBStore for ListStore {}

impl ListStore {
    pub fn push_front(&mut self, cmd: impl Into<PushFrontCommand>) -> Result<usize> {
        let PushFrontCommand(k, v) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            for value in v {
                list.push_front(value.to_owned());
            }
            Ok(list.len())
        } else {
            self.0.insert(k.to_owned(), v.into());
            Ok(1)
        }
    }

    pub fn pop_front(&mut self, cmd: impl Into<PopFrontCommand>) -> Result<Option<KVDBValue>> {
        let PopFrontCommand(k) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            Ok(list.pop_front())
        } else {
            Ok(None)
        }
    }

    pub fn push_back(&mut self, cmd: impl Into<PushBackCommand>) -> Result<usize> {
        let PushBackCommand(k, v) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            for value in v {
                list.push_back(value);
            }
            Ok(list.len())
        } else {
            self.0.insert(k.to_owned(), v.into());
            Ok(1)
        }
    }

    pub fn pop_back(&mut self, cmd: impl Into<PopBackCommand>) -> Result<Option<KVDBValue>> {
        let PopBackCommand(k) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            Ok(list.pop_back())
        } else {
            Ok(None)
        }
    }

    pub fn range(&self, cmd: impl Into<ListRangeCommand>) -> Result<Vec<KVDBValue>> {
        let ListRangeCommand(k, start, count) = cmd.into();
        if let Some(list) = self.0.get(&k) {
            if list.is_empty() || (start > list.len() as i32 - 1 && count < 0) {
                return Ok(vec![]);
            }

            let list: Vec<KVDBValue> = list.clone().into();

            if count < 0 {
                if start < 0 {
                    let offset = list.len() as i32 - -start;
                    if offset < 0 {
                        return Ok(vec![]);
                    } else {
                        return Ok(list[offset as usize..].to_vec());
                    }
                } else {
                    return Ok(list[start as usize..].to_vec());
                }
            }

            Ok(list[start as usize..((start + count) as usize).min(list.len())].to_vec())
        } else {
            Ok(vec![])
        }
    }

    pub fn len(&self, cmd: impl Into<ListLenCommmand>) -> Result<usize> {
        let ListLenCommmand(k) = cmd.into();
        Ok(self.0.get(&k).map(|l| l.len()).unwrap_or(0))
    }

    pub fn remove(&mut self, cmd: impl Into<ListRemoveCommand>) -> Result<usize> {
        let ListRemoveCommand(k, mut c, v) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            let mut dc = 0;
            if c == 0 {
                list.retain(|el| {
                    if *el != v {
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

    pub fn contains(&self, cmd: impl Into<ListContainsValueCommand>) -> Result<bool> {
        let ListContainsValueCommand(k, v) = cmd.into();
        if let Some(list) = self.0.get(&k) {
            Ok(list.iter().any(|i| *i == v))
        } else {
            Ok(false)
        }
    }
}

impl MiniKVDB {
    pub fn push_front(
        &self,
        key: impl Into<Key>,
        values: impl Into<Vec<KVDBValue>>,
    ) -> Result<usize> {
        self.list
            .write()?
            .push_front(PushFrontCommand(key.into(), values.into()))
    }

    pub fn pop_front(&self, cmd: impl Into<PopFrontCommand>) -> Result<Option<KVDBValue>> {
        ListStore::pop_front(&mut *self.list.write()?, cmd)
    }

    pub fn push_back(
        &self,
        key: impl Into<Key>,
        values: impl Into<Vec<KVDBValue>>,
    ) -> Result<usize> {
        self.list
            .write()?
            .push_back(PushBackCommand(key.into(), values.into()))
    }

    pub fn pop_back(&self, cmd: impl Into<PopBackCommand>) -> Result<Option<KVDBValue>> {
        self.list.write()?.pop_back(cmd)
    }

    pub fn list_range(&self, cmd: impl Into<ListRangeCommand>) -> Result<Vec<KVDBValue>> {
        self.list.read()?.range(cmd)
    }

    pub fn list_len(&self, key: impl Into<Key>) -> Result<usize> {
        self.list.read()?.len(ListLenCommmand(key.into()))
    }

    pub fn list_remove(&self, cmd: impl Into<ListRemoveCommand>) -> Result<usize> {
        self.list.write()?.remove(cmd)
    }

    pub fn list_contains(&self, key: impl Into<Key>, value: impl Into<KVDBValue>) -> Result<bool> {
        self.list
            .read()?
            .contains(ListContainsValueCommand(key.into(), value.into()))
    }
}
