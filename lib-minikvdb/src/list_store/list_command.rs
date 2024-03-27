use crate::minikvdb::kvdb_value::KVDBValue;

#[derive(Debug, Clone)]
pub struct PushFrontCommand<'a>(pub &'a str, pub Vec<KVDBValue>);

impl<'a, T> From<(&'a str, T)> for PushFrontCommand<'a>
where
    T: Into<Vec<KVDBValue>>,
{
    fn from((k, v): (&'a str, T)) -> Self {
        Self(k, v.into())
    }
}

#[derive(Debug, Clone)]
pub struct PopFrontCommand<'a>(pub &'a str);

impl<'a, T> From<T> for PopFrontCommand<'a>
where
    T: Into<&'a str>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct PushBackCommand<'a>(pub &'a str, pub Vec<KVDBValue>);

impl<'a, T> From<(&'a str, T)> for PushBackCommand<'a>
where
    T: Into<Vec<KVDBValue>>,
{
    fn from((k, v): (&'a str, T)) -> Self {
        Self(k, v.into())
    }
}

#[derive(Debug, Clone)]
pub struct PopBackCommand<'a>(pub &'a str);

impl<'a, T> From<T> for PopBackCommand<'a>
where
    T: Into<&'a str>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct ListLenCommmand<'a>(pub &'a str);

impl<'a, T> From<T> for ListLenCommmand<'a>
where
    T: Into<&'a str>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
