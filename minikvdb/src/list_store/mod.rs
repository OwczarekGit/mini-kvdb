use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

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
            let len = v.len();
            self.0.insert(k.clone(), Default::default());
            let db = self.0.get_mut(&k).expect("Was just inserted");
            for value in v {
                db.push_front(value);
            }
            Ok(len)
        }
    }

    pub fn pop_front(&mut self, cmd: impl Into<PopFrontCommand>) -> Result<Option<KVDBValue>> {
        let PopFrontCommand(k) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            let pop = list.pop_front();
            if list.is_empty() {
                self.0.remove(&k);
            }
            Ok(pop)
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
            let len = v.len();
            self.0.insert(k.to_owned(), v.into());
            Ok(len)
        }
    }

    pub fn pop_back(&mut self, cmd: impl Into<PopBackCommand>) -> Result<Option<KVDBValue>> {
        let PopBackCommand(k) = cmd.into();
        if let Some(list) = self.0.get_mut(&k) {
            let pop = list.pop_back();
            if list.is_empty() {
                self.0.remove(&k);
            }
            Ok(pop)
        } else {
            Ok(None)
        }
    }

    pub fn range(&self, cmd: impl Into<ListRangeCommand>) -> Result<Option<Vec<KVDBValue>>> {
        let ListRangeCommand(k, opts) = cmd.into();

        if let Some(list) = self.0.get(&k) {
            match opts {
                list_command::ListRangeOption::Whole => Ok(Some(list.clone().into())),
                list_command::ListRangeOption::FromIndex(start) => {
                    if start < list.len() {
                        let list: Vec<KVDBValue> = list.clone().into();
                        let list = list[start..].to_vec();
                        Ok(Some(list))
                    } else {
                        Ok(Some(vec![]))
                    }
                }
                list_command::ListRangeOption::FromIndexWithLen(start, count) => {
                    if start < list.len() {
                        let list: Vec<KVDBValue> = list.clone().into();
                        let list = list[start..=(start + count).min(list.len() - 1)].to_vec();
                        Ok(Some(list))
                    } else {
                        Ok(Some(vec![]))
                    }
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn len(&self, cmd: impl Into<ListLenCommmand>) -> Result<Option<usize>> {
        let ListLenCommmand(k) = cmd.into();
        Ok(self.0.get(&k).map(|l| l.len()))
    }

    pub fn remove(&mut self, cmd: impl Into<ListRemoveCommand>) -> Result<usize> {
        let ListRemoveCommand(k, opts) = cmd.into();

        match opts {
            list_command::ListRemoveOption::All(v) => {
                if let Some(list) = self.0.get_mut(&k) {
                    let mut dc = 0;
                    list.retain(|el| {
                        if *el == v {
                            dc += 1;
                            false
                        } else {
                            true
                        }
                    });

                    if list.is_empty() {
                        self.0.remove(&k);
                    }

                    Ok(dc)
                } else {
                    Ok(0)
                }
            }
            list_command::ListRemoveOption::Count(mut n, v) => {
                if let Some(list) = self.0.get_mut(&k) {
                    let mut dc = 0;
                    list.retain(|el| {
                        if *el == v && n > 0 {
                            dc += 1;
                            n -= 1;
                            false
                        } else {
                            true
                        }
                    });

                    if list.is_empty() {
                        self.0.remove(&k);
                    }

                    Ok(dc)
                } else {
                    Ok(0)
                }
            }
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

    pub fn list_range(&self, cmd: impl Into<ListRangeCommand>) -> Result<Option<Vec<KVDBValue>>> {
        self.list.read()?.range(cmd)
    }

    pub fn list_len(&self, key: impl Into<Key>) -> Result<Option<usize>> {
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

#[cfg(test)]
mod tests {
    use tests::list_command::ListRemoveOption;

    use crate::values;

    use self::list_command::ListRangeOption;

    use super::*;

    fn test_db() -> ListStore {
        ListStore::default()
    }

    #[test]
    fn pushes_back() {
        let mut db = test_db();
        let res = db
            .push_back(PushBackCommand("a".into(), values!(1, 2, 3, 4)))
            .unwrap();

        assert_eq!(res, 4);
        assert_eq!(db.0.get("a").unwrap().clone(), values!(1, 2, 3, 4));
    }

    #[test]
    fn pushes_back_to_existing() {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("a".into(), values!(22)));
        let res = db
            .push_back(PushBackCommand("a".into(), values!(1, 2, 3, 4)))
            .unwrap();

        assert_eq!(res, 5);
        assert_eq!(db.0.get("a").unwrap().clone(), values!(22, 1, 2, 3, 4));
    }

    #[test]
    fn pushes_front() {
        let mut db = test_db();
        let res = db
            .push_front(PushFrontCommand("a".into(), values!(1, 2, 3, 4)))
            .unwrap();

        assert_eq!(res, 4);
        assert_eq!(db.0.get("a").unwrap().clone(), values!(4, 3, 2, 1));
    }

    #[test]
    fn pushes_front_to_existing() {
        let mut db = test_db();
        let _ = db.push_front(PushFrontCommand("a".into(), values!(33)));
        let res = db
            .push_front(PushFrontCommand("a".into(), values!(1, 2, 3, 4)))
            .unwrap();

        assert_eq!(res, 5);
        assert_eq!(db.0.get("a").unwrap().clone(), values!(4, 3, 2, 1, 33));
    }

    #[test]
    fn pops_back() {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("a".into(), values!(1, 2, 3, 4, 5)));
        for i in (1..=5).rev() {
            let pop = db.pop_back(PopBackCommand("a".into())).unwrap();
            assert!(pop.is_some());
            assert_eq!(pop.unwrap(), KVDBValue::Int(i));
        }

        let pop = db.pop_back(PopBackCommand("a".into())).unwrap();
        assert!(pop.is_none());
        assert!(db.0.get("a").is_none());
    }

    #[test]
    fn pops_front() {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("a".into(), values!(1, 2, 3, 4, 5)));
        for i in 1..=5 {
            let pop = db.pop_front(PopFrontCommand("a".into())).unwrap();
            assert!(pop.is_some());
            assert_eq!(pop.unwrap(), KVDBValue::Int(i));
        }

        let pop = db.pop_front(PopFrontCommand("a".into())).unwrap();
        assert!(pop.is_none());
        assert!(db.0.get("a").is_none());
    }

    #[test]
    fn gets_list_len() {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("a".into(), values!(1, 2, 3, 4, 5)));
        let len = db.len(ListLenCommmand("a".into())).unwrap();
        assert!(len.is_some());
        assert_eq!(len.unwrap(), 5);

        let empty_list_len = db.len(ListLenCommmand("qwe".into())).unwrap();
        assert!(empty_list_len.is_none());
    }

    fn seeded_db() -> ListStore {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("ints".into(), values!(1, 2, 3, 4, 5, 6)));
        let _ = db.push_back(PushBackCommand(
            "mixed".into(),
            values!(1, 2.2, true, 4, false, false, "text"),
        ));
        let _ = db.push_back(PushBackCommand(
            "texts".into(),
            values!("t1", "t2", "t3", "t4", "t5", "t6"),
        ));

        db
    }

    #[test]
    fn removes_nothing_when_non_existing_key() {
        let mut db = seeded_db();
        let del_num = db
            .remove(ListRemoveCommand(
                "abscent".into(),
                ListRemoveOption::All(false.into()),
            ))
            .unwrap();

        assert_eq!(del_num, 0);
    }

    #[test]
    fn when_list_becomes_empty_after_remove_said_list_gets_removed() {
        let mut db = test_db();
        let _ = db.push_back(PushBackCommand("items".into(), values!(2, 2, 2, 2, 2)));
        let count = db
            .remove(ListRemoveCommand(
                "items".into(),
                ListRemoveOption::All(2.into()),
            ))
            .unwrap();
        assert_eq!(count, 5);
        assert!(db.0.get("items").is_none());
    }

    #[test]
    fn removes_all() {
        let mut db = seeded_db();
        let del_num = db
            .remove(ListRemoveCommand(
                "mixed".into(),
                ListRemoveOption::All(false.into()),
            ))
            .unwrap();

        assert_eq!(del_num, 2);
        assert_eq!(
            db.0.get("mixed".into()).unwrap().clone(),
            values!(1, 2.2, true, 4, "text")
        );
    }

    #[test]
    fn removes_specified_number_of_items() {
        let mut db = seeded_db();
        let del_num = db
            .remove(ListRemoveCommand(
                "mixed".into(),
                ListRemoveOption::Count(1, false.into()),
            ))
            .unwrap();

        assert_eq!(del_num, 1);
        assert_eq!(
            db.0.get("mixed".into()).unwrap().clone(),
            values!(1, 2.2, true, 4, false, "text")
        );
    }

    #[test]
    fn contains_value_on_existing_key() {
        let db = seeded_db();
        let contains_4 = db
            .contains(ListContainsValueCommand("mixed".into(), 4.into()))
            .unwrap();
        assert!(contains_4);
    }

    #[test]
    fn does_not_contain_value_on_existing_key() {
        let db = seeded_db();
        let contains_44 = db
            .contains(ListContainsValueCommand("mixed".into(), 44.into()))
            .unwrap();
        assert!(!contains_44);
    }

    #[test]
    fn does_not_contain_value_on_non_existing_key() {
        let db = seeded_db();
        let contains = db
            .contains(ListContainsValueCommand(
                "non_existing_key".into(),
                4.into(),
            ))
            .unwrap();
        assert!(!contains);
    }

    #[test]
    fn gets_entire_when_calling_range_on_existing_key() {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand("ints".into(), ListRangeOption::Whole))
            .unwrap();
        assert!(list.is_some());
        assert_eq!(list.unwrap(), values!(1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn gets_none_when_calling_range_on_non_existing_key() {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand(
                "non_existing".into(),
                ListRangeOption::Whole,
            ))
            .unwrap();
        assert!(list.is_none());
    }

    #[test]
    fn gets_list_from_specified_start_index_to_the_end_when_calling_range_on_existing_key() {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand(
                "mixed".into(),
                ListRangeOption::FromIndex(4),
            ))
            .unwrap();
        assert!(list.is_some());
        assert_eq!(list.unwrap(), values!(false, false, "text"));
    }

    #[test]
    fn gets_empty_list_when_calling_range_on_existing_key_with_start_outside_of_bounds() {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand(
                "mixed".into(),
                ListRangeOption::FromIndex(99),
            ))
            .unwrap();
        assert!(list.is_some());
        assert!(list.unwrap().is_empty());
    }

    #[test]
    fn gets_count_from_start_when_calling_rango_on_existing_key() {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand(
                "mixed".into(),
                ListRangeOption::FromIndexWithLen(2, 2),
            ))
            .unwrap();
        assert!(list.is_some());
        assert_eq!(list.unwrap(), values!(true, 4, false));
    }

    #[test]
    fn gets_to_end_from_start_when_calling_rango_on_existing_key_when_count_plus_start_is_out_of_bounds(
    ) {
        let db = seeded_db();
        let list = db
            .range(ListRangeCommand(
                "mixed".into(),
                ListRangeOption::FromIndexWithLen(2, 2000),
            ))
            .unwrap();
        assert!(list.is_some());
        assert_eq!(list.unwrap(), values!(true, 4, false, false, "text"));
    }

    #[test]
    fn gets_empty_when_calling_range_on_empty_list_on_existing_key() {
        // NOTE: Technically this should never happen, because empty lists are removed.
        let mut db = seeded_db();
        db.0.insert("empty".into(), vec![].into());

        let list = db
            .range(ListRangeCommand(
                "empty".into(),
                ListRangeOption::FromIndexWithLen(2, 2000),
            ))
            .unwrap();
        assert!(list.is_some());
        assert_eq!(list.unwrap(), vec![]);
    }
}
