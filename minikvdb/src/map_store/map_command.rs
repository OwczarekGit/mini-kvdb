use crate::minikvdb::{kvdb_key::Key, kvdb_value::KVDBObject};

#[derive(Debug, Clone)]
pub struct SetCommand(pub Key, pub KVDBObject);

#[derive(Debug, Clone)]
pub struct GetCommand(pub Key, pub Key);

#[derive(Debug, Clone)]
pub struct GetAllCommand(pub Key);

#[derive(Debug, Clone)]
pub struct DeleteCommand(pub Key);

#[derive(Debug, Clone)]
pub struct GetObjectCommand(pub Key);

#[derive(Debug, Clone)]
pub struct ContainsKeyCommand(pub Key);
