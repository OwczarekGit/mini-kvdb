use crate::minikvdb::{kvdb_key::Key, kvdb_value::KVDBValue};

#[derive(Debug, Clone)]
pub struct SetCommand(pub Key, pub KVDBValue);

#[derive(Debug, Clone)]
pub struct GetCommand(pub Key);

#[derive(Debug, Clone)]
pub struct IncrementCommand(pub Key, pub f32);
