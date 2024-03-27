use crate::minikvdb::kvdb_value::KVDBValue;

#[derive(Debug, Clone)]
pub struct SetCommand<'a>(pub &'a str, pub KVDBValue);

impl<'a, T> From<(&'a str, T)> for SetCommand<'a>
where
    T: Into<KVDBValue>,
{
    fn from((k, v): (&'a str, T)) -> Self {
        Self(k, v.into())
    }
}

#[derive(Debug, Clone)]
pub struct GetCommand<'a>(pub &'a str);

impl<'a, T> From<T> for GetCommand<'a>
where
    T: Into<&'a str>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct IncrementCommand<'a>(pub &'a str, pub f32);

impl<'a, T> From<(&'a str, T)> for IncrementCommand<'a>
where
    T: Into<f32>,
{
    fn from((k, v): (&'a str, T)) -> Self {
        Self(k, v.into())
    }
}
