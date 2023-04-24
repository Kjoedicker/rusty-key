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

    pub fn set (&mut self, key: String, value: String, tracking: Option<bool>) -> Option<String> {
        let result = self.store.insert(
            key.to_owned(), 
            value.to_owned()
        );

        match tracking {
            Some(true) => {
                self.aof.append_file(
                    format!("^set|{}|{}", key, value).as_bytes()
                );
            }
            _ => {}
        }

        result
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}
