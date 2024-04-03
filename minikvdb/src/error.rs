use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

use crate::minikvdb::KVDBStore;

pub type Result<T> = std::result::Result<T, MiniKVDBError>;

#[derive(Debug, Clone)]
pub enum MiniKVDBError {
    RWLockWritePoison,
    RWLockReadPoison,
    CannotIncrement,
    WrongFieldType,
    InvalidObject,
    MissingField(String),
}

impl<T: KVDBStore> From<PoisonError<RwLockWriteGuard<'_, T>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        Self::RWLockWritePoison
    }
}

impl<T: KVDBStore> From<PoisonError<RwLockReadGuard<'_, T>>> for MiniKVDBError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Self::RWLockReadPoison
    }
}
