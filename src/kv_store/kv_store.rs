use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    pub static ref KEY_VALUE_STORE: Mutex<KeyValueStore> = Mutex::new(KeyValueStore{store: HashMap::new()});
}

#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, String>
}

impl KeyValueStore {

    pub fn set (&mut self, key: String, value: String) -> Option<String> {
        // TODO: take into consideration when a KEY will be overwritten - do we want non-mutable key as a point?
        self.store.insert(key.to_owned(), value.to_owned())
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}
