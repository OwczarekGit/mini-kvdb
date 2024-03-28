use chrono::{DateTime, Utc};
use lib_minikvdb::prelude::*;
use minikvdb_macros::KVDBEntity;

fn main() {
    let db = MiniKVDB::default();

    let _ = db.set("name", "Tom");

    let _ = db.push_front(
        "things",
        values!(
            "John",
            3.141529,
            69,
            420.0,
            false,
            true,
            false,
            false,
            true,
            Utc::now(),
            Utc::now()
        ),
    );

    // dbg!(db.list_range(("things", 0)));
    dbg!(db.list_remove(("things", 1, true)));

    dbg!(&db);

    let _ = db.hash_set("user:2", Person::default());
    let _ = db.hash_set(
        "user:1",
        Person {
            name: "Bob".to_owned(),
            age: 44,
            money: 123.4,
            premium: false,
            joined: Utc::now(),
        },
    );

    let x = Utc::now();

    // dbg!(db);
}

#[derive(Debug, Default, Clone, KVDBEntity)]
pub struct Person {
    name: String,
    age: i32,
    money: f32,
    premium: bool,
    joined: DateTime<Utc>,
}
