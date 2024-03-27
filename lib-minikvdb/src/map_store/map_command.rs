use crate::minikvdb::kvdb_value::KVDBObject;

#[derive(Debug, Clone)]
pub struct SetCommand<'a>(pub &'a str, pub KVDBObject);

impl<'a, T> From<(&'a str, T)> for SetCommand<'a>
where
    T: Into<KVDBObject>,
{
    fn from((k, v): (&'a str, T)) -> Self {
        Self(k, v.into())
    }
}

#[derive(Debug, Clone)]
pub struct GetCommand<'a>(pub &'a str, pub &'a str);

impl<'a> From<(&'a str, &'a str)> for GetCommand<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Debug, Clone)]
pub struct GetAllCommand<'a>(pub &'a str);

impl<'a> From<&'a str> for GetAllCommand<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteCommand<'a>(pub &'a str);

impl<'a> From<&'a str> for DeleteCommand<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}
