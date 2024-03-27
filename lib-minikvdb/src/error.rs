pub type Result<T> = std::result::Result<T, MiniKVDBError>;

#[derive(Debug, Clone)]
pub enum MiniKVDBError {
    RWLockWritePoison,
    RWLockReadPoison,
    CannotIncrement,
    WrongFieldType,
    MissingField(String),
}
