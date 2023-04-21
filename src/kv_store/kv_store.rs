use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::aof::*;

lazy_static!{
    pub static ref KEY_VALUE_STORE: Mutex<KeyValueStore> = Mutex::new(
        KeyValueStore{
            store: HashMap::new(),
            // TODO: add support for pulling this from a config
            aof: init_file_handler("./persistence.aof")
        }
    );
}

#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, String>,
    aof: FileHandler,
}

impl KeyValueStore {

    pub fn set (&mut self, key: String, value: String) -> Option<String> {
        // TODO: take into consideration when a KEY will be overwritten - do we want non-mutable key as a point?
        match self.store.insert(key.to_owned(), value.to_owned()) {
            Some(value) => {
                self.aof.append_file(
                    format!("^set:{}:{}", key, value).as_bytes()
                );

                Some(value)
            },
            None => None
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}
