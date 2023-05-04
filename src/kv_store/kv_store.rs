use crate::aof::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
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
    pub fn set(&mut self, key: String, value: String, tracking: Option<bool>) -> Option<String> {
        let result = self.store.insert(key.to_owned(), value.to_owned());

        if let Some(true) = tracking {
            self.aof
                .append_file(format!("^set|{}|{}", key, value).as_bytes());
        }

        result
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str, tracking: Option<bool>) -> Option<String> {
        match self.store.remove(key) {
            Some(_) => {
                if let Some(true) = tracking {
                    self.aof.append_file(format!("^delete|{}", key).as_bytes());
                }

                Some(key.to_string())
            }
            None => None,
        }
    }

    pub fn process_actions(&mut self) {
        let actions = self.aof.parse_actions();

        for action in actions.iter() {
            let command = action[0].as_str();

            match command {
                "set" => {
                    let (key, value) = (&action[1], &action[2]);
                    println!("Setting key {}", key);
                    self.set(key.to_string(), value.to_owned(), Some(false));
                }
                "delete" => {
                    let key = &action[1];
                    println!("Deleting key {}", key);
                    self.delete(key, Some(false));
                }
                _ => println!("How did we get here?"),
            }
        }
    }
}
