use std::fmt::Display;

use super::kvdb_key::Key;

#[derive(Debug, Default, Clone)]
pub struct NamespacedKey(Vec<String>);

impl NamespacedKey {
    pub fn new(init: impl Display) -> Self {
        Self(vec![init.to_string()])
    }

    pub fn ns(mut self, value: impl Display) -> Self {
        self.0.push(value.to_string());
        self
    }

    pub fn compose(self) -> Key {
        Key(self.0.join(":"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn composes_correctly() {
        assert_eq!(
            Key("name:space:1".to_owned()),
            NamespacedKey::new("name").ns("space").ns("1").compose()
        )
    }
}
