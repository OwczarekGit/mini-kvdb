use chrono::{DateTime, Utc};
use lib_minikvdb::{minikvdb::kvdb_key::Key, prelude::*};
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

    dbg!(db.list_contains(format!("{}", "things"), true));
    dbg!(db.list_remove(("things", true)));
    dbg!(db.list_contains("things", true));

    dbg!(db.pop_back("things"));
    dbg!(db.pop_front("things"));
    dbg!(db.list_range(("things", 0, 5)));
    dbg!(db.list_range(("things", -2)));
    dbg!(db.list_len("things"));

    // TODO: This should also work.
    //dbg!(db.list_range("things"));

    // dbg!(&db);

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

    dbg!(db.hash_get_object::<Person>("user:2"));

    let _ = db.hash_set(
        Key::namespaced("cred", 3.141529),
        Credentials {
            email: "user@addr.com".to_owned(),
            password: "passw0rd".to_owned(),
        },
    );
    dbg!(db.hash_get_object::<Credentials>("cred:3.141529"));

    let _x = Utc::now();

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

#[derive(Debug, Default, Clone, KVDBEntity)]
pub struct Credentials {
    email: String,
    password: String,
}
